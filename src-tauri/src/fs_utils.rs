use std::fs;
use std::path::Path;
use std::time::{Duration, SystemTime};

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

