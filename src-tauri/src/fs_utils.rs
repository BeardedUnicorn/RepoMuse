use std::fs;
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime};
use ignore::WalkBuilder;
use ignore::overrides::{Override, OverrideBuilder};
use std::fs::File;
use std::io::{BufReader, Read};
use once_cell::sync::Lazy;
use std::sync::Mutex;
use std::collections::HashMap;
use rayon::prelude::*;

// Cache for walker builders to avoid recreating them
static WALKER_CACHE: Lazy<Mutex<HashMap<PathBuf, Override>>> = 
    Lazy::new(|| Mutex::new(HashMap::with_capacity(10)));

// Determine language from file extension
pub fn get_language_from_extension(path: &str) -> String {
    match Path::new(path).extension().and_then(|ext| ext.to_str()) {
        Some("rs") => "Rust".to_string(),
        Some("js") | Some("jsx") => "JavaScript".to_string(),
        Some("ts") | Some("tsx") => "TypeScript".to_string(),
        Some("py") => "Python".to_string(),
        Some("java") => "Java".to_string(),
        Some("cpp") | Some("cc") | Some("cxx") => "C++".to_string(),
        Some("c") => "C".to_string(),
        Some("go") => "Go".to_string(),
        Some("php") => "PHP".to_string(),
        Some("rb") => "Ruby".to_string(),
        Some("cs") => "C#".to_string(),
        Some("swift") => "Swift".to_string(),
        Some("kt") => "Kotlin".to_string(),
        Some("html") => "HTML".to_string(),
        Some("css") => "CSS".to_string(),
        Some("scss") | Some("sass") => "SCSS".to_string(),
        Some("json") => "JSON".to_string(),
        Some("xml") => "XML".to_string(),
        Some("yml") | Some("yaml") => "YAML".to_string(),
        Some("toml") => "TOML".to_string(),
        Some("md") => "Markdown".to_string(),
        _ => "Unknown".to_string(),
    }
}

// Filter files we should analyze
pub fn should_analyze_file(path: &str) -> bool {
    let ignore_extensions = vec![
        "png", "jpg", "jpeg", "gif", "svg", "ico", "woff", "woff2", "ttf", "eot", "pdf", "zip", "tar", "gz",
    ];
    let ignore_dirs = vec![
        "node_modules", "target", "build", "dist", ".git", ".svn", "vendor", "__pycache__",
    ];

    for ignore_dir in ignore_dirs {
        if path.contains(&format!("/{}/", ignore_dir)) || path.contains(&format!("\\{}\\", ignore_dir)) {
            return false;
        }
    }

    if let Some(ext) = Path::new(path).extension().and_then(|ext| ext.to_str()) {
        return !ignore_extensions.contains(&ext);
    }

    true
}

// Get or create cached overrides for a path
fn get_cached_overrides(root: &Path) -> Option<Override> {
    let root_buf = root.to_path_buf();
    
    // Try to get from cache first
    if let Ok(cache) = WALKER_CACHE.lock() {
        if let Some(overrides) = cache.get(&root_buf) {
            return Some(overrides.clone());
        }
    }
    
    // Create new overrides
    let overrides = default_overrides(root)?;
    
    // Store in cache
    if let Ok(mut cache) = WALKER_CACHE.lock() {
        // Limit cache size to prevent unbounded growth
        if cache.len() > 100 {
            cache.clear();
        }
        cache.insert(root_buf, overrides.clone());
    }
    
    Some(overrides)
}

// Build a gitignore-aware walker with sensible defaults
pub fn walker(path: &Path) -> ignore::Walk {
    let mut builder = WalkBuilder::new(path);
    builder
        .follow_links(false)
        .git_ignore(true)
        .git_global(true)
        .git_exclude(true)
        .ignore(true)
        .hidden(true)
        .parents(true);
    if let Some(overrides) = get_cached_overrides(path) {
        builder.overrides(overrides);
    }
    builder.build()
}

pub fn walker_with_depth(path: &Path, max_depth: Option<usize>) -> ignore::Walk {
    let mut builder = WalkBuilder::new(path);
    builder
        .follow_links(false)
        .git_ignore(true)
        .git_global(true)
        .git_exclude(true)
        .ignore(true)
        .hidden(true)
        .parents(true)
        .max_depth(max_depth);
    if let Some(overrides) = get_cached_overrides(path) {
        builder.overrides(overrides);
    }
    builder.build()
}

fn default_overrides(root: &Path) -> Option<Override> {
    let mut ob = OverrideBuilder::new(root);
    // Common heavy directories (excluded regardless of .gitignore)
    let dirs = [
        "**/node_modules/**", "**/.git/**", "**/dist/**", "**/build/**", "**/target/**", "**/vendor/**",
        "**/__pycache__/**", "**/.next/**", "**/.svelte-kit/**", "**/.venv/**", "**/venv/**",
        "**/.pnpm-store/**", "**/.yardoc/**", "**/.bundle/**", "**/.terraform/**", "**/.m2/**",
        "**/.cache/**", "**/coverage/**", "**/Pods/**", "**/DerivedData/**", "**/tmp/**",
    ];
    for d in dirs {
        let _ = ob.add(&format!("!{}", d));
    }

    // Binary and non-code file types to skip early
    let exts = [
        "png", "jpg", "jpeg", "gif", "svg", "ico", "webp", "bmp", "tiff",
        "woff", "woff2", "ttf", "eot",
        "pdf", "zip", "tar", "gz", "bz2", "xz", "7z",
        "mp3", "mp4", "mkv", "mov", "avi", "wav", "flac",
        "wasm",
    ];
    for ext in exts {
        let _ = ob.add(&format!("!**/*.{}", ext));
    }

    match ob.build() {
        Ok(overrides) => Some(overrides),
        Err(_) => None,
    }
}

// Optimized: Read only up to cap_bytes from a file and return whether it was truncated
pub fn read_text_prefix_limited(path: &str, cap_bytes: usize) -> Result<(String, bool), std::io::Error> {
    let file = File::open(path)?;
    let mut reader = BufReader::with_capacity(8192, file);
    let mut buffer = Vec::with_capacity(cap_bytes.min(8192));
    let mut total = 0usize;
    let mut chunk = [0u8; 4096]; // Smaller chunks for better control
    let mut was_truncated = false;
    
    while total < cap_bytes {
        let to_read = (cap_bytes - total).min(chunk.len());
        match reader.read(&mut chunk[..to_read]) {
            Ok(0) => break, // EOF
            Ok(n) => {
                buffer.extend_from_slice(&chunk[..n]);
                total += n;
                if total >= cap_bytes {
                    was_truncated = true;
                    break;
                }
            }
            Err(e) if e.kind() == std::io::ErrorKind::Interrupted => continue,
            Err(e) => return Err(e),
        }
    }
    
    // Check if there's more data available
    if !was_truncated {
        let mut test_byte = [0u8; 1];
        if reader.read(&mut test_byte)? > 0 {
            was_truncated = true;
        }
    }
    
    Ok((String::from_utf8_lossy(&buffer).into_owned(), was_truncated))
}

// Legacy function for compatibility - redirects to optimized version
#[allow(dead_code)]
pub fn read_text_prefix(path: &str, cap_bytes: usize) -> Option<String> {
    read_text_prefix_limited(path, cap_bytes)
        .ok()
        .map(|(content, _)| content)
}

// Fast non-crypto short hash of first N bytes (FNV-1a variant)
pub fn short_hash_prefix(path: &str, cap_bytes: usize) -> Option<u64> {
    let file = File::open(path).ok()?;
    let mut reader = BufReader::with_capacity(8192, file);
    let mut total = 0usize;
    let mut chunk = [0u8; 4096];
    let mut hash: u64 = 0xcbf29ce484222325; // FNV offset basis
    const PRIME: u64 = 0x100000001b3;
    
    while total < cap_bytes {
        let to_read = (cap_bytes - total).min(chunk.len());
        let n = reader.read(&mut chunk[..to_read]).ok()?;
        if n == 0 { break; }
        for &b in &chunk[..n] {
            hash ^= b as u64;
            hash = hash.wrapping_mul(PRIME);
        }
        total += n;
    }
    
    Some(hash)
}

// Parallel walker builders
pub fn walker_parallel(path: &Path) -> ignore::WalkParallel {
    let mut builder = WalkBuilder::new(path);
    let num_threads = std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(4)
        .min(8);
    
    builder
        .follow_links(false)
        .git_ignore(true)
        .git_global(true)
        .git_exclude(true)
        .ignore(true)
        .hidden(true)
        .parents(true)
        .threads(num_threads);
        
    if let Some(overrides) = get_cached_overrides(path) {
        builder.overrides(overrides);
    }
    builder.build_parallel()
}

// Directory last modified seconds since epoch
pub fn get_dir_modified_time(path: &Path) -> u64 {
    if let Ok(metadata) = fs::metadata(path) {
        if let Ok(modified) = metadata.modified() {
            return modified
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap_or(Duration::from_secs(0))
                .as_secs();
        }
    }
    0
}

// Optimized batch file reading for multiple files
#[allow(dead_code)]
pub fn read_files_batch(paths: &[String], cap_bytes: usize) -> Vec<Option<(String, bool)>> {
    paths.par_iter()
        .map(|path| read_text_prefix_limited(path, cap_bytes).ok())
        .collect()
}

// Check if a file is likely binary by sampling first bytes
#[allow(dead_code)]
pub fn is_likely_binary(path: &str) -> bool {
    let file = match File::open(path) {
        Ok(f) => f,
        Err(_) => return false,
    };
    
    let mut reader = BufReader::new(file);
    let mut buffer = [0u8; 512];
    
    let bytes_read = match reader.read(&mut buffer) {
        Ok(n) => n,
        Err(_) => return false,
    };
    
    // Check for null bytes (common in binary files)
    for &byte in &buffer[..bytes_read] {
        if byte == 0 {
            return true;
        }
    }
    
    // Check for high ratio of non-printable characters
    let non_printable = buffer[..bytes_read]
        .iter()
        .filter(|&&b| b < 0x20 && b != 0x09 && b != 0x0A && b != 0x0D)
        .count();
    
    non_printable as f32 / bytes_read as f32 > 0.3
}
