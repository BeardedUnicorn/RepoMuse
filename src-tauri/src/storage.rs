use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tauri::State;
use crate::db::{self, DbPool};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Settings {
    pub api_url: String,
    pub model: String,
    pub api_key: String,
}

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
        serde_json::from_str(&json).map_err(|e| e.to_string())
    } else {
        Ok(Settings {
            api_url: "http://localhost:11434/v1/chat/completions".to_string(),
            model: "llama2".to_string(),
            api_key: "".to_string(),
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