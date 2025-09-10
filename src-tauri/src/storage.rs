use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tauri::State;
use crate::db::{self, DbPool};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Settings {
    pub api_url: String,
    pub model: String,
    pub api_key: String,
    #[serde(default = "default_temperature_ideas")]
    pub temperature_ideas: f32,
    #[serde(default = "default_frequency_penalty_ideas")]
    pub frequency_penalty_ideas: f32,
    #[serde(default = "default_presence_penalty_ideas")]
    pub presence_penalty_ideas: f32,
    #[serde(default = "default_max_tokens_ideas")]
    pub max_tokens_ideas: u32,
    #[serde(default = "default_temperature_summary")]
    pub temperature_summary: f32,
    #[serde(default = "default_presence_penalty_summary")]
    pub presence_penalty_summary: f32,
    #[serde(default = "default_max_tokens_summary")]
    pub max_tokens_summary: u32,
    #[serde(default = "default_use_stop_ideas")]
    pub use_stop_ideas: bool,
}

fn default_temperature_ideas() -> f32 { 0.6 }
fn default_frequency_penalty_ideas() -> f32 { 0.3 }
fn default_presence_penalty_ideas() -> f32 { 0.1 }
fn default_max_tokens_ideas() -> u32 { 1500 }
fn default_temperature_summary() -> f32 { 0.4 }
fn default_presence_penalty_summary() -> f32 { 0.1 }
fn default_max_tokens_summary() -> u32 { 1200 }
fn default_use_stop_ideas() -> bool { true }

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ThemePreference {
    pub theme: String, // "light" or "dark" or "system"
    pub last_updated: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Task {
    pub id: String,
    pub text: String,
    pub completed: bool,
    pub created_at: String,
    pub completed_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TaskList {
    pub project_path: String,
    pub tasks: Vec<Task>,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FavoriteProjects {
    pub favorites: Vec<String>,
    pub last_updated: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProjectSummary {
    pub project_path: String,
    pub summary: String,
    pub generated_at: String,
    pub technologies: Vec<String>,
    pub key_features: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DatabaseStats {
    pub total_projects: i64,
    pub total_files: i64,
    pub total_size_bytes: i64,
    pub cached_analyses: i64,
    pub total_tasks: i64,
    pub total_summaries: i64,
    pub database_size_bytes: i64,
    pub database_size_mb: f64,
}

#[tauri::command]
pub async fn get_app_data_directory() -> Result<String, String> {
    dirs::data_local_dir()
        .ok_or("Failed to get app data directory".to_string())
        .map(|d| d.join("repomuse").to_string_lossy().to_string())
}

#[tauri::command]
pub async fn get_database_stats(
    db_pool: State<'_, Arc<DbPool>>,
) -> Result<DatabaseStats, String> {
    let conn = db_pool.get().map_err(|e| e.to_string())?;
    
    // Get project statistics
    let (total_projects, total_files, total_size_bytes) = conn.query_row(
        "SELECT COUNT(*) as project_count,
                COALESCE(SUM(file_count), 0) as total_files,
                COALESCE(SUM(total_size_bytes), 0) as total_size
         FROM projects",
        [],
        |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)),
    ).map_err(|e| e.to_string())?;
    
    // Get cached analyses count
    let cached_analyses: i64 = conn.query_row(
        "SELECT COUNT(*) FROM analysis_cache WHERE expires_at > CURRENT_TIMESTAMP",
        [],
        |row| row.get(0),
    ).map_err(|e| e.to_string())?;
    
    // Get total tasks
    let total_tasks: i64 = conn.query_row(
        "SELECT COUNT(*) FROM tasks",
        [],
        |row| row.get(0),
    ).map_err(|e| e.to_string())?;
    
    // Get total summaries
    let total_summaries: i64 = conn.query_row(
        "SELECT COUNT(*) FROM summaries",
        [],
        |row| row.get(0),
    ).map_err(|e| e.to_string())?;
    
    // Get database file size
    let database_size_bytes: i64 = conn.query_row(
        "SELECT page_count * page_size as size FROM pragma_page_count(), pragma_page_size()",
        [],
        |row| row.get(0),
    ).unwrap_or(0);
    
    let database_size_mb = database_size_bytes as f64 / (1024.0 * 1024.0);
    
    Ok(DatabaseStats {
        total_projects,
        total_files,
        total_size_bytes,
        cached_analyses,
        total_tasks,
        total_summaries,
        database_size_bytes,
        database_size_mb,
    })
}

#[tauri::command]
pub async fn vacuum_database(
    db_pool: State<'_, Arc<DbPool>>,
) -> Result<String, String> {
    let conn = db_pool.get().map_err(|e| e.to_string())?;
    
    // Get size before vacuum
    let size_before: i64 = conn.query_row(
        "SELECT page_count * page_size FROM pragma_page_count(), pragma_page_size()",
        [],
        |row| row.get(0),
    ).unwrap_or(0);
    
    // Run VACUUM
    conn.execute("VACUUM", []).map_err(|e| e.to_string())?;
    
    // Get size after vacuum
    let size_after: i64 = conn.query_row(
        "SELECT page_count * page_size FROM pragma_page_count(), pragma_page_size()",
        [],
        |row| row.get(0),
    ).unwrap_or(0);
    
    let reclaimed = size_before - size_after;
    let reclaimed_mb = reclaimed as f64 / (1024.0 * 1024.0);
    
    Ok(format!("Vacuum complete. Reclaimed {:.2} MB", reclaimed_mb))
}

#[tauri::command]
pub async fn clear_expired_cache(
    db_pool: State<'_, Arc<DbPool>>,
) -> Result<String, String> {
    let conn = db_pool.get().map_err(|e| e.to_string())?;
    
    let deleted = conn.execute(
        "DELETE FROM analysis_cache WHERE expires_at < CURRENT_TIMESTAMP",
        [],
    ).map_err(|e| e.to_string())?;
    
    Ok(format!("Cleared {} expired cache entries", deleted))
}

#[tauri::command]
pub async fn optimize_database(
    db_pool: State<'_, Arc<DbPool>>,
) -> Result<String, String> {
    let conn = db_pool.get().map_err(|e| e.to_string())?;
    
    // Run ANALYZE to update statistics
    conn.execute("ANALYZE", []).map_err(|e| e.to_string())?;
    
    // Clear expired cache
    let deleted: usize = conn.execute(
        "DELETE FROM analysis_cache WHERE expires_at < CURRENT_TIMESTAMP",
        [],
    ).map_err(|e| e.to_string())?;
    
    Ok(format!("Optimization complete. Cleared {} expired cache entries", deleted))
}

#[tauri::command]
pub async fn save_theme_preference(
    db_pool: State<'_, Arc<DbPool>>,
    theme: String,
) -> Result<(), String> {
    let conn = db_pool.get().map_err(|e| e.to_string())?;
    db::save_setting(&conn, "theme_preference", &theme)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn load_theme_preference(
    db_pool: State<'_, Arc<DbPool>>,
) -> Result<Option<String>, String> {
    let conn = db_pool.get().map_err(|e| e.to_string())?;
    db::load_setting(&conn, "theme_preference")
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn save_settings(
    db_pool: State<'_, Arc<DbPool>>,
    settings: Settings,
) -> Result<(), String> {
    let conn = db_pool.get().map_err(|e| e.to_string())?;
    let json = serde_json::to_string(&settings).map_err(|e| e.to_string())?;
    db::save_setting(&conn, "api_settings", &json)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn load_settings(
    db_pool: State<'_, Arc<DbPool>>,
) -> Result<Settings, String> {
    let conn = db_pool.get().map_err(|e| e.to_string())?;
    
    if let Some(json) = db::load_setting(&conn, "api_settings").map_err(|e| e.to_string())? {
        // Backward-compatible: provide defaults for any missing fields
        let settings: Settings = serde_json::from_str(&json).map_err(|e| e.to_string())?;
        // Fields with serde(default) are already filled; just return
        Ok(settings)
    } else {
        Ok(Settings {
            api_url: "http://localhost:11434/v1/chat/completions".to_string(),
            model: "llama2".to_string(),
            api_key: "".to_string(),
            temperature_ideas: default_temperature_ideas(),
            frequency_penalty_ideas: default_frequency_penalty_ideas(),
            presence_penalty_ideas: default_presence_penalty_ideas(),
            max_tokens_ideas: default_max_tokens_ideas(),
            temperature_summary: default_temperature_summary(),
            presence_penalty_summary: default_presence_penalty_summary(),
            max_tokens_summary: default_max_tokens_summary(),
            use_stop_ideas: default_use_stop_ideas(),
        })
    }
}

#[tauri::command]
pub async fn save_project_summary(
    db_pool: State<'_, Arc<DbPool>>,
    summary: ProjectSummary,
) -> Result<(), String> {
    let conn = db_pool.get().map_err(|e| e.to_string())?;
    
    // Get or create project
    let project_path = summary.project_path.clone();
    let project = db::get_project_by_path(&conn, &project_path)
        .map_err(|e| e.to_string())?
        .ok_or("Project not found")?;
    
    db::save_summary(&conn, project.id, &summary)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn load_project_summary(
    db_pool: State<'_, Arc<DbPool>>,
    project_path: String,
) -> Result<Option<ProjectSummary>, String> {
    let conn = db_pool.get().map_err(|e| e.to_string())?;
    
    if let Some(project) = db::get_project_by_path(&conn, &project_path).map_err(|e| e.to_string())? {
        db::load_summary(&conn, project.id, &project_path)
            .map_err(|e| e.to_string())
    } else {
        Ok(None)
    }
}

#[tauri::command]
pub async fn save_root_folder(
    db_pool: State<'_, Arc<DbPool>>,
    root_folder: String,
) -> Result<(), String> {
    let conn = db_pool.get().map_err(|e| e.to_string())?;
    db::save_setting(&conn, "root_folder", &root_folder)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn load_root_folder(
    db_pool: State<'_, Arc<DbPool>>,
) -> Result<Option<String>, String> {
    let conn = db_pool.get().map_err(|e| e.to_string())?;
    let root_folder = db::load_setting(&conn, "root_folder")
        .map_err(|e| e.to_string())?;
    
    // Verify the folder still exists
    if let Some(ref path) = root_folder {
        if std::path::Path::new(path).exists() {
            Ok(Some(path.clone()))
        } else {
            Ok(None)
        }
    } else {
        Ok(None)
    }
}

#[tauri::command]
pub async fn save_task_list(
    db_pool: State<'_, Arc<DbPool>>,
    task_list: TaskList,
) -> Result<(), String> {
    let conn = db_pool.get().map_err(|e| e.to_string())?;
    
    // Get or create project
    let project = db::get_project_by_path(&conn, &task_list.project_path)
        .map_err(|e| e.to_string())?
        .ok_or("Project not found")?;
    
    db::save_task_list(&conn, project.id, &task_list.tasks)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn load_task_list(
    db_pool: State<'_, Arc<DbPool>>,
    project_path: String,
) -> Result<Option<TaskList>, String> {
    let conn = db_pool.get().map_err(|e| e.to_string())?;
    
    if let Some(project) = db::get_project_by_path(&conn, &project_path).map_err(|e| e.to_string())? {
        db::load_task_list(&conn, project.id, &project_path)
            .map_err(|e| e.to_string())
    } else {
        Ok(None)
    }
}

#[tauri::command]
pub async fn save_favorite_projects(
    db_pool: State<'_, Arc<DbPool>>,
    favorites: Vec<String>,
) -> Result<(), String> {
    let conn = db_pool.get().map_err(|e| e.to_string())?;
    
    // First, unfavorite all projects
    conn.execute("UPDATE projects SET is_favorite = FALSE", [])
        .map_err(|e| e.to_string())?;
    
    // Then favorite the specified ones
    for path in favorites {
        db::toggle_favorite(&conn, &path, true)
            .map_err(|e| e.to_string())?;
    }
    
    Ok(())
}

#[tauri::command]
pub async fn load_favorite_projects(
    db_pool: State<'_, Arc<DbPool>>,
) -> Result<Vec<String>, String> {
    let conn = db_pool.get().map_err(|e| e.to_string())?;
    db::get_favorites(&conn)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn clear_all_data(
    db_pool: State<'_, Arc<DbPool>>,
) -> Result<(), String> {
    let conn = db_pool.get().map_err(|e| e.to_string())?;
    
    // Clear all data but keep schema
    conn.execute("DELETE FROM tasks", []).map_err(|e| e.to_string())?;
    conn.execute("DELETE FROM summaries", []).map_err(|e| e.to_string())?;
    conn.execute("DELETE FROM analysis_cache", []).map_err(|e| e.to_string())?;
    conn.execute("DELETE FROM files", []).map_err(|e| e.to_string())?;
    conn.execute("DELETE FROM git_info", []).map_err(|e| e.to_string())?;
    conn.execute("DELETE FROM projects", []).map_err(|e| e.to_string())?;
    conn.execute("DELETE FROM settings", []).map_err(|e| e.to_string())?;
    
    // Run vacuum to reclaim space
    conn.execute("VACUUM", []).map_err(|e| e.to_string())?;
    
    Ok(())
}
