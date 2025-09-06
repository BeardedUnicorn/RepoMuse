use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;
use std::time::{Duration, SystemTime};
use walkdir::WalkDir;
use chrono::{DateTime, Utc};

use crate::cache::{load_analysis_cache, save_analysis_cache, AnalysisCacheEntry};
use crate::fs_utils::{get_dir_modified_time, get_language_from_extension, should_analyze_file};

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
  pub generated_at: Option<String>,
  pub from_cache: Option<bool>,
}

async fn analyze_repository_impl(folder_path: String, force: bool) -> Result<RepoAnalysis, String> {
  let path = Path::new(&folder_path);
  if !path.exists() || !path.is_dir() {
    return Err("Invalid folder path".to_string());
  }

  // Cache for up to 1 hour
  let last_modified = get_dir_modified_time(path);
  let now = SystemTime::now()
    .duration_since(SystemTime::UNIX_EPOCH)
    .unwrap_or(Duration::from_secs(0))
    .as_secs();
  const TTL_SECS: u64 = 3600;

  let mut cache = load_analysis_cache();
  if !force {
    if let Some(entry) = cache.get(&folder_path).cloned() {
      if entry.last_modified >= last_modified && (now - entry.cached_at) < TTL_SECS {
        let mut a = entry.analysis.clone();
        a.from_cache = Some(true);
        let ts: DateTime<Utc> = DateTime::<Utc>::from_timestamp(entry.cached_at as i64, 0)
          .unwrap_or_else(|| Utc::now());
        a.generated_at = Some(ts.to_rfc3339());
        return Ok(a);
      }
    }
  }

  // Collect candidate files and process in parallel
  let valid_files: Vec<_> = WalkDir::new(&folder_path)
    .into_iter()
    .filter_map(|e| e.ok())
    .filter(|entry| entry.path().is_file())
    .filter(|entry| should_analyze_file(&entry.path().to_string_lossy()))
    .map(|entry| entry.path().to_owned())
    .collect();

  #[derive(Clone)]
  struct FileProcessResult {
    file_info: Option<FileInfo>,
    lines: usize,
    language: String,
    parent: Option<String>,
    path: String,
  }

  let results: Vec<FileProcessResult> = valid_files
    .par_iter()
    .filter_map(|path| {
      let path_str = path.to_string_lossy().to_string();
      match fs::read_to_string(path) {
        Ok(content) => {
          let lines = content.lines().count();
          let language = get_language_from_extension(&path_str);
          let include = content.len() < 100_000;
          let file_info = if include {
            Some(FileInfo {
              path: path_str.clone(),
              content: if content.len() > 5000 {
                format!("{}...(truncated)", &content[..5000])
              } else {
                content
              },
              language: language.clone(),
              size: path.metadata().map(|m| m.len()).unwrap_or(0),
            })
          } else { None };
          let parent = path.parent().map(|p| p.to_string_lossy().to_string());
          Some(FileProcessResult { file_info, lines, language, parent, path: path_str })
        }
        Err(_) => None,
      }
    })
    .collect();

  let mut files: Vec<FileInfo> = Vec::new();
  let mut structure: HashMap<String, Vec<String>> = HashMap::new();
  let mut technologies_set: HashSet<String> = HashSet::new();
  let mut total_files: i32 = 0;
  let mut total_lines: i32 = 0;

  for r in &results {
    total_files += 1;
    total_lines += r.lines as i32;
    if r.language != "Unknown" { technologies_set.insert(r.language.clone()); }
    if let Some(ref fi) = r.file_info {
      files.push(fi.clone());
      if let Some(parent) = &r.parent {
        let name = Path::new(&r.path).file_name().unwrap_or_default().to_string_lossy().to_string();
        structure.entry(parent.clone()).or_default().push(name);
      }
    }
  }

  let technologies: Vec<String> = technologies_set.into_iter().collect();
  let mut metrics = HashMap::new();
  metrics.insert("total_files".to_string(), total_files);
  metrics.insert("total_lines".to_string(), total_lines);
  metrics.insert("analyzed_files".to_string(), files.len() as i32);

  let mut analysis = RepoAnalysis { files, structure, technologies, metrics, generated_at: None, from_cache: Some(false) };
  analysis.generated_at = Some(Utc::now().to_rfc3339());

  cache.insert(
    folder_path.clone(),
    AnalysisCacheEntry { path: folder_path, last_modified, cached_at: now, analysis: analysis.clone() },
  );
  save_analysis_cache(&cache);

  Ok(analysis)
}

#[tauri::command]
pub async fn analyze_repository(folder_path: String) -> Result<RepoAnalysis, String> {
  analyze_repository_impl(folder_path, false).await
}

#[tauri::command]
pub async fn analyze_repository_fresh(folder_path: String) -> Result<RepoAnalysis, String> {
  analyze_repository_impl(folder_path, true).await
}

