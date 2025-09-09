use serde::{Deserialize, Serialize};
use std::fs;

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

#[tauri::command]
pub async fn save_theme_preference(theme: String) -> Result<(), String> {
    let dir = app_dir()?;
    if !dir.exists() { 
        fs::create_dir_all(&dir).map_err(|e| e.to_string())?; 
    }
    let path = dir.join("theme.json");
    let preference = ThemePreference {
        theme,
        last_updated: chrono::Utc::now().to_rfc3339(),
    };
    let json = serde_json::to_string_pretty(&preference).map_err(|e| e.to_string())?;
    fs::write(path, json).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn load_theme_preference() -> Result<Option<String>, String> {
    let dir = app_dir()?;
    let path = dir.join("theme.json");
    if !path.exists() { 
        return Ok(None); 
    }
    let s = fs::read_to_string(path).map_err(|e| e.to_string())?;
    let preference: ThemePreference = serde_json::from_str(&s).map_err(|e| e.to_string())?;
    Ok(Some(preference.theme))
}

fn app_dir() -> Result<std::path::PathBuf, String> {
    dirs::data_local_dir()
        .ok_or("Failed to get app data directory".to_string())
        .map(|d| d.join("repomuse"))
}

#[tauri::command]
pub async fn save_settings(settings: Settings) -> Result<(), String> {
    let dir = app_dir()?;
    if !dir.exists() { fs::create_dir_all(&dir).map_err(|e| e.to_string())?; }
    let path = dir.join("settings.json");
    let json = serde_json::to_string_pretty(&settings).map_err(|e| e.to_string())?;
    fs::write(path, json).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn load_settings() -> Result<Settings, String> {
    let dir = app_dir()?;
    let path = dir.join("settings.json");
    if !path.exists() {
        return Ok(Settings {
            api_url: "http://localhost:11434/v1/chat/completions".to_string(),
            model: "llama2".to_string(),
            api_key: "".to_string(),
        });
    }
    let s = fs::read_to_string(path).map_err(|e| e.to_string())?;
    serde_json::from_str(&s).map_err(|e| e.to_string())
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
pub async fn save_project_summary(summary: ProjectSummary) -> Result<(), String> {
    let dir = app_dir()?.join("summaries");
    if !dir.exists() { fs::create_dir_all(&dir).map_err(|e| e.to_string())?; }
    let filename = summary
        .project_path
        .replace("/", "_")
        .replace("\\", "_")
        .replace(":", "");
    let path = dir.join(format!("{}.json", filename));
    let json = serde_json::to_string_pretty(&summary).map_err(|e| e.to_string())?;
    fs::write(path, json).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn load_project_summary(project_path: String) -> Result<Option<ProjectSummary>, String> {
    let dir = app_dir()?.join("summaries");
    let filename = project_path.replace("/", "_").replace("\\", "_").replace(":", "");
    let path = dir.join(format!("{}.json", filename));
    if !path.exists() { return Ok(None); }
    let s = fs::read_to_string(path).map_err(|e| e.to_string())?;
    let summary: ProjectSummary = serde_json::from_str(&s).map_err(|e| e.to_string())?;
    Ok(Some(summary))
}

#[tauri::command]
pub async fn save_root_folder(root_folder: String) -> Result<(), String> {
    let dir = app_dir()?;
    if !dir.exists() { fs::create_dir_all(&dir).map_err(|e| e.to_string())?; }
    let path = dir.join("root_folder.txt");
    fs::write(path, root_folder).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn load_root_folder() -> Result<Option<String>, String> {
    let dir = app_dir()?;
    let path = dir.join("root_folder.txt");
    if !path.exists() { return Ok(None); }
    let s = fs::read_to_string(path).map_err(|e| e.to_string())?;
    if std::path::Path::new(&s).exists() { Ok(Some(s)) } else { Ok(None) }
}

// Task list functions
#[tauri::command]
pub async fn save_task_list(task_list: TaskList) -> Result<(), String> {
    let dir = app_dir()?.join("tasks");
    if !dir.exists() { 
        fs::create_dir_all(&dir).map_err(|e| e.to_string())?; 
    }
    
    let filename = task_list
        .project_path
        .replace("/", "_")
        .replace("\\", "_")
        .replace(":", "");
    let path = dir.join(format!("{}.json", filename));
    
    let json = serde_json::to_string_pretty(&task_list).map_err(|e| e.to_string())?;
    fs::write(path, json).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn load_task_list(project_path: String) -> Result<Option<TaskList>, String> {
    let dir = app_dir()?.join("tasks");
    let filename = project_path.replace("/", "_").replace("\\", "_").replace(":", "");
    let path = dir.join(format!("{}.json", filename));
    
    if !path.exists() { 
        return Ok(None); 
    }
    
    let s = fs::read_to_string(path).map_err(|e| e.to_string())?;
    let task_list: TaskList = serde_json::from_str(&s).map_err(|e| e.to_string())?;
    Ok(Some(task_list))
}

// Favorite projects functions
#[tauri::command]
pub async fn save_favorite_projects(favorites: Vec<String>) -> Result<(), String> {
    println!("[Rust] Attempting to save favorites: {:?}", favorites);
    
    let dir = app_dir()?;
    println!("[Rust] App directory: {:?}", dir);
    
    if !dir.exists() { 
        println!("[Rust] Creating app directory...");
        fs::create_dir_all(&dir).map_err(|e| format!("Failed to create directory: {}", e))?; 
    }
    
    let path = dir.join("favorites.json");
    println!("[Rust] Full path for favorites: {:?}", path);
    
    let favorite_data = FavoriteProjects {
        favorites,
        last_updated: chrono::Utc::now().to_rfc3339(),
    };
    
    let json = serde_json::to_string_pretty(&favorite_data)
        .map_err(|e| format!("Failed to serialize favorites: {}", e))?;
    
    fs::write(&path, &json)
        .map_err(|e| format!("Failed to write favorites file at {:?}: {}", path, e))?;
    
    println!("[Rust] Favorites saved successfully to {:?}", path);
    Ok(())
}

#[tauri::command]
pub async fn load_favorite_projects() -> Result<Vec<String>, String> {
    println!("[Rust] Attempting to load favorites...");
    
    let dir = app_dir()?;
    let path = dir.join("favorites.json");
    
    println!("[Rust] Looking for favorites at: {:?}", path);
    
    if !path.exists() { 
        println!("[Rust] No favorites file found, returning empty list");
        return Ok(Vec::new()); 
    }
    
    let s = fs::read_to_string(&path)
        .map_err(|e| format!("Failed to read favorites file: {}", e))?;
    
    let favorite_data: FavoriteProjects = serde_json::from_str(&s)
        .map_err(|e| format!("Failed to parse favorites JSON: {}", e))?;
    
    println!("[Rust] Loaded {} favorites", favorite_data.favorites.len());
    Ok(favorite_data.favorites)
}