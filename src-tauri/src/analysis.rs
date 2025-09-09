use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime, Instant};
use std::sync::{Arc, Mutex, RwLock};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
// use ignore::WalkBuilder; // now using helper walkers in fs_utils
use chrono::{DateTime, Utc};
use tauri::Emitter;
use tokio::task;
use tokio::sync::mpsc;
use once_cell::sync::Lazy;

use crate::cache::{
  load_analysis_cache,
  save_analysis_cache,
  AnalysisCacheEntry,
};
use crate::fs_utils::{get_dir_modified_time, get_language_from_extension, should_analyze_file, walker, walker_parallel, read_text_prefix, short_hash_prefix};
use crate::cache::{load_file_metadata_cache, save_file_metadata_cache, FileMetadataCache};
use crate::storage::load_favorite_projects;

static CANCEL_FLAGS: Lazy<Mutex<HashMap<String, Arc<AtomicBool>>>> = Lazy::new(|| Mutex::new(HashMap::new()));

fn set_cancel_flag(path: &str, flag: Arc<AtomicBool>) {
  if let Ok(mut map) = CANCEL_FLAGS.lock() {
    map.insert(path.to_string(), flag);
  }
}

fn take_cancel_flag(path: &str) -> Option<Arc<AtomicBool>> {
  if let Ok(mut map) = CANCEL_FLAGS.lock() {
    map.remove(path)
  } else { None }
}

fn get_cancel_flag(path: &str) -> Option<Arc<AtomicBool>> {
  if let Ok(map) = CANCEL_FLAGS.lock() {
    map.get(path).cloned()
  } else { None }
}

#[tauri::command]
pub async fn cancel_analysis(folder_path: String) -> Result<(), String> {
  if let Some(flag) = get_cancel_flag(&folder_path) {
    flag.store(true, Ordering::Relaxed);
    Ok(())
  } else {
    Err("No running analysis for this path".into())
  }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FileInfo {
  pub path: String,
  pub content: String,
  pub language: String,
  pub size: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SizeMetrics {
  pub total_size_bytes: u64,
  pub total_size_kb: f64,
  pub total_size_mb: f64,
  pub analyzed_size_bytes: u64,
  pub analyzed_size_kb: f64,
  pub analyzed_size_mb: f64,
  pub largest_files: Vec<FileSizeInfo>,
  pub size_by_language: HashMap<String, u64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FileSizeInfo {
  pub path: String,
  pub size_bytes: u64,
  pub size_kb: f64,
  pub language: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ScanProgress {
  pub files_scanned: usize,
  pub scan_limit: usize,
  pub is_complete: bool,
  pub estimated_total_files: Option<usize>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProgressUpdate {
  pub folder_path: String,
  pub phase: String,
  pub files_discovered: usize,
  pub files_processed: usize,
  pub total_files: usize,
  pub percentage: f64,
  pub current_file: Option<String>,
  pub is_complete: bool,
  pub is_favorite: bool,
  pub elapsed_ms: u64,
  pub estimated_remaining_ms: Option<u64>,
  pub bytes_processed: u64,
  pub total_bytes: Option<u64>,
  pub skipped_filtered: Option<usize>,
  pub dirs_seen: Option<usize>,
}

#[derive(Clone)]
struct FileMetadata {
  path: String,
  size: u64,
  language: String,
  parent: Option<String>,
}

#[derive(Clone)]
struct FileProcessResult {
  file_info: Option<FileInfo>,
  lines: usize,
  language: String,
  parent: Option<String>,
  path: String,
  size: u64,
  is_analyzed: bool,
}

// Configuration for lazy scanning
#[derive(Debug, Clone)]
pub struct LazyLoadConfig {
  pub initial_scan_limit: usize,
  pub sample_content_limit: usize,
  pub max_file_size: u64,
  pub batch_size: usize,
  pub channel_buffer_size: usize,
}

impl Default for LazyLoadConfig {
  fn default() -> Self {
    LazyLoadConfig {
      initial_scan_limit: 100,
      sample_content_limit: 20,
      max_file_size: 100_000,
      batch_size: 10,
      channel_buffer_size: 100,
    }
  }
}

// Removed lock-heavy StreamingProcessor; we now map/reduce in parallel

// Progress tracker for streaming analysis
struct ProgressTracker {
  start_time: Instant,
  files_discovered: Arc<AtomicUsize>,
  files_processed: Arc<AtomicUsize>,
  bytes_processed: Arc<AtomicUsize>,
  total_files: Arc<AtomicUsize>,
  total_bytes: Arc<AtomicUsize>,
  is_complete: Arc<AtomicBool>,
  current_file: Arc<RwLock<Option<String>>>,
  phase: Arc<RwLock<String>>,
  skipped_filtered: Arc<AtomicUsize>,
  dirs_seen: Arc<AtomicUsize>,
}

impl ProgressTracker {
  fn new() -> Self {
    ProgressTracker {
      start_time: Instant::now(),
      files_discovered: Arc::new(AtomicUsize::new(0)),
      files_processed: Arc::new(AtomicUsize::new(0)),
      bytes_processed: Arc::new(AtomicUsize::new(0)),
      total_files: Arc::new(AtomicUsize::new(0)),
      total_bytes: Arc::new(AtomicUsize::new(0)),
      is_complete: Arc::new(AtomicBool::new(false)),
      current_file: Arc::new(RwLock::new(None)),
      phase: Arc::new(RwLock::new("discovery".to_string())),
      skipped_filtered: Arc::new(AtomicUsize::new(0)),
      dirs_seen: Arc::new(AtomicUsize::new(0)),
    }
  }

  fn set_current_file(&self, file: Option<String>) {
    if let Ok(mut guard) = self.current_file.write() {
      *guard = file;
    }
  }

  fn increment_discovered(&self) -> usize {
    self.files_discovered.fetch_add(1, Ordering::Relaxed)
  }

  fn increment_processed(&self, bytes: usize) -> usize {
    self.bytes_processed.fetch_add(bytes, Ordering::Relaxed);
    self.files_processed.fetch_add(1, Ordering::Relaxed)
  }

  fn set_phase(&self, phase: &str) {
    if let Ok(mut p) = self.phase.write() { *p = phase.to_string(); }
  }

  fn increment_skipped_filtered(&self) { self.skipped_filtered.fetch_add(1, Ordering::Relaxed); }
  fn increment_dirs_seen(&self) { self.dirs_seen.fetch_add(1, Ordering::Relaxed); }

  fn set_total_files(&self, total: usize) {
    self.total_files.store(total, Ordering::Relaxed);
  }

  fn set_total_bytes(&self, total: usize) {
    self.total_bytes.store(total, Ordering::Relaxed);
  }

  fn mark_complete(&self) {
    self.is_complete.store(true, Ordering::Relaxed);
  }

  fn get_progress(&self, folder_path: &str, is_favorite: bool) -> ProgressUpdate {
    let files_discovered = self.files_discovered.load(Ordering::Relaxed);
    let files_processed = self.files_processed.load(Ordering::Relaxed);
    let total_files = self.total_files.load(Ordering::Relaxed);
    let bytes_processed = self.bytes_processed.load(Ordering::Relaxed);
    let total_bytes = self.total_bytes.load(Ordering::Relaxed);
    let is_complete = self.is_complete.load(Ordering::Relaxed);
    let elapsed = self.start_time.elapsed();
    let phase = self
      .phase
      .read()
      .ok()
      .map(|g| g.clone())
      .unwrap_or_else(|| "".to_string());
    let skipped_filtered = self.skipped_filtered.load(Ordering::Relaxed);
    let dirs_seen = self.dirs_seen.load(Ordering::Relaxed);
    
    let percentage = if total_files > 0 {
      (files_processed as f64 / total_files as f64) * 100.0
    } else if files_discovered > 0 {
      (files_processed as f64 / files_discovered as f64) * 100.0
    } else {
      0.0
    };

    let estimated_remaining_ms = if files_processed > 0 && total_files > files_processed {
      let elapsed_ms = elapsed.as_millis() as u64;
      let rate = elapsed_ms as f64 / files_processed as f64;
      let remaining_files = total_files - files_processed;
      Some((rate * remaining_files as f64) as u64)
    } else {
      None
    };

    ProgressUpdate {
      folder_path: folder_path.to_string(),
      phase,
      files_discovered,
      files_processed,
      total_files: total_files.max(files_discovered),
      percentage,
      current_file: self.current_file.read().ok().and_then(|guard| guard.clone()),
      is_complete,
      is_favorite,
      elapsed_ms: elapsed.as_millis() as u64,
      estimated_remaining_ms,
      bytes_processed: bytes_processed as u64,
      total_bytes: if total_bytes > 0 { Some(total_bytes as u64) } else { None },
      skipped_filtered: Some(skipped_filtered),
      dirs_seen: Some(dirs_seen),
    }
  }
}

// Check if a project is a favorite
async fn is_favorite_project(folder_path: &str) -> bool {
  match load_favorite_projects().await {
    Ok(favorites) => favorites.contains(&folder_path.to_string()),
    Err(_) => false,
  }
}

fn bytes_to_kb(bytes: u64) -> f64 {
  (bytes as f64) / 1024.0
}

fn bytes_to_mb(bytes: u64) -> f64 {
  (bytes as f64) / (1024.0 * 1024.0)
}

// (removed unused: discover_files_streaming)

// Process files in parallel batches using rayon scope
fn process_files_parallel(
  files: &[FileMetadata],
  is_favorite: bool,
  sample_limit: usize,
  tracker: &Arc<ProgressTracker>,
) -> Vec<FileProcessResult> {
  let sampled_count = Arc::new(AtomicUsize::new(0));
  let max_content_size = if is_favorite { 150_000 } else { 100_000 } as u64;
  let content_limit = if is_favorite { 7500 } else { 5000 };

  files
    .par_iter()
    .map(|metadata| {
      tracker.set_current_file(Some(metadata.path.clone()));
      let current_sampled = sampled_count.load(Ordering::Relaxed);
      let should_load = (metadata.size < max_content_size) && (current_sampled < sample_limit);
      let result: FileProcessResult;

      if should_load {
        sampled_count.fetch_add(1, Ordering::Relaxed);
        let prefix = read_text_prefix(&metadata.path, content_limit).unwrap_or_default();
        let lines = prefix.lines().count();
        let file_info = Some(FileInfo {
          path: metadata.path.clone(),
          content: if prefix.len() >= content_limit { format!("{}...(truncated)", prefix) } else { prefix },
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
    .collect()
}

fn aggregate_results(results: Vec<FileProcessResult>) -> (
  Vec<FileInfo>,
  HashMap<String, Vec<String>>,
  Vec<String>,
  HashMap<String, i32>,
  SizeMetrics,
) {
  let mut files: Vec<FileInfo> = Vec::new();
  let mut structure: HashMap<String, Vec<String>> = HashMap::new();
  let mut technologies_set: HashSet<String> = HashSet::new();
  let mut size_by_language: HashMap<String, u64> = HashMap::new();
  let mut all_file_sizes: Vec<FileSizeInfo> = Vec::new();
  
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
  
  let mut metrics = HashMap::new();
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

// Streaming lazy analysis with incremental results
async fn analyze_repository_lazy_streaming(
  folder_path: String,
  window: Option<tauri::Window>,
) -> Result<RepoAnalysis, String> {
  let path = Path::new(&folder_path);
  if !path.exists() || !path.is_dir() {
    return Err("Invalid folder path".to_string());
  }

  let is_favorite = is_favorite_project(&folder_path).await;
  
  if is_favorite {
    println!("[Streaming Analysis] Analyzing favorite project: {}", folder_path);
  }

  // Create configuration
  let mut config = LazyLoadConfig::default();
  if is_favorite {
    config.initial_scan_limit = 150;
    config.sample_content_limit = 30;
    config.max_file_size = 150_000;
    config.batch_size = 15;
  }

  // Create tracker
  let tracker = Arc::new(ProgressTracker::new());
  
  // Create and register a cancel flag for this analysis run
  let cancel_flag = Arc::new(AtomicBool::new(false));
  set_cancel_flag(&folder_path, cancel_flag.clone());
  
  // Create channel for streaming file paths (bounded)
  let (tx, mut rx) = mpsc::channel::<PathBuf>(config.channel_buffer_size);
  
  // Start progress emitter if window provided
  tracker.set_phase("discovery");
  let progress_handle = if let Some(w) = &window {
    Some(spawn_progress_emitter(
      w.clone(),
      tracker.clone(),
      folder_path.clone(),
      is_favorite,
    ).await)
  } else {
    None
  };

  // Load file metadata cache to prioritize changed files
  let preloaded_fcache: FileMetadataCache = load_file_metadata_cache();

  // Spawn parallel file discovery task using ignore::WalkParallel
  let folder_path_clone = path.to_path_buf();
  let tracker_clone = tracker.clone();
  let cflag_discovery = cancel_flag.clone();
  let discovery_handle = task::spawn_blocking(move || {
    use std::sync::atomic::{AtomicUsize, Ordering};
    let count = Arc::new(AtomicUsize::new(0));
    walker_parallel(&folder_path_clone).run(|| {
      let tx = tx.clone();
      let tracker = tracker_clone.clone();
      let cflag = cflag_discovery.clone();
      let count = count.clone();
      Box::new(move |entry_res| {
        if cflag.load(Ordering::Relaxed) { return ignore::WalkState::Quit; }
        match entry_res {
          Ok(entry) => {
            if entry.file_type().map_or(false, |ft| ft.is_file()) {
              tracker.increment_discovered();
              if should_analyze_file(&entry.path().to_string_lossy()) {
                // backpressure via bounded channel
                let _ = tx.blocking_send(entry.path().to_owned());
                count.fetch_add(1, Ordering::Relaxed);
              } else {
                tracker.increment_skipped_filtered();
              }
            } else if entry.file_type().map_or(false, |ft| ft.is_dir()) {
              tracker.increment_dirs_seen();
            }
          }
          Err(_) => {}
        }
        ignore::WalkState::Continue
      })
    });
    count.load(Ordering::Relaxed)
  });

  // Process files as they come in; prioritize changed/new files
  let mut changed = Vec::new();
  let mut unchanged = Vec::new();
  let mut collected_count = 0;
  
  while let Some(file_path) = rx.recv().await {
    if cancel_flag.load(Ordering::Relaxed) { break; }
    if collected_count >= config.initial_scan_limit {
      break;
    }
    
    let path_str = file_path.to_string_lossy().to_string();
    let language = get_language_from_extension(&path_str);
    let parent = file_path.parent().map(|p| p.to_string_lossy().to_string());
    
    if let Ok(metadata) = file_path.metadata() {
      let meta = FileMetadata {
        path: path_str.clone(),
        size: metadata.len(),
        language,
        parent,
      };
      let unchanged_entry = preloaded_fcache.get_valid_metadata(&path_str);
      if unchanged_entry.is_some() {
        unchanged.push(meta);
      } else {
        changed.push(meta);
      }
      collected_count += 1;
    }
  }

  // Merge changed first to hit the cap, then unchanged
  let mut file_metadatas = Vec::with_capacity(collected_count);
  file_metadatas.extend(changed.into_iter());
  if file_metadatas.len() < collected_count {
    let need = collected_count - file_metadatas.len();
    file_metadatas.extend(unchanged.into_iter().take(need));
  }

  // Switch to processing phase
  tracker.set_phase("processing");

  // Process collected files in parallel (lock-free map/reduce)
  tracker.set_total_files(file_metadatas.len());
  let total_bytes: usize = file_metadatas.iter().map(|m| m.size as usize).sum();
  tracker.set_total_bytes(total_bytes);
  let results = process_files_parallel(
    &file_metadatas,
    is_favorite,
    config.sample_content_limit,
    &tracker,
  );

  // Mark as complete or cancelled
  if cancel_flag.load(Ordering::Relaxed) {
    tracker.set_phase("cancelled");
  } else {
    tracker.set_phase("complete");
  }
  tracker.mark_complete();
  
  // Wait for progress emitter to finish
  if let Some(handle) = progress_handle {
    let _ = tokio::time::timeout(Duration::from_secs(1), handle).await;
  }

  // Get total discovered files
  let total_discovered = discovery_handle.await.unwrap_or(0);
  
  // Aggregate final results
  let (files, structure, technologies, metrics, size_metrics) = aggregate_results(results);
  
  let scan_progress = ScanProgress {
    files_scanned: collected_count,
    scan_limit: config.initial_scan_limit,
    is_complete: collected_count >= total_discovered,
    estimated_total_files: Some(total_discovered),
  };
  
  let analysis = RepoAnalysis {
    files,
    structure,
    technologies,
    metrics,
    size_metrics,
    generated_at: Some(Utc::now().to_rfc3339()),
    from_cache: Some(false),
    is_lazy_scan: Some(true),
    scan_progress: Some(scan_progress),
  };

  // Persist to analysis cache
  let mut cache = load_analysis_cache();
  let entry = AnalysisCacheEntry {
    path: folder_path.clone(),
    last_modified: get_dir_modified_time(path),
    cached_at: SystemTime::now()
      .duration_since(SystemTime::UNIX_EPOCH)
      .unwrap_or(Duration::from_secs(0))
      .as_secs(),
    analysis: analysis.clone(),
  };
  cache.insert(folder_path.clone(), entry);
  save_analysis_cache(&cache);

  // Persist file metadata cache (incremental)
  let mut fcache: FileMetadataCache = load_file_metadata_cache();
  for m in &file_metadatas {
    // compute short hash only for sampled content to keep it cheap
    let short = short_hash_prefix(&m.path, 64 * 1024);
    let _ = fcache.insert_metadata_with_hash(m.path.clone(), m.language.clone(), m.size, short);
  }
  save_file_metadata_cache(&fcache);
  // Remove cancel flag for this run
  let _ = take_cancel_flag(&folder_path);

  Ok(analysis)
}

// Spawn progress emitter task
async fn spawn_progress_emitter(
  window: tauri::Window,
  tracker: Arc<ProgressTracker>,
  folder_path: String,
  is_favorite: bool,
) -> tokio::task::JoinHandle<()> {
  tokio::spawn(async move {
    // Emit progress ~5x/second to reduce UI/event overhead
    let mut interval = tokio::time::interval(Duration::from_millis(200));
    
    loop {
      interval.tick().await;
      
      let progress = tracker.get_progress(&folder_path, is_favorite);
      
      let _ = window.emit("analysis:progress", &progress);
      
      if progress.is_complete {
        break;
      }
      
      if progress.elapsed_ms > 300_000 {
        break;
      }
    }
  })
}

// Main analysis implementation (keeping existing structure for compatibility)
async fn analyze_repository_impl(
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

  let is_favorite = is_favorite_project(&folder_path).await;
  
  if is_favorite {
    println!("[Analysis] Analyzing favorite project with priority: {}", folder_path);
  }

  // Cache check
  let last_modified = get_dir_modified_time(path);
  let now = SystemTime::now()
    .duration_since(SystemTime::UNIX_EPOCH)
    .unwrap_or(Duration::from_secs(0))
    .as_secs();
  
  const DEFAULT_TTL_SECS: u64 = 3600;
  const FAVORITE_TTL_SECS: u64 = 7200;
  let ttl_secs = if is_favorite { FAVORITE_TTL_SECS } else { DEFAULT_TTL_SECS };

  let cache = load_analysis_cache();
  
  if !force && !trigger_full_scan {
    if let Some(entry) = cache.get(&folder_path).cloned() {
      if entry.last_modified >= last_modified && (now - entry.cached_at) < ttl_secs {
        let mut a = entry.analysis.clone();
        a.from_cache = Some(true);
        let ts: DateTime<Utc> = DateTime::<Utc>::from_timestamp(entry.cached_at as i64, 0)
          .unwrap_or_else(|| Utc::now());
        a.generated_at = Some(ts.to_rfc3339());
        
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
  }

  // Use streaming lazy scan if requested
  if use_lazy_scan && !trigger_full_scan {
    return analyze_repository_lazy_streaming(folder_path, window).await;
  }

  // Fall back to full scan implementation (keeping existing code)
  // Full scan: discover all files, then process in parallel
  let mut config = LazyLoadConfig::default();
  if is_favorite {
    config.initial_scan_limit = usize::MAX; // not used for full scan
    config.sample_content_limit = 50; // read larger prefixes for favorites
    config.max_file_size = 200_000;
    config.batch_size = 32;
  }

  let tracker = Arc::new(ProgressTracker::new());
  tracker.set_phase("discovery");

  // Register cancel flag
  let cancel_flag = Arc::new(AtomicBool::new(false));
  set_cancel_flag(&folder_path, cancel_flag.clone());

  // Start progress emitter if window provided
  let progress_handle = if let Some(w) = &window {
    Some(spawn_progress_emitter(
      w.clone(),
      tracker.clone(),
      folder_path.clone(),
      is_favorite,
    ).await)
  } else { None };

  // Discover files (single-threaded walk for simplicity and correctness)
  let mut file_metadatas: Vec<FileMetadata> = Vec::new();
  for result in walker(path) {
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

  // Switch to processing phase and compute totals
  tracker.set_phase("processing");
  tracker.set_total_files(file_metadatas.len());
  let total_bytes: usize = file_metadatas.iter().map(|m| m.size as usize).sum();
  tracker.set_total_bytes(total_bytes);

  // Process all collected files in parallel
  let results = process_files_parallel(
    &file_metadatas,
    is_favorite,
    file_metadatas.len().max(config.sample_content_limit), // effectively sample all under thresholds
    &tracker,
  );

  // Aggregate
  let (files, structure, technologies, metrics, size_metrics) = aggregate_results(results);

  // Build analysis object
  let analysis = RepoAnalysis {
    files,
    structure,
    technologies,
    metrics,
    size_metrics,
    generated_at: Some(Utc::now().to_rfc3339()),
    from_cache: Some(false),
    is_lazy_scan: Some(false),
    scan_progress: None,
  };

  // Persist analysis cache
  let mut cache = load_analysis_cache();
  let entry = AnalysisCacheEntry {
    path: folder_path.clone(),
    last_modified: get_dir_modified_time(path),
    cached_at: SystemTime::now()
      .duration_since(SystemTime::UNIX_EPOCH)
      .unwrap_or(Duration::from_secs(0))
      .as_secs(),
    analysis: analysis.clone(),
  };
  cache.insert(folder_path.clone(), entry);
  save_analysis_cache(&cache);

  // Persist file metadata cache (full)
  let mut fcache: FileMetadataCache = load_file_metadata_cache();
  for m in &file_metadatas {
    let short = short_hash_prefix(&m.path, 64 * 1024);
    let _ = fcache.insert_metadata_with_hash(m.path.clone(), m.language.clone(), m.size, short);
  }
  save_file_metadata_cache(&fcache);

  // Mark completion state and cleanup
  if cancel_flag.load(Ordering::Relaxed) {
    tracker.set_phase("cancelled");
  } else {
    tracker.set_phase("complete");
  }
  tracker.mark_complete();
  if let Some(handle) = progress_handle { let _ = tokio::time::timeout(Duration::from_secs(1), handle).await; }
  let _ = take_cancel_flag(&folder_path);

  Ok(analysis)
}

#[tauri::command]
pub async fn analyze_repository(window: tauri::Window, folder_path: String) -> Result<RepoAnalysis, String> {
  analyze_repository_impl(folder_path, false, false, false, Some(window)).await
}

#[tauri::command]
pub async fn analyze_repository_fresh(window: tauri::Window, folder_path: String) -> Result<RepoAnalysis, String> {
  analyze_repository_impl(folder_path, true, false, true, Some(window)).await
}

#[tauri::command]
pub async fn analyze_repository_lazy(window: tauri::Window, folder_path: String) -> Result<RepoAnalysis, String> {
  analyze_repository_impl(folder_path, false, true, false, Some(window)).await
}

#[tauri::command]
pub async fn trigger_full_scan(window: tauri::Window, folder_path: String) -> Result<RepoAnalysis, String> {
  analyze_repository_impl(folder_path, false, false, true, Some(window)).await
}

// Batch analysis with priority queue and progress
#[tauri::command]
pub async fn analyze_multiple_repositories(
  window: tauri::Window,
  folder_paths: Vec<String>,
) -> Result<Vec<RepoAnalysis>, String> {
  let mut results = Vec::new();
  
  for (index, path) in folder_paths.iter().enumerate() {
    let _ = window.emit("batch:progress", serde_json::json!({
      "current": index + 1,
      "total": folder_paths.len(),
      "current_project": path,
    }));
    
    match analyze_repository_lazy_streaming(path.clone(), Some(window.clone())).await {
      Ok(analysis) => results.push(analysis),
      Err(e) => {
        eprintln!("Failed to analyze {}: {}", path, e);
      }
    }
  }
  
  Ok(results)
}
