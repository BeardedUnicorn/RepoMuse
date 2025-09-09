use std::fs;
use std::path::Path;
use std::time::{Duration, SystemTime};
use ignore::WalkBuilder;
use ignore::overrides::{Override, OverrideBuilder};
use std::fs::File;
use std::io::{BufReader, Read};

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
    if let Some(overrides) = default_overrides(path) {
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
    if let Some(overrides) = default_overrides(path) {
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

// Read up to cap_bytes from a file as lossy UTF-8 text
pub fn read_text_prefix(path: &str, cap_bytes: usize) -> Option<String> {
    let file = File::open(path).ok()?;
    let mut reader = BufReader::with_capacity(64 * 1024, file);
    let mut buf = Vec::with_capacity(cap_bytes.min(64 * 1024));
    let mut total = 0usize;
    let mut chunk = [0u8; 8192];
    while total < cap_bytes {
        let to_read = (cap_bytes - total).min(chunk.len());
        match reader.read(&mut chunk[..to_read]) {
            Ok(0) => break,
            Ok(n) => {
                buf.extend_from_slice(&chunk[..n]);
                total += n;
            }
            Err(_) => break,
        }
    }
    Some(String::from_utf8_lossy(&buf).into_owned())
}

// Fast non-crypto short hash of first N bytes (FNV-1a variant)
pub fn short_hash_prefix(path: &str, cap_bytes: usize) -> Option<u64> {
    let file = File::open(path).ok()?;
    let mut reader = BufReader::with_capacity(64 * 1024, file);
    let mut total = 0usize;
    let mut chunk = [0u8; 8192];
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
    builder
        .follow_links(false)
        .git_ignore(true)
        .git_global(true)
        .git_exclude(true)
        .ignore(true)
        .hidden(true)
        .parents(true);
    if let Some(overrides) = default_overrides(path) {
        builder.overrides(overrides);
    }
    builder.build_parallel()
}

// (removed unused: is_ignored_dir_name, walker_parallel_with_depth)

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
