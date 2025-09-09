use std::collections::{HashMap, HashSet};
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, AtomicU64, AtomicUsize, Ordering};
use std::time::{Duration, Instant};

use chrono::Utc;
use once_cell::sync::Lazy;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use tauri::{Emitter, State};

use crate::db::{self, DbPool};
use crate::fs_utils::{get_language_from_extension, read_text_prefix_limited, should_analyze_file, walker};

// Analysis data structures
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileInfo {
  pub path: String,
  pub content: String,
  pub language: String,
  pub size: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileSizeInfo {
  pub path: String,
  pub size_bytes: u64,
  pub size_kb: u64,
  pub language: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SizeMetrics {
  pub total_size_bytes: u64,
  pub total_size_kb: u64,
  pub total_size_mb: u64,
  pub analyzed_size_bytes: u64,
  pub analyzed_size_kb: u64,
  pub analyzed_size_mb: u64,
  pub largest_files: Vec<FileSizeInfo>,
  pub size_by_language: HashMap<String, u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanProgress {
  pub files_scanned: usize,
  pub scan_limit: usize,
  pub is_complete: bool,
  pub estimated_total_files: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepoAnalysis {
  pub files: Vec<FileInfo>,
  pub structure: HashMap<String, Vec<String>>,
  pub technologies: Vec<String>,
  pub metrics: HashMap<String, i32>,
  pub size_metrics: SizeMetrics,
  pub generated_at: Option<String>,
  pub from_cache: Option<bool>,
  pub is_lazy_scan: Option<bool>,
  pub scan_progress: Option<ScanProgress>,
}

// Internal structures for processing
struct FileMetadata {
  pub path: String,
  pub size: u64,
  pub language: String,
  pub parent: Option<String>,
}

struct FileProcessResult {
  pub file_info: Option<FileInfo>,
  pub lines: usize,
  pub language: String,
  pub parent: Option<String>,
  pub path: String,
  pub size: u64,
  pub is_analyzed: bool,
}

#[derive(Clone)]
struct LazyLoadConfig {
  initial_scan_limit: usize,   // how many files to scan in lazy mode
  sample_content_limit: usize, // how many file contents to sample in lazy mode
  max_file_size: u64,          // maximum size to read content
  batch_size: usize,           // not currently used in this file
}

impl Default for LazyLoadConfig {
  fn default() -> Self {
    Self {
      initial_scan_limit: 100,
      sample_content_limit: 25,
      max_file_size: 100_000,
      batch_size: 10,
    }
  }
}

#[derive(Debug, Clone, Serialize)]
struct ProgressUpdate {
  folder_path: String,
  phase: String,
  files_discovered: usize,
  files_processed: usize,
  total_files: usize,
  percentage: f64,
  current_file: Option<String>,
  is_complete: bool,
  is_favorite: bool,
  elapsed_ms: u64,
  estimated_remaining_ms: Option<u64>,
  bytes_processed: u64,
  total_bytes: Option<u64>,
  skipped_filtered: Option<usize>,
  dirs_seen: Option<usize>,
}

struct ProgressTracker {
  start: Instant,
  phase: Mutex<String>,
  current_file: Mutex<Option<String>>,
  files_discovered: AtomicUsize,
  files_processed: AtomicUsize,
  total_files: AtomicUsize,
  skipped_filtered: AtomicUsize,
  dirs_seen: AtomicUsize,
  bytes_processed: AtomicU64,
  total_bytes: AtomicU64,
  complete: AtomicBool,
}

impl ProgressTracker {
  fn new() -> Self {
    Self {
      start: Instant::now(),
      phase: Mutex::new("init".to_string()),
      current_file: Mutex::new(None),
      files_discovered: AtomicUsize::new(0),
      files_processed: AtomicUsize::new(0),
      total_files: AtomicUsize::new(0),
      skipped_filtered: AtomicUsize::new(0),
      dirs_seen: AtomicUsize::new(0),
      bytes_processed: AtomicU64::new(0),
      total_bytes: AtomicU64::new(0),
      complete: AtomicBool::new(false),
    }
  }

  fn set_phase(&self, phase: &str) {
    if let Ok(mut p) = self.phase.lock() { *p = phase.to_string(); }
  }
  fn set_current_file(&self, file: Option<String>) {
    if let Ok(mut cf) = self.current_file.lock() { *cf = file; }
  }
  fn increment_discovered(&self) {
    self.files_discovered.fetch_add(1, Ordering::Relaxed);
  }
  fn increment_processed(&self, bytes: usize) {
    self.files_processed.fetch_add(1, Ordering::Relaxed);
    self.bytes_processed.fetch_add(bytes as u64, Ordering::Relaxed);
  }
  fn increment_skipped_filtered(&self) {
    self.skipped_filtered.fetch_add(1, Ordering::Relaxed);
  }
  fn increment_dirs_seen(&self) {
    self.dirs_seen.fetch_add(1, Ordering::Relaxed);
  }
  fn set_total_files(&self, total: usize) { self.total_files.store(total, Ordering::Relaxed); }
  fn set_total_bytes(&self, total: usize) { self.total_bytes.store(total as u64, Ordering::Relaxed); }
  fn mark_complete(&self) { self.complete.store(true, Ordering::Relaxed); }

  fn get_progress(&self, folder_path: &str, is_favorite: bool) -> ProgressUpdate {
    let elapsed = self.start.elapsed();
    let elapsed_ms = (elapsed.as_secs() * 1000) + (elapsed.subsec_millis() as u64);
    let total = self.total_files.load(Ordering::Relaxed);
    let processed = self.files_processed.load(Ordering::Relaxed);
    let percentage = if total > 0 { (processed as f64 / total as f64) * 100.0 } else { 0.0 };
    let phase = self
      .phase
      .lock()
      .ok()
      .map(|p| p.clone())
      .unwrap_or_else(|| "unknown".to_string());
    let current_file = self
      .current_file
      .lock()
      .ok()
      .and_then(|p| p.clone());
    let bytes_processed = self.bytes_processed.load(Ordering::Relaxed);
    let total_bytes = self.total_bytes.load(Ordering::Relaxed);

    ProgressUpdate {
      folder_path: folder_path.to_string(),
      phase,
      files_discovered: self.files_discovered.load(Ordering::Relaxed),
      files_processed: processed,
      total_files: total,
      percentage,
      current_file,
      is_complete: self.complete.load(Ordering::Relaxed),
      is_favorite,
      elapsed_ms,
      estimated_remaining_ms: None,
      bytes_processed,
      total_bytes: Some(total_bytes),
      skipped_filtered: Some(self.skipped_filtered.load(Ordering::Relaxed)),
      dirs_seen: Some(self.dirs_seen.load(Ordering::Relaxed)),
    }
  }
}

// Simple byte conversion helpers
fn bytes_to_kb(bytes: u64) -> u64 { bytes / 1024 }
fn bytes_to_mb(bytes: u64) -> u64 { bytes / (1024 * 1024) }

// Global cancel flags per path
static CANCEL_FLAGS: Lazy<Mutex<HashMap<String, Arc<AtomicBool>>>> = Lazy::new(|| Mutex::new(HashMap::new()));

fn set_cancel_flag(path: &str, flag: Arc<AtomicBool>) {
  if let Ok(mut map) = CANCEL_FLAGS.lock() {
    map.insert(path.to_string(), flag);
  }
}

fn take_cancel_flag(path: &str) -> Option<Arc<AtomicBool>> {
  if let Ok(mut map) = CANCEL_FLAGS.lock() { map.remove(path) } else { None }
}

async fn is_favorite_project(db_pool: &Arc<DbPool>, folder_path: &str) -> bool {
  if let Ok(conn) = db_pool.get() {
    if let Ok(Some(project)) = db::get_project_by_path(&conn, folder_path) {
      return project.is_favorite;
    }
  }
  false
}

#[tauri::command]
pub async fn cancel_analysis(folder_path: String) -> Result<(), String> {
  // Set cancel flag if exists; do not remove it here to allow in-flight checks
  if let Ok(map) = CANCEL_FLAGS.lock() {
    if let Some(flag) = map.get(&folder_path) {
      flag.store(true, Ordering::Relaxed);
    }
  }
  Ok(())
}

// Process files in parallel batches
async fn process_files_parallel(
  files: &[FileMetadata],
  is_favorite: bool,
  sample_limit: usize,
  tracker: &Arc<ProgressTracker>,
) -> Vec<FileProcessResult> {
  let sampled_count = Arc::new(AtomicUsize::new(0));
  let max_content_size = if is_favorite { 150_000 } else { 100_000 } as u64;
  let content_limit = if is_favorite { 7500 } else { 5000 };

  let chunk_size = 50;
  let mut all_results = Vec::with_capacity(files.len());
  
  for chunk in files.chunks(chunk_size) {
    let chunk_results: Vec<FileProcessResult> = chunk
      .par_iter()
      .map(|metadata| {
        tracker.set_current_file(Some(metadata.path.clone()));
        let current_sampled = sampled_count.load(Ordering::Relaxed);
        let should_load = (metadata.size < max_content_size) && (current_sampled < sample_limit);
        let result: FileProcessResult;

        if should_load {
          sampled_count.fetch_add(1, Ordering::Relaxed);
          
          let (content, was_truncated) = read_text_prefix_limited(&metadata.path, content_limit)
            .unwrap_or_else(|_| (String::new(), false));
          
          let lines = content.lines().count();
          let display_content = if was_truncated {
            format!("{}...(truncated)", content)
          } else {
            content
          };
          
          let file_info = Some(FileInfo {
            path: metadata.path.clone(),
            content: display_content,
            language: metadata.language.clone(),
            size: metadata.size,
          });
          
          result = FileProcessResult {
            file_info,
            lines,
            language: metadata.language.clone(),
            parent: metadata.parent.clone(),
            path: metadata.path.clone(),
            size: metadata.size,
            is_analyzed: true,
          };
        } else {
          result = FileProcessResult {
            file_info: None,
            lines: 0,
            language: metadata.language.clone(),
            parent: metadata.parent.clone(),
            path: metadata.path.clone(),
            size: metadata.size,
            is_analyzed: false,
          };
        }

        tracker.increment_processed(metadata.size as usize);
        result
      })
      .collect();
    
    all_results.extend(chunk_results);
  }
  
  all_results
}

fn aggregate_results(results: Vec<FileProcessResult>) -> (
  Vec<FileInfo>,
  HashMap<String, Vec<String>>,
  Vec<String>,
  HashMap<String, i32>,
  SizeMetrics,
) {
  let mut files: Vec<FileInfo> = Vec::with_capacity(results.len() / 2);
  let mut structure: HashMap<String, Vec<String>> = HashMap::with_capacity(100);
  let mut technologies_set: HashSet<String> = HashSet::with_capacity(20);
  let mut size_by_language: HashMap<String, u64> = HashMap::with_capacity(20);
  let mut all_file_sizes: Vec<FileSizeInfo> = Vec::with_capacity(results.len());
  
  let (total_files, total_lines, total_size_bytes, analyzed_size_bytes) = results
    .par_iter()
    .map(|r| {
      let analyzed = if r.is_analyzed { r.size } else { 0 };
      (1i32, r.lines as i32, r.size, analyzed)
    })
    .reduce(
      || (0, 0, 0, 0),
      |acc, item| (acc.0 + item.0, acc.1 + item.1, acc.2 + item.2, acc.3 + item.3)
    );
  
  for r in &results {
    if r.language != "Unknown" {
      technologies_set.insert(r.language.clone());
      *size_by_language.entry(r.language.clone()).or_insert(0) += r.size;
    }
    
    all_file_sizes.push(FileSizeInfo {
      path: r.path.clone(),
      size_bytes: r.size,
      size_kb: bytes_to_kb(r.size),
      language: r.language.clone(),
    });
    
    if let Some(ref fi) = r.file_info {
      files.push(fi.clone());
      if let Some(parent) = &r.parent {
        let name = Path::new(&r.path)
          .file_name()
          .unwrap_or_default()
          .to_string_lossy()
          .to_string();
        structure.entry(parent.clone()).or_default().push(name);
      }
    }
  }
  
  all_file_sizes.par_sort_unstable_by(|a, b| b.size_bytes.cmp(&a.size_bytes));
  let largest_files: Vec<FileSizeInfo> = all_file_sizes.into_iter().take(10).collect();
  
  let technologies: Vec<String> = technologies_set.into_iter().collect();
  
  let mut metrics = HashMap::with_capacity(3);
  metrics.insert("total_files".to_string(), total_files);
  metrics.insert("total_lines".to_string(), total_lines);
  metrics.insert("analyzed_files".to_string(), files.len() as i32);
  
  let size_metrics = SizeMetrics {
    total_size_bytes,
    total_size_kb: bytes_to_kb(total_size_bytes),
    total_size_mb: bytes_to_mb(total_size_bytes),
    analyzed_size_bytes,
    analyzed_size_kb: bytes_to_kb(analyzed_size_bytes),
    analyzed_size_mb: bytes_to_mb(analyzed_size_bytes),
    largest_files,
    size_by_language,
  };
  
  (files, structure, technologies, metrics, size_metrics)
}

// Main analysis implementation with SQLite caching
async fn analyze_repository_impl(
  db_pool: Arc<DbPool>,
  folder_path: String,
  force: bool,
  use_lazy_scan: bool,
  trigger_full_scan: bool,
  window: Option<tauri::Window>,
) -> Result<RepoAnalysis, String> {
  let path = Path::new(&folder_path);
  if !path.exists() || !path.is_dir() {
    return Err("Invalid folder path".to_string());
  }

  let is_favorite = is_favorite_project(&db_pool, &folder_path).await;
  
  if is_favorite {
    println!("[Analysis] Analyzing favorite project with priority: {}", folder_path);
  }

  // Get or create project in database
  let conn = db_pool.get().map_err(|e| e.to_string())?;
  let project = db::get_project_by_path(&conn, &folder_path)
    .map_err(|e| e.to_string())?;
  
  let project_id = if let Some(p) = project {
    p.id
  } else {
    // Create new project entry
    let name = Path::new(&folder_path)
      .file_name()
      .and_then(|n| n.to_str())
      .unwrap_or("Unknown")
      .to_string();
    let is_git = path.join(".git").exists();
    
    let id = db::upsert_project(&conn, &folder_path, &name, None, is_git)
      .map_err(|e| e.to_string())?;
    id
  };

  // Cache check using SQLite
  if !force && !trigger_full_scan {
    if let Ok(Some(cached)) = db::get_cached_analysis(&conn, project_id) {
      let mut a = cached.clone();
      a.from_cache = Some(true);
      
      if a.is_lazy_scan.unwrap_or(false) && trigger_full_scan {
        // Continue to full scan
      } else {
        if let Some(w) = &window {
          let _ = w.emit("analysis:progress", &ProgressUpdate {
            folder_path: folder_path.clone(),
            phase: "cached".to_string(),
            files_discovered: a.metrics.get("total_files").copied().unwrap_or(0) as usize,
            files_processed: a.metrics.get("total_files").copied().unwrap_or(0) as usize,
            total_files: a.metrics.get("total_files").copied().unwrap_or(0) as usize,
            percentage: 100.0,
            current_file: None,
            is_complete: true,
            is_favorite,
            elapsed_ms: 0,
            estimated_remaining_ms: None,
            bytes_processed: a.size_metrics.total_size_bytes,
            total_bytes: Some(a.size_metrics.total_size_bytes),
            skipped_filtered: None,
            dirs_seen: None,
          });
        }
        return Ok(a);
      }
    }
  }

  // Perform analysis
  let mut config = LazyLoadConfig::default();
  if is_favorite {
    config.initial_scan_limit = 150;
    config.sample_content_limit = 30;
    config.max_file_size = 150_000;
    config.batch_size = 15;
  }

  let tracker = Arc::new(ProgressTracker::new());
  tracker.set_phase("discovery");

  let cancel_flag = Arc::new(AtomicBool::new(false));
  set_cancel_flag(&folder_path, cancel_flag.clone());

  let progress_handle = if let Some(w) = &window {
    Some(spawn_progress_emitter(
      w.clone(),
      tracker.clone(),
      folder_path.clone(),
      is_favorite,
    ).await)
  } else { None };

  // Discover files
  let mut file_metadatas: Vec<FileMetadata> = Vec::with_capacity(1000);
  
  let scan_limit = if use_lazy_scan && !trigger_full_scan {
    config.initial_scan_limit
  } else {
    usize::MAX
  };

  for result in walker(path).take(scan_limit) {
    if cancel_flag.load(Ordering::Relaxed) { break; }
    if let Ok(entry) = result {
      if entry.file_type().map_or(false, |ft| ft.is_file()) {
        tracker.increment_discovered();
        if should_analyze_file(&entry.path().to_string_lossy()) {
          if let Ok(metadata) = entry.metadata() {
            let path_str = entry.path().to_string_lossy().to_string();
            file_metadatas.push(FileMetadata {
              path: path_str,
              size: metadata.len(),
              language: get_language_from_extension(&entry.path().to_string_lossy()),
              parent: entry.path().parent().map(|p| p.to_string_lossy().to_string()),
            });
          }
        } else {
          tracker.increment_skipped_filtered();
        }
      } else if entry.file_type().map_or(false, |ft| ft.is_dir()) {
        tracker.increment_dirs_seen();
      }
    }
  }

  tracker.set_phase("processing");
  tracker.set_total_files(file_metadatas.len());
  let total_bytes: usize = file_metadatas.iter().map(|m| m.size as usize).sum();
  tracker.set_total_bytes(total_bytes);

  let results = process_files_parallel(
    &file_metadatas,
    is_favorite,
    if use_lazy_scan { config.sample_content_limit } else { file_metadatas.len() },
    &tracker,
  ).await;

  let (files, structure, technologies, metrics, size_metrics) = aggregate_results(results);

  let is_lazy = use_lazy_scan && !trigger_full_scan;
  let analysis = RepoAnalysis {
    files,
    structure,
    technologies,
    metrics,
    size_metrics,
    generated_at: Some(Utc::now().to_rfc3339()),
    from_cache: Some(false),
    is_lazy_scan: Some(is_lazy),
    scan_progress: if is_lazy {
      Some(ScanProgress {
        files_scanned: file_metadatas.len(),
        scan_limit,
        is_complete: file_metadatas.len() < scan_limit,
        estimated_total_files: Some(file_metadatas.len()),
      })
    } else {
      None
    },
  };

  // Cache the analysis in SQLite
  let ttl_hours = if is_favorite { 2 } else { 1 };
  if let Err(e) = db::cache_analysis(&conn, project_id, &analysis, ttl_hours) {
    eprintln!("Failed to cache analysis: {}", e);
  }

  if cancel_flag.load(Ordering::Relaxed) {
    tracker.set_phase("cancelled");
  } else {
    tracker.set_phase("complete");
  }
  tracker.mark_complete();
  
  if let Some(handle) = progress_handle { 
    let _ = tokio::time::timeout(Duration::from_secs(1), handle).await; 
  }
  let _ = take_cancel_flag(&folder_path);

  Ok(analysis)
}

async fn spawn_progress_emitter(
  window: tauri::Window,
  tracker: Arc<ProgressTracker>,
  folder_path: String,
  is_favorite: bool,
) -> tokio::task::JoinHandle<()> {
  tokio::spawn(async move {
    let mut interval = tokio::time::interval(Duration::from_millis(500));
    
    loop {
      interval.tick().await;
      let progress = tracker.get_progress(&folder_path, is_favorite);
      let _ = window.emit("analysis:progress", &progress);
      if progress.is_complete || progress.elapsed_ms > 300_000 {
        break;
      }
    }
  })
}

// Tauri command handlers
#[tauri::command]
pub async fn analyze_repository(
  db_pool: State<'_, Arc<DbPool>>,
  window: tauri::Window,
  folder_path: String,
) -> Result<RepoAnalysis, String> {
  analyze_repository_impl(db_pool.inner().clone(), folder_path, false, false, false, Some(window)).await
}

#[tauri::command]
pub async fn analyze_repository_fresh(
  db_pool: State<'_, Arc<DbPool>>,
  window: tauri::Window,
  folder_path: String,
) -> Result<RepoAnalysis, String> {
  analyze_repository_impl(db_pool.inner().clone(), folder_path, true, false, true, Some(window)).await
}

#[tauri::command]
pub async fn analyze_repository_lazy(
  db_pool: State<'_, Arc<DbPool>>,
  window: tauri::Window,
  folder_path: String,
) -> Result<RepoAnalysis, String> {
  analyze_repository_impl(db_pool.inner().clone(), folder_path, false, true, false, Some(window)).await
}

#[tauri::command]
pub async fn trigger_full_scan(
  db_pool: State<'_, Arc<DbPool>>,
  window: tauri::Window,
  folder_path: String,
) -> Result<RepoAnalysis, String> {
  analyze_repository_impl(db_pool.inner().clone(), folder_path, false, false, true, Some(window)).await
}

#[tauri::command]
pub async fn analyze_multiple_repositories(
  db_pool: State<'_, Arc<DbPool>>,
  window: tauri::Window,
  folder_paths: Vec<String>,
) -> Result<Vec<RepoAnalysis>, String> {
  let mut results = Vec::with_capacity(folder_paths.len());
  
  for (index, path) in folder_paths.iter().enumerate() {
    let _ = window.emit("batch:progress", serde_json::json!({
      "current": index + 1,
      "total": folder_paths.len(),
      "current_project": path,
    }));
    
    match analyze_repository_impl(
      db_pool.inner().clone(),
      path.clone(),
      false,
      true,
      false,
      Some(window.clone()),
    ).await {
      Ok(analysis) => results.push(analysis),
      Err(e) => {
        eprintln!("Failed to analyze {}: {}", path, e);
      }
    }
  }
  
  Ok(results)
}
