#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use tauri::State;
use walkdir::WalkDir;

#[derive(Debug, Serialize, Deserialize)]
struct Settings {
    api_url: String,
    model: String,
    api_key: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct FileInfo {
    path: String,
    content: String,
    language: String,
    size: u64,
}

#[derive(Debug, Serialize, Deserialize)]
struct RepoAnalysis {
    files: Vec<FileInfo>,
    structure: HashMap<String, Vec<String>>,
    technologies: Vec<String>,
    metrics: HashMap<String, i32>,
}

#[derive(Debug, Serialize, Deserialize)]
struct IdeaRequest {
    analysis: RepoAnalysis,
    settings: Settings,
}

#[derive(Debug, Serialize, Deserialize)]
struct ApiResponse {
    ideas: Vec<String>,
}

// Get language from file extension
fn get_language_from_extension(path: &str) -> String {
    match Path::new(path).extension().and_then(|ext| ext.to_str()) {
        Some("rs") => "Rust".to_string(),
        Some("js") | Some("jsx") => "JavaScript".to_string(),
        Some("ts") | Some("tsx") => "TypeScript".to_string(),
        Some("py") => "Python".to_string(),
        Some("java") => "Java".to_string(),
        Some("cpp") | Some("cc") | Some("cxx") => "C++".to_string(),
        Some("c") => "C".to_string(),
        Some("go") => "Go".to_string(),
        Some("php") => "PHP".to_string(),
        Some("rb") => "Ruby".to_string(),
        Some("cs") => "C#".to_string(),
        Some("swift") => "Swift".to_string(),
        Some("kt") => "Kotlin".to_string(),
        Some("html") => "HTML".to_string(),
        Some("css") => "CSS".to_string(),
        Some("scss") | Some("sass") => "SCSS".to_string(),
        Some("json") => "JSON".to_string(),
        Some("xml") => "XML".to_string(),
        Some("yml") | Some("yaml") => "YAML".to_string(),
        Some("toml") => "TOML".to_string(),
        Some("md") => "Markdown".to_string(),
        _ => "Unknown".to_string(),
    }
}

// Check if file should be analyzed
fn should_analyze_file(path: &str) -> bool {
    let ignore_extensions = vec!["png", "jpg", "jpeg", "gif", "svg", "ico", "woff", "woff2", "ttf", "eot", "pdf", "zip", "tar", "gz"];
    let ignore_dirs = vec!["node_modules", "target", "build", "dist", ".git", ".svn", "vendor", "__pycache__"];
    
    // Check if it's in an ignored directory
    for ignore_dir in ignore_dirs {
        if path.contains(&format!("/{}/", ignore_dir)) || path.contains(&format!("\\{}\\", ignore_dir)) {
            return false;
        }
    }
    
    // Check file extension
    if let Some(ext) = Path::new(path).extension().and_then(|ext| ext.to_str()) {
        return !ignore_extensions.contains(&ext);
    }
    
    true
}

#[tauri::command]
async fn analyze_repository(folder_path: String) -> Result<RepoAnalysis, String> {
    let mut files = Vec::new();
    let mut structure: HashMap<String, Vec<String>> = HashMap::new();
    let mut technologies = Vec::new();
    let mut metrics = HashMap::new();
    
    let mut total_files = 0;
    let mut total_lines = 0;
    
    // Walk through directory
    for entry in WalkDir::new(&folder_path).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.is_file() {
            let path_str = path.to_string_lossy().to_string();
            
            if should_analyze_file(&path_str) {
                total_files += 1;
                
                // Read file content
                match fs::read_to_string(path) {
                    Ok(content) => {
                        let lines = content.lines().count();
                        total_lines += lines;
                        
                        let language = get_language_from_extension(&path_str);
                        if !technologies.contains(&language) && language != "Unknown" {
                            technologies.push(language.clone());
                        }
                        
                        // Only include smaller files in detailed analysis
                        if content.len() < 100000 { // 100KB limit
                            files.push(FileInfo {
                                path: path_str.clone(),
                                content: if content.len() > 5000 {
                                    format!("{}...(truncated)", &content[..5000])
                                } else {
                                    content
                                },
                                language,
                                size: path.metadata().map(|m| m.len()).unwrap_or(0),
                            });
                        }
                        
                        // Build directory structure
                        if let Some(parent) = path.parent() {
                            let parent_str = parent.to_string_lossy().to_string();
                            structure.entry(parent_str).or_insert_with(Vec::new)
                                .push(path.file_name().unwrap().to_string_lossy().to_string());
                        }
                    }
                    Err(_) => continue, // Skip binary or unreadable files
                }
            }
        }
    }
    
    metrics.insert("total_files".to_string(), total_files);
    metrics.insert("total_lines".to_string(), total_lines as i32);
    metrics.insert("analyzed_files".to_string(), files.len() as i32);
    
    Ok(RepoAnalysis {
        files,
        structure,
        technologies,
        metrics,
    })
}

#[tauri::command]
async fn generate_ideas(request: IdeaRequest) -> Result<Vec<String>, String> {
    let client = reqwest::Client::new();
    
    // Create prompt based on analysis
    let prompt = format!(
        "Analyze this code repository and generate 5-10 creative, actionable development ideas. 

Repository Analysis:
- Technologies: {}
- Total Files: {}
- Total Lines: {}
- Directory Structure: {} directories analyzed

Key Files Preview:
{}

Please provide specific, actionable suggestions for:
1. Code improvements and refactoring opportunities
2. New features that would enhance the project
3. Architecture improvements
4. Developer experience enhancements
5. Performance optimizations
6. Testing strategies
7. Documentation improvements

Format each idea as a clear, actionable item.",
        request.analysis.technologies.join(", "),
        request.analysis.metrics.get("total_files").unwrap_or(&0),
        request.analysis.metrics.get("total_lines").unwrap_or(&0),
        request.analysis.structure.len(),
        request.analysis.files
            .iter()
            .take(10) // Only first 10 files
            .map(|f| format!("{} ({}): {}", f.path, f.language, 
                if f.content.len() > 200 { 
                    format!("{}...", &f.content[..200]) 
                } else { 
                    f.content.clone() 
                }))
            .collect::<Vec<_>>()
            .join("\n\n")
    );

    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        reqwest::header::AUTHORIZATION,
        format!("Bearer {}", request.settings.api_key).parse().unwrap(),
    );
    headers.insert(
        reqwest::header::CONTENT_TYPE,
        "application/json".parse().unwrap(),
    );

    let body = serde_json::json!({
        "model": request.settings.model,
        "messages": [
            {
                "role": "system",
                "content": "You are a senior software engineer and architect who provides creative, practical development ideas for code repositories."
            },
            {
                "role": "user",
                "content": prompt
            }
        ],
        "max_tokens": 2000,
        "temperature": 0.8
    });

    let response = client
        .post(&request.settings.api_url)
        .headers(headers)
        .json(&body)
        .send()
        .await
        .map_err(|e| format!("Request failed: {}", e))?;

    let response_json: serde_json::Value = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse response: {}", e))?;

    if let Some(choices) = response_json["choices"].as_array() {
        if let Some(choice) = choices.first() {
            if let Some(message) = choice["message"]["content"].as_str() {
                // Split the response into individual ideas
                let ideas: Vec<String> = message
                    .lines()
                    .filter(|line| !line.trim().is_empty())
                    .map(|line| line.trim().to_string())
                    .collect();
                
                return Ok(ideas);
            }
        }
    }

    Err("Failed to parse AI response".to_string())
}

#[tauri::command]
async fn save_settings(settings: Settings) -> Result<(), String> {
    let app_dir = tauri::api::path::app_local_data_dir(&tauri::Config::default())
        .ok_or("Failed to get app data directory")?;
    
    if !app_dir.exists() {
        fs::create_dir_all(&app_dir).map_err(|e| format!("Failed to create app directory: {}", e))?;
    }
    
    let settings_path = app_dir.join("settings.json");
    let settings_json = serde_json::to_string_pretty(&settings)
        .map_err(|e| format!("Failed to serialize settings: {}", e))?;
    
    fs::write(settings_path, settings_json)
        .map_err(|e| format!("Failed to save settings: {}", e))?;
    
    Ok(())
}

#[tauri::command]
async fn load_settings() -> Result<Settings, String> {
    let app_dir = tauri::api::path::app_local_data_dir(&tauri::Config::default())
        .ok_or("Failed to get app data directory")?;
    
    let settings_path = app_dir.join("settings.json");
    
    if !settings_path.exists() {
        // Return default settings
        return Ok(Settings {
            api_url: "http://localhost:11434/v1/chat/completions".to_string(),
            model: "llama2".to_string(),
            api_key: "".to_string(),
        });
    }
    
    let settings_content = fs::read_to_string(settings_path)
        .map_err(|e| format!("Failed to read settings: {}", e))?;
    
    let settings: Settings = serde_json::from_str(&settings_content)
        .map_err(|e| format!("Failed to parse settings: {}", e))?;
    
    Ok(settings)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            analyze_repository,
            generate_ideas,
            save_settings,
            load_settings
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}