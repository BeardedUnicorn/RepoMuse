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