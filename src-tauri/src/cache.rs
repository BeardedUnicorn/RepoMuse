use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;
use std::time::SystemTime;
// use ignore::WalkBuilder; // switched to helper walkers in fs_utils
use bincode;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FileCountCache {
    pub path: String,
    pub count: usize,
    pub last_modified: u64,
    pub cached_at: u64,
    pub file_inventory: Option<HashMap<String, u64>>, // Track individual files and their mod times
}

impl FileCountCache {
    pub fn new(path: String) -> Self {
        FileCountCache {
            path,
            count: 0,
            last_modified: 0,
            cached_at: 0,
            file_inventory: Some(HashMap::new()),
        }
    }

    /// Incrementally update file count by scanning for changes
    pub fn incremental_update(&mut self, root_path: &Path) -> Result<bool, String> {
        let mut inventory = self.file_inventory.clone().unwrap_or_default();
        let mut current_files = HashSet::new();
        let mut changed = false;
        let mut new_count = 0;

        // Walk the directory and track current files (gitignore-aware)
        for result in crate::fs_utils::walker(root_path) {
            let entry = match result { Ok(e) => e, Err(_) => continue };
            if !entry.file_type().map_or(false, |ft| ft.is_file()) { continue; }
            let path_str = entry.path().to_string_lossy().to_string();
            current_files.insert(path_str.clone());

            // Check if file should be counted
            if should_analyze_file(&path_str) {
                new_count += 1;

                // Check if file is new or modified
                if let Ok(metadata) = entry.metadata() {
                    if let Ok(modified) = metadata.modified() {
                        let mod_time = modified
                            .duration_since(SystemTime::UNIX_EPOCH)
                            .unwrap_or_default()
                            .as_secs();

                        match inventory.get(&path_str) {
                            Some(&cached_time) if cached_time == mod_time => {
                                // File unchanged
                            }
                            _ => {
                                // New or modified file
                                inventory.insert(path_str, mod_time);
                                changed = true;
                            }
                        }
                    }
                }
            }
        }

        // Remove deleted files from inventory
        let deleted: Vec<String> = inventory
            .keys()
            .filter(|k| !current_files.contains(*k))
            .cloned()
            .collect();

        if !deleted.is_empty() {
            for file in deleted {
                inventory.remove(&file);
            }
            changed = true;
        }

        // Update cache if changes detected
        if changed || self.count != new_count {
            self.count = new_count;
            self.file_inventory = Some(inventory);
            self.cached_at = SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs();
            self.last_modified = get_dir_modified_time(root_path);
        }

        Ok(changed)
    }

    /// Fast validation - check if cache is likely still valid
    pub fn is_likely_valid(&self, root_path: &Path) -> bool {
        // Quick check: has directory modification time changed?
        let current_mod_time = get_dir_modified_time(root_path);
        if current_mod_time > self.last_modified {
            return false;
        }

        // Check cache age (consider invalid after 24 hours)
        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        
        const MAX_CACHE_AGE: u64 = 24 * 60 * 60; // 24 hours
        if now - self.cached_at > MAX_CACHE_AGE {
            return false;
        }

        true
    }

    /// Get count of files modified since last cache update
    pub fn get_modified_count(&self, _root_path: &Path) -> usize {
        let inventory = match &self.file_inventory {
            Some(inv) => inv,
            None => return usize::MAX, // Full rescan needed
        };

        let mut modified_count = 0;

        for (cached_path, cached_time) in inventory {
            if let Ok(metadata) = fs::metadata(cached_path) {
                if let Ok(modified) = metadata.modified() {
                    let mod_time = modified
                        .duration_since(SystemTime::UNIX_EPOCH)
                        .unwrap_or_default()
                        .as_secs();
                    
                    if mod_time != *cached_time {
                        modified_count += 1;
                    }
                }
            }
        }

        modified_count
    }
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

// New file-level cache structures
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FileMetadata {
    pub path: String,
    pub language: String,
    pub size: u64,
    pub last_modified: u64,
    pub cached_at: u64,
    #[serde(default)]
    pub short_hash: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FileMetadataCache {
    pub entries: HashMap<String, FileMetadata>,
    pub cache_version: u32,
}

impl FileMetadataCache {
    pub fn new() -> Self {
        FileMetadataCache {
            entries: HashMap::new(),
            cache_version: 1,
        }
    }

    /// Incremental scan - only process new or modified files
    #[allow(dead_code)]
    pub fn incremental_scan(&mut self, root_path: &Path) -> Result<Vec<String>, String> {
        let mut new_or_modified = Vec::new();

        for result in crate::fs_utils::walker(root_path) {
            let entry = match result { Ok(e) => e, Err(_) => continue };
            if !entry.file_type().map_or(false, |ft| ft.is_file()) { continue; }
            let path_str = entry.path().to_string_lossy().to_string();
            
            if !should_analyze_file(&path_str) {
                continue;
            }

            if let Ok(metadata) = entry.metadata() {
                if let Ok(modified) = metadata.modified() {
                    let mod_time = modified
                        .duration_since(SystemTime::UNIX_EPOCH)
                        .unwrap_or_default()
                        .as_secs();

                    let needs_update = match self.entries.get(&path_str) {
                        Some(cached) if cached.last_modified == mod_time => false,
                        _ => true,
                    };

                    if needs_update {
                        new_or_modified.push(path_str);
                    }
                }
            }
        }

        Ok(new_or_modified)
    }

    /// Get cached file metadata if it exists and is still valid
    pub fn get_valid_metadata(&self, file_path: &str) -> Option<&FileMetadata> {
        if let Some(metadata) = self.entries.get(file_path) {
            // Check if file still exists and modification time matches
            if let Ok(file_meta) = fs::metadata(file_path) {
                if let Ok(modified) = file_meta.modified() {
                    let mod_time = modified
                        .duration_since(SystemTime::UNIX_EPOCH)
                        .unwrap_or_default()
                        .as_secs();
                    
                    if mod_time == metadata.last_modified {
                        return Some(metadata);
                    }
                }
            }
        }
        None
    }

    /// Insert or update file metadata in cache
    pub fn insert_metadata(&mut self, file_path: String, language: String, size: u64) -> Result<(), String> {
        let mod_time = if let Ok(file_meta) = fs::metadata(&file_path) {
            if let Ok(modified) = file_meta.modified() {
                modified
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs()
            } else {
                0
            }
        } else {
            return Err(format!("Cannot access file: {}", file_path));
        };

        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        let metadata = FileMetadata {
            path: file_path.clone(),
            language,
            size,
            last_modified: mod_time,
            cached_at: now,
            short_hash: None,
        };

        self.entries.insert(file_path, metadata);
        Ok(())
    }

    pub fn insert_metadata_with_hash(&mut self, file_path: String, language: String, size: u64, short_hash: Option<u64>) -> Result<(), String> {
        let mod_time = if let Ok(file_meta) = fs::metadata(&file_path) {
            if let Ok(modified) = file_meta.modified() {
                modified
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs()
            } else {
                0
            }
        } else {
            return Err(format!("Cannot access file: {}", file_path));
        };

        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        let metadata = FileMetadata {
            path: file_path.clone(),
            language,
            size,
            last_modified: mod_time,
            cached_at: now,
            short_hash,
        };

        self.entries.insert(file_path, metadata);
        Ok(())
    }

    /// Remove entries older than TTL seconds
    pub fn prune_old_entries(&mut self, ttl_seconds: u64) {
        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        self.entries.retain(|_, metadata| {
            now - metadata.cached_at < ttl_seconds
        });
    }

    /// Validate and clean invalid entries (files that no longer exist or have been modified)
    pub fn validate_and_clean(&mut self) {
        self.entries.retain(|path, metadata| {
            if let Ok(file_meta) = fs::metadata(path) {
                if let Ok(modified) = file_meta.modified() {
                    let mod_time = modified
                        .duration_since(SystemTime::UNIX_EPOCH)
                        .unwrap_or_default()
                        .as_secs();
                    
                    return mod_time == metadata.last_modified;
                }
            }
            false
        });
    }

    /// Get cache statistics
    pub fn get_stats(&self) -> (usize, usize, u64) {
        let total_entries = self.entries.len();
        let mut valid_entries = 0;
        let mut total_size = 0u64;

        for (path, metadata) in &self.entries {
            if Path::new(path).exists() {
                if let Ok(file_meta) = fs::metadata(path) {
                    if let Ok(modified) = file_meta.modified() {
                        let mod_time = modified
                            .duration_since(SystemTime::UNIX_EPOCH)
                            .unwrap_or_default()
                            .as_secs();
                        
                        if mod_time == metadata.last_modified {
                            valid_entries += 1;
                            total_size += metadata.size;
                        }
                    }
                }
            }
        }

        (total_entries, valid_entries, total_size)
    }
}

// Optimized global cache management
#[derive(Debug, Serialize, Deserialize)]
pub struct GlobalFileCountCache {
    pub projects: HashMap<String, FileCountCache>,
    pub last_cleanup: u64,
}

impl GlobalFileCountCache {
    pub fn new() -> Self {
        GlobalFileCountCache {
            projects: HashMap::new(),
            last_cleanup: SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
        }
    }

    /// Incremental update for a specific project
    pub fn update_project(&mut self, project_path: &str) -> Result<(usize, bool), String> {
        let path = Path::new(project_path);
        
        let mut cache = self.projects
            .remove(project_path)
            .unwrap_or_else(|| FileCountCache::new(project_path.to_string()));

        // Check if incremental update is worthwhile
        if cache.is_likely_valid(path) && cache.file_inventory.is_some() {
            // Perform incremental update
            let changed = cache.incremental_update(path)?;
            let count = cache.count;
            self.projects.insert(project_path.to_string(), cache);
            Ok((count, changed))
        } else {
            // Full rescan needed
            cache.incremental_update(path)?;
            let count = cache.count;
            self.projects.insert(project_path.to_string(), cache);
            Ok((count, true))
        }
    }

    /// Batch update multiple projects efficiently
    pub fn batch_update(&mut self, project_paths: Vec<String>) -> HashMap<String, (usize, bool)> {
        let mut results = HashMap::new();
        
        for path in project_paths {
            if let Ok(result) = self.update_project(&path) {
                results.insert(path, result);
            }
        }
        
        results
    }

    /// Cleanup old entries periodically
    pub fn cleanup_if_needed(&mut self) {
        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        const CLEANUP_INTERVAL: u64 = 60 * 60; // 1 hour
        
        if now - self.last_cleanup > CLEANUP_INTERVAL {
            self.projects.retain(|path, _| Path::new(path).exists());
            self.last_cleanup = now;
        }
    }
}

fn app_data_dir() -> Option<std::path::PathBuf> {
    dirs::data_local_dir().map(|d| d.join("repomuse"))
}

// Helper function to check if file should be analyzed
fn should_analyze_file(path: &str) -> bool {
    let ignore_extensions = vec![
        "png", "jpg", "jpeg", "gif", "svg", "ico", "woff", "woff2", "ttf", "eot", 
        "pdf", "zip", "tar", "gz", "exe", "dll", "so", "dylib",
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

// Helper function to get directory modification time
fn get_dir_modified_time(path: &Path) -> u64 {
    if let Ok(metadata) = fs::metadata(path) {
        if let Ok(modified) = metadata.modified() {
            return modified
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs();
        }
    }
    0
}

pub fn load_file_count_cache() -> GlobalFileCountCache {
    let cache_path = match app_data_dir() { 
        Some(d) => d.join("file_count_cache_v2.json"), 
        None => return GlobalFileCountCache::new() 
    };
    
    if let Ok(s) = fs::read_to_string(cache_path) {
        if let Ok(mut cache) = serde_json::from_str::<GlobalFileCountCache>(&s) {
            cache.cleanup_if_needed();
            return cache;
        }
    }
    
    GlobalFileCountCache::new()
}

pub fn save_file_count_cache(cache: &GlobalFileCountCache) {
    if let Some(dir) = app_data_dir() {
        let _ = fs::create_dir_all(&dir);
        let path = dir.join("file_count_cache_v2.json");
        if let Ok(json) = serde_json::to_string_pretty(cache) {
            let _ = fs::write(path, json);
        }
    }
}

// Legacy compatibility functions
#[allow(dead_code)]
pub fn load_file_count_cache_legacy() -> HashMap<String, FileCountCache> {
    let cache_path = match app_data_dir() { 
        Some(d) => d.join("file_count_cache.json"), 
        None => return HashMap::new() 
    };
    
    if let Ok(s) = fs::read_to_string(cache_path) {
        if let Ok(map) = serde_json::from_str(&s) { return map; }
    }
    HashMap::new()
}

pub fn clear_file_count_cache_file() -> Result<(), String> {
    let dir = app_data_dir().ok_or("Failed to get app data directory")?;
    
    // Clear both legacy and new cache files
    let legacy_cache_path = dir.join("file_count_cache.json");
    if legacy_cache_path.exists() { 
        fs::remove_file(legacy_cache_path).map_err(|e| e.to_string())?; 
    }
    
    let new_cache_path = dir.join("file_count_cache_v2.json");
    if new_cache_path.exists() { 
        fs::remove_file(new_cache_path).map_err(|e| e.to_string())?; 
    }
    
    Ok(())
}

pub fn load_analysis_cache() -> HashMap<String, AnalysisCacheEntry> {
    let dir = match app_data_dir() { Some(d) => d, None => return HashMap::new() };
    let bin_path = dir.join("analysis_cache.bin");
    if let Ok(bytes) = fs::read(&bin_path) {
        if let Ok(map) = bincode::deserialize::<HashMap<String, AnalysisCacheEntry>>(&bytes) { return map; }
    }
    let json_path = dir.join("analysis_cache.json");
    if let Ok(s) = fs::read_to_string(json_path) {
        if let Ok(map) = serde_json::from_str(&s) { return map; }
    }
    HashMap::new()
}

pub fn save_analysis_cache(cache: &HashMap<String, AnalysisCacheEntry>) {
    if let Some(dir) = app_data_dir() {
        let _ = fs::create_dir_all(&dir);
        // Write binary first
        let bin_path = dir.join("analysis_cache.bin");
        if let Ok(bytes) = bincode::serialize(cache) {
            let _ = fs::write(&bin_path, bytes);
        }
        // Keep JSON as a fallback/for debuggability
        let json_path = dir.join("analysis_cache.json");
        if let Ok(json) = serde_json::to_string_pretty(cache) {
            let _ = fs::write(json_path, json);
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

// File metadata cache functions
pub fn load_file_metadata_cache() -> FileMetadataCache {
    let dir = match app_data_dir() { Some(d) => d, None => return FileMetadataCache::new() };
    let bin_path = dir.join("file_metadata_cache.bin");
    if let Ok(bytes) = fs::read(&bin_path) {
        if let Ok(mut cache) = bincode::deserialize::<FileMetadataCache>(&bytes) {
            cache.validate_and_clean();
            cache.prune_old_entries(7 * 24 * 60 * 60);
            return cache;
        }
    }
    let json_path = dir.join("file_metadata_cache.json");
    if let Ok(s) = fs::read_to_string(json_path) {
        if let Ok(mut cache) = serde_json::from_str::<FileMetadataCache>(&s) {
            cache.validate_and_clean();
            cache.prune_old_entries(7 * 24 * 60 * 60);
            return cache;
        }
    }
    FileMetadataCache::new()
}

pub fn save_file_metadata_cache(cache: &FileMetadataCache) {
    if let Some(dir) = app_data_dir() {
        let _ = fs::create_dir_all(&dir);
        let bin_path = dir.join("file_metadata_cache.bin");
        if let Ok(bytes) = bincode::serialize(cache) {
            let _ = fs::write(&bin_path, bytes);
        }
        let json_path = dir.join("file_metadata_cache.json");
        if let Ok(json) = serde_json::to_string_pretty(cache) {
            let _ = fs::write(json_path, json);
        }
    }
}

pub fn clear_file_metadata_cache() -> Result<(), String> {
    let dir = app_data_dir().ok_or("Failed to get app data directory")?;
    let cache_path = dir.join("file_metadata_cache.json");
    if cache_path.exists() { 
        fs::remove_file(cache_path).map_err(|e| e.to_string())?; 
    }
    let bin_path = dir.join("file_metadata_cache.bin");
    if bin_path.exists() {
        fs::remove_file(bin_path).map_err(|e| e.to_string())?;
    }
    Ok(())
}

// Batch operations for file metadata cache
#[tauri::command]
pub async fn batch_get_file_metadata(file_paths: Vec<String>) -> Result<HashMap<String, FileMetadata>, String> {
    let cache = load_file_metadata_cache();
    let mut result = HashMap::new();
    
    for path in file_paths {
        if let Some(metadata) = cache.get_valid_metadata(&path) {
            result.insert(path, metadata.clone());
        }
    }
    
    Ok(result)
}

#[tauri::command]
pub async fn batch_update_file_metadata(updates: Vec<(String, String, u64)>) -> Result<(), String> {
    let mut cache = load_file_metadata_cache();
    
    for (path, language, size) in updates {
        cache.insert_metadata(path, language, size)?;
    }
    
    save_file_metadata_cache(&cache);
    Ok(())
}

// Tauri commands
#[tauri::command]
pub async fn get_file_metadata_cache_stats() -> Result<(usize, usize, u64), String> {
    let cache = load_file_metadata_cache();
    Ok(cache.get_stats())
}

#[tauri::command]
pub async fn get_file_count_cache_stats() -> Result<HashMap<String, (usize, u64)>, String> {
    let cache = load_file_count_cache();
    let mut stats = HashMap::new();
    
    for (path, project_cache) in &cache.projects {
        let modified_count = project_cache.get_modified_count(Path::new(path));
        stats.insert(path.clone(), (project_cache.count, modified_count as u64));
    }
    
    Ok(stats)
}

#[tauri::command]
pub async fn incremental_update_file_count(project_path: String) -> Result<(usize, bool), String> {
    let mut cache = load_file_count_cache();
    let result = cache.update_project(&project_path)?;
    save_file_count_cache(&cache);
    Ok(result)
}

#[tauri::command]
pub async fn batch_update_file_counts(project_paths: Vec<String>) -> Result<HashMap<String, (usize, bool)>, String> {
    let mut cache = load_file_count_cache();
    let results = cache.batch_update(project_paths);
    save_file_count_cache(&cache);
    Ok(results)
}

#[tauri::command]
pub async fn clear_all_caches() -> Result<(), String> {
    clear_file_count_cache_file()?;
    clear_file_metadata_cache()?;
    
    // Clear analysis cache
    let dir = app_data_dir().ok_or("Failed to get app data directory")?;
    let analysis_cache_path = dir.join("analysis_cache.json");
    if analysis_cache_path.exists() {
        fs::remove_file(analysis_cache_path).map_err(|e| e.to_string())?;
    }
    let analysis_cache_bin = dir.join("analysis_cache.bin");
    if analysis_cache_bin.exists() {
        fs::remove_file(analysis_cache_bin).map_err(|e| e.to_string())?;
    }
    
    // Clear project meta cache
    let project_meta_path = dir.join("project_meta_cache.json");
    if project_meta_path.exists() {
        fs::remove_file(project_meta_path).map_err(|e| e.to_string())?;
    }
    
    Ok(())
}
