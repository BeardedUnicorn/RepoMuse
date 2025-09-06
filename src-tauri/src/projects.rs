use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::time::{Duration, SystemTime};
use walkdir::WalkDir;

use crate::cache::{
    clear_file_count_cache_file, load_file_count_cache, load_project_meta_cache, save_file_count_cache,
    save_project_meta_cache, FileCountCache, ProjectMetaCacheEntry,
};
use crate::fs_utils::{get_dir_modified_time, should_analyze_file};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProjectDirectory {
    pub name: String,
    pub path: String,
    pub is_git_repo: bool,
    pub file_count: usize,
    pub description: Option<String>,
    pub is_counting: bool,
}

fn is_project_directory(path: &Path) -> bool {
    let project_indicators = vec![
        "package.json", "Cargo.toml", "pom.xml", "build.gradle", "requirements.txt", "Gemfile", "go.mod",
        "composer.json", "project.clj", "mix.exs", ".csproj", "pubspec.yaml", "CMakeLists.txt", "Makefile",
        "README.md", "README.txt",
    ];

    for indicator in project_indicators {
        if indicator.ends_with(".csproj") {
            if let Ok(entries) = fs::read_dir(path) {
                for entry in entries.flatten() {
                    if let Some(name) = entry.file_name().to_str() {
                        if name.ends_with(".csproj") {
                            return true;
                        }
                    }
                }
            }
        } else if path.join(indicator).exists() {
            return true;
        }
    }
    false
}

fn get_project_description(path: &Path) -> Option<String> {
    if let Ok(package_json) = fs::read_to_string(path.join("package.json")) {
        if let Ok(json) = serde_json::from_str::<serde_json::Value>(&package_json) {
            if let Some(description) = json["description"].as_str() {
                return Some(description.to_string());
            }
        }
    }
    if let Ok(cargo_toml) = fs::read_to_string(path.join("Cargo.toml")) {
        if let Some(desc_line) = cargo_toml.lines().find(|line| line.starts_with("description")) {
            if let Some(desc) = desc_line.split('=').nth(1) {
                return Some(desc.trim().trim_matches('"').to_string());
            }
        }
    }
    for readme_name in &["README.md", "README.txt", "readme.md", "readme.txt"] {
        if let Ok(readme) = fs::read_to_string(path.join(readme_name)) {
            let first_line = readme.lines().next().unwrap_or("").trim();
            if !first_line.is_empty() && first_line.len() < 200 {
                let cleaned = first_line.trim_start_matches('#').trim();
                if !cleaned.is_empty() {
                    return Some(cleaned.to_string());
                }
            }
        }
    }
    None
}

fn estimate_deep_files(path: &Path) -> usize {
    let mut count = 0;
    let mut checked = 0;
    const MAX_CHECK: usize = 50;
    for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()).take(MAX_CHECK) {
        checked += 1;
        if entry.path().is_file() {
            let path_str = entry.path().to_string_lossy();
            if should_analyze_file(&path_str) {
                count += 1;
            }
        }
    }
    if checked >= MAX_CHECK { count * 2 } else { count }
}

fn estimate_file_count(path: &Path) -> usize {
    let mut count = 0;
    let mut depth = 0;
    const MAX_DEPTH: usize = 3;
    const SAMPLE_FACTOR: usize = 10;
    for entry in WalkDir::new(path).max_depth(MAX_DEPTH).into_iter().filter_map(|e| e.ok()) {
        if entry.path().is_file() {
            let path_str = entry.path().to_string_lossy();
            if should_analyze_file(&path_str) { count += 1; }
        }
        if entry.path().is_dir() && entry.depth() == MAX_DEPTH {
            depth += 1;
            if depth % SAMPLE_FACTOR == 0 {
                count += estimate_deep_files(entry.path()) * SAMPLE_FACTOR;
            }
        }
    }
    count
}

fn count_project_files(path: &Path) -> usize {
    WalkDir::new(path)
        .into_iter()
        .par_bridge()
        .filter_map(|e| e.ok())
        .filter(|entry| entry.path().is_file() && should_analyze_file(&entry.path().to_string_lossy()))
        .count()
}

fn process_project_directory_fast(
    path: std::path::PathBuf,
    count_cache: &HashMap<String, FileCountCache>,
    meta_cache: &HashMap<String, ProjectMetaCacheEntry>,
) -> Option<(ProjectDirectory, Option<ProjectMetaCacheEntry>)> {
    let dir_name = path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("Unknown")
        .to_string();

    if dir_name.starts_with('.')
        || ["node_modules", "target", "build", "dist", "vendor", "__pycache__"].contains(&dir_name.as_str())
    {
        return None;
    }

    if is_project_directory(&path) {
        let path_str = path.to_string_lossy().to_string();
        let dir_modified = get_dir_modified_time(&path);
        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap_or(Duration::from_secs(0))
            .as_secs();
        const META_TTL: u64 = 3600;

        let mut is_git_repo = path.join(".git").exists();
        let mut description: Option<String> = None;
        let mut meta_update: Option<ProjectMetaCacheEntry> = None;

        if let Some(meta) = meta_cache.get(&path_str) {
            if meta.last_modified >= dir_modified && (now - meta.cached_at) < META_TTL {
                is_git_repo = meta.is_git_repo;
                description = meta.description.clone();
            }
        }
        if description.is_none() {
            let desc = get_project_description(&path);
            description = desc.clone();
            meta_update = Some(ProjectMetaCacheEntry {
                path: path_str.clone(),
                description: desc,
                is_git_repo,
                last_modified: dir_modified,
                cached_at: now,
            });
        }

        let (file_count, is_counting) = if let Some(cached) = count_cache.get(&path_str) {
            if cached.last_modified >= dir_modified && (now - cached.cached_at) < 86400 {
                (cached.count, false)
            } else {
                (estimate_file_count(&path), true)
            }
        } else {
            (estimate_file_count(&path), true)
        };

        let project = ProjectDirectory {
            name: dir_name,
            path: path_str,
            is_git_repo,
            file_count,
            description,
            is_counting,
        };

        Some((project, meta_update))
    } else {
        None
    }
}

#[tauri::command]
pub async fn list_project_directories(root_path: String) -> Result<Vec<ProjectDirectory>, String> {
    let root = Path::new(&root_path);
    if !root.exists() || !root.is_dir() {
        return Err("Invalid root directory".to_string());
    }

    let count_cache = load_file_count_cache();
    let mut meta_cache = load_project_meta_cache();

    let entries: Vec<std::path::PathBuf> = fs::read_dir(root)
        .map_err(|e| format!("Failed to read directory: {}", e))?
        .filter_map(|e| e.ok())
        .filter(|entry| entry.path().is_dir())
        .map(|entry| entry.path())
        .collect();

    let results: Vec<(ProjectDirectory, Option<ProjectMetaCacheEntry>)> = entries
        .par_iter()
        .filter_map(|p| process_project_directory_fast(p.clone(), &count_cache, &meta_cache))
        .collect();

    let mut projects: Vec<ProjectDirectory> = Vec::with_capacity(results.len());
    for (proj, update) in results {
        if let Some(entry) = update { meta_cache.insert(proj.path.clone(), entry); }
        projects.push(proj);
    }
    save_project_meta_cache(&meta_cache);

    projects.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
    Ok(projects)
}

#[tauri::command]
pub async fn update_project_file_count(project_path: String) -> Result<usize, String> {
    let path = Path::new(&project_path);
    if !path.exists() || !path.is_dir() { return Err("Invalid project path".to_string()); }
    let count = count_project_files(path);
    let last_modified = get_dir_modified_time(path);
    let cached_at = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap_or(Duration::from_secs(0))
        .as_secs();
    let mut cache = load_file_count_cache();
    cache.insert(project_path.clone(), FileCountCache { path: project_path, count, last_modified, cached_at });
    save_file_count_cache(&cache);
    Ok(count)
}

#[tauri::command]
pub async fn clear_file_count_cache() -> Result<(), String> {
    clear_file_count_cache_file()
}

