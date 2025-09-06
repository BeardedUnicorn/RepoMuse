use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FileCountCache {
    pub path: String,
    pub count: usize,
    pub last_modified: u64,
    pub cached_at: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AnalysisCacheEntry {
    pub path: String,
    pub last_modified: u64,
    pub cached_at: u64,
    pub analysis: crate::analysis::RepoAnalysis,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProjectMetaCacheEntry {
    pub path: String,
    pub description: Option<String>,
    pub is_git_repo: bool,
    pub last_modified: u64,
    pub cached_at: u64,
}

fn app_data_dir() -> Option<std::path::PathBuf> {
    dirs::data_local_dir().map(|d| d.join("repomuse"))
}

pub fn load_file_count_cache() -> HashMap<String, FileCountCache> {
    let cache_path = match app_data_dir() { Some(d) => d.join("file_count_cache.json"), None => return HashMap::new() };
    if let Ok(s) = fs::read_to_string(cache_path) {
        if let Ok(map) = serde_json::from_str(&s) { return map; }
    }
    HashMap::new()
}

pub fn save_file_count_cache(cache: &HashMap<String, FileCountCache>) {
    if let Some(dir) = app_data_dir() {
        let _ = fs::create_dir_all(&dir);
        let path = dir.join("file_count_cache.json");
        if let Ok(json) = serde_json::to_string_pretty(cache) {
            let _ = fs::write(path, json);
        }
    }
}

pub fn clear_file_count_cache_file() -> Result<(), String> {
    let dir = app_data_dir().ok_or("Failed to get app data directory")?;
    let cache_path = dir.join("file_count_cache.json");
    if cache_path.exists() { fs::remove_file(cache_path).map_err(|e| e.to_string())?; }
    Ok(())
}

pub fn load_analysis_cache() -> HashMap<String, AnalysisCacheEntry> {
    let cache_path = match app_data_dir() { Some(d) => d.join("analysis_cache.json"), None => return HashMap::new() };
    if let Ok(s) = fs::read_to_string(cache_path) {
        if let Ok(map) = serde_json::from_str(&s) { return map; }
    }
    HashMap::new()
}

pub fn save_analysis_cache(cache: &HashMap<String, AnalysisCacheEntry>) {
    if let Some(dir) = app_data_dir() {
        let _ = fs::create_dir_all(&dir);
        let path = dir.join("analysis_cache.json");
        if let Ok(json) = serde_json::to_string_pretty(cache) {
            let _ = fs::write(path, json);
        }
    }
}

pub fn load_project_meta_cache() -> HashMap<String, ProjectMetaCacheEntry> {
    let cache_path = match app_data_dir() { Some(d) => d.join("project_meta_cache.json"), None => return HashMap::new() };
    if let Ok(s) = fs::read_to_string(cache_path) {
        if let Ok(map) = serde_json::from_str(&s) { return map; }
    }
    HashMap::new()
}

pub fn save_project_meta_cache(cache: &HashMap<String, ProjectMetaCacheEntry>) {
    if let Some(dir) = app_data_dir() {
        let _ = fs::create_dir_all(&dir);
        let path = dir.join("project_meta_cache.json");
        if let Ok(json) = serde_json::to_string_pretty(cache) {
            let _ = fs::write(path, json);
        }
    }
}

