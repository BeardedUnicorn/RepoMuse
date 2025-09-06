use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Settings {
    pub api_url: String,
    pub model: String,
    pub api_key: String,
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

