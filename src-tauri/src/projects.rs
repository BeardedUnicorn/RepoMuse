use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use std::sync::Arc;
use tauri::State;

use crate::fs_utils::{should_analyze_file, walker_parallel};
use crate::db::{self, DbPool};

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

fn count_project_files(path: &Path) -> usize {
    use std::sync::atomic::{AtomicUsize, Ordering};
    let counter = AtomicUsize::new(0);
    walker_parallel(path).run(|| {
        let c = &counter;
        Box::new(move |entry_res| {
            if let Ok(entry) = entry_res {
                if entry.file_type().map_or(false, |ft| ft.is_file()) {
                    let p = entry.path();
                    if should_analyze_file(&p.to_string_lossy()) {
                        c.fetch_add(1, Ordering::Relaxed);
                    }
                }
            }
            ignore::WalkState::Continue
        })
    });
    counter.load(Ordering::Relaxed)
}

fn process_project_directory(
    path: std::path::PathBuf,
    conn: &rusqlite::Connection,
) -> Option<ProjectDirectory> {
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
        let is_git_repo = path.join(".git").exists();
        let description = get_project_description(&path);
        
        // Get or create project in database
        let project = db::get_project_by_path(conn, &path_str).ok().flatten();
        
        let file_count = if let Some(p) = &project {
            p.file_count as usize
        } else {
            // First time seeing this project - do a quick count
            let count = count_project_files(&path);
            
            // Store in database
            let _ = db::upsert_project(
                conn,
                &path_str,
                &dir_name,
                description.as_deref(),
                is_git_repo,
            );
            
            if let Ok(Some(proj)) = db::get_project_by_path(conn, &path_str) {
                let _ = db::update_project_file_count(conn, proj.id, count as i64);
            }
            
            count
        };

        Some(ProjectDirectory {
            name: dir_name,
            path: path_str,
            is_git_repo,
            file_count,
            description,
            is_counting: false,
        })
    } else {
        None
    }
}

#[tauri::command]
pub async fn list_project_directories(
    db_pool: State<'_, Arc<DbPool>>,
    root_path: String,
) -> Result<Vec<ProjectDirectory>, String> {
    let root = Path::new(&root_path);
    if !root.exists() || !root.is_dir() {
        return Err("Invalid root directory".to_string());
    }

    let conn = db_pool.get().map_err(|e| e.to_string())?;
    
    let entries: Vec<std::path::PathBuf> = fs::read_dir(root)
        .map_err(|e| format!("Failed to read directory: {}", e))?
        .filter_map(|e| e.ok())
        .filter(|entry| entry.path().is_dir())
        .map(|entry| entry.path())
        .collect();

    // Process in parallel but collect sequentially for database access
    let mut projects = Vec::new();
    for path in entries {
        if let Some(project) = process_project_directory(path, &conn) {
            projects.push(project);
        }
    }

    projects.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
    Ok(projects)
}

#[tauri::command]
pub async fn update_project_file_count(
    db_pool: State<'_, Arc<DbPool>>,
    project_path: String,
) -> Result<usize, String> {
    let path = Path::new(&project_path);
    if !path.exists() || !path.is_dir() { 
        return Err("Invalid project path".to_string()); 
    }
    
    let count = count_project_files(path);
    
    let conn = db_pool.get().map_err(|e| e.to_string())?;
    
    // Get or create project
    if let Ok(Some(project)) = db::get_project_by_path(&conn, &project_path) {
        db::update_project_file_count(&conn, project.id, count as i64)
            .map_err(|e| e.to_string())?;
    }
    
    Ok(count)
}
