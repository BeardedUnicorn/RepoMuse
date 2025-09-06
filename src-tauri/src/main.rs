#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::sync::{Arc, Mutex};
use walkdir::WalkDir;
use rayon::prelude::*;
use std::time::{SystemTime, Duration};

// Cache structure for file counts
#[derive(Debug, Serialize, Deserialize, Clone)]
struct FileCountCache {
    path: String,
    count: usize,
    last_modified: u64,
    cached_at: u64,
}

#[derive(Debug, Serialize, Deserialize)]
struct Settings {
    api_url: String,
    model: String,
    api_key: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct ModelInfo {
    id: String,
    name: Option<String>,
    description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ModelsResponse {
    data: Vec<ModelInfo>,
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

#[derive(Debug, Serialize, Deserialize, Clone)]
struct ProjectSummary {
    project_path: String,
    summary: String,
    generated_at: String,
    technologies: Vec<String>,
    key_features: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct SummaryRequest {
    analysis: RepoAnalysis,
    settings: Settings,
    project_path: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct ProjectDirectory {
    name: String,
    path: String,
    is_git_repo: bool,
    file_count: usize,
    description: Option<String>,
    is_counting: bool, // Flag to indicate if counting is in progress
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

// Check if a directory looks like a project
fn is_project_directory(path: &Path) -> bool {
    let project_indicators = vec![
        "package.json",     // Node.js
        "Cargo.toml",       // Rust
        "pom.xml",          // Maven Java
        "build.gradle",     // Gradle
        "requirements.txt", // Python
        "Gemfile",          // Ruby
        "go.mod",           // Go
        "composer.json",    // PHP
        "project.clj",      // Clojure
        "mix.exs",          // Elixir
        ".csproj",          // C#
        "pubspec.yaml",     // Dart/Flutter
        "CMakeLists.txt",   // CMake
        "Makefile",         // Make
        "README.md",        // General project
        "README.txt",       // General project
    ];

    for indicator in project_indicators {
        if indicator.ends_with(".csproj") {
            // Special handling for .csproj files
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

// Get a brief description of the project based on its contents
fn get_project_description(path: &Path) -> Option<String> {
    // Try to read package.json for Node.js projects
    if let Ok(package_json) = fs::read_to_string(path.join("package.json")) {
        if let Ok(json) = serde_json::from_str::<serde_json::Value>(&package_json) {
            if let Some(description) = json["description"].as_str() {
                return Some(description.to_string());
            }
        }
    }

    // Try to read Cargo.toml for Rust projects
    if let Ok(cargo_toml) = fs::read_to_string(path.join("Cargo.toml")) {
        if let Some(desc_line) = cargo_toml.lines().find(|line| line.starts_with("description")) {
            if let Some(desc) = desc_line.split('=').nth(1) {
                return Some(desc.trim().trim_matches('"').to_string());
            }
        }
    }

    // Try to read first line of README
    for readme_name in &["README.md", "README.txt", "readme.md", "readme.txt"] {
        if let Ok(readme) = fs::read_to_string(path.join(readme_name)) {
            let first_line = readme.lines().next().unwrap_or("").trim();
            if !first_line.is_empty() && first_line.len() < 200 {
                // Remove markdown header syntax
                let cleaned = first_line.trim_start_matches('#').trim();
                if !cleaned.is_empty() {
                    return Some(cleaned.to_string());
                }
            }
        }
    }

    None
}

// Get directory last modified time
fn get_dir_modified_time(path: &Path) -> u64 {
    if let Ok(metadata) = fs::metadata(path) {
        if let Ok(modified) = metadata.modified() {
            return modified.duration_since(SystemTime::UNIX_EPOCH)
                .unwrap_or(Duration::from_secs(0))
                .as_secs();
        }
    }
    0
}

// Load file count cache
fn load_file_count_cache() -> HashMap<String, FileCountCache> {
    let app_dir = match dirs::data_local_dir() {
        Some(dir) => dir.join("repomuse"),
        None => return HashMap::new(),
    };
    
    let cache_path = app_dir.join("file_count_cache.json");
    
    if let Ok(cache_content) = fs::read_to_string(cache_path) {
        if let Ok(cache) = serde_json::from_str(&cache_content) {
            return cache;
        }
    }
    
    HashMap::new()
}

// Save file count cache
fn save_file_count_cache(cache: &HashMap<String, FileCountCache>) {
    let app_dir = match dirs::data_local_dir() {
        Some(dir) => dir.join("repomuse"),
        None => return,
    };
    
    if !app_dir.exists() {
        let _ = fs::create_dir_all(&app_dir);
    }
    
    let cache_path = app_dir.join("file_count_cache.json");
    if let Ok(cache_json) = serde_json::to_string_pretty(&cache) {
        let _ = fs::write(cache_path, cache_json);
    }
}

// Quick file count estimation (faster but less accurate)
fn estimate_file_count(path: &Path) -> usize {
    let mut count = 0;
    let mut depth = 0;
    const MAX_DEPTH: usize = 3; // Only go 3 levels deep for estimation
    const SAMPLE_FACTOR: usize = 10; // Sample every 10th directory
    
    for entry in WalkDir::new(path)
        .max_depth(MAX_DEPTH)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if entry.path().is_file() {
            let path_str = entry.path().to_string_lossy();
            if should_analyze_file(&path_str) {
                count += 1;
            }
        }
        
        // Sample deeper directories
        if entry.path().is_dir() && entry.depth() == MAX_DEPTH {
            depth += 1;
            if depth % SAMPLE_FACTOR == 0 {
                // Estimate based on sample
                count += estimate_deep_files(entry.path()) * SAMPLE_FACTOR;
            }
        }
    }
    
    count
}

// Estimate files in deep directories
fn estimate_deep_files(path: &Path) -> usize {
    let mut count = 0;
    let mut checked = 0;
    const MAX_CHECK: usize = 50; // Check up to 50 files
    
    for entry in WalkDir::new(path)
        .into_iter()
        .filter_map(|e| e.ok())
        .take(MAX_CHECK)
    {
        checked += 1;
        if entry.path().is_file() {
            let path_str = entry.path().to_string_lossy();
            if should_analyze_file(&path_str) {
                count += 1;
            }
        }
    }
    
    // Extrapolate if we hit the limit
    if checked >= MAX_CHECK {
        count * 2
    } else {
        count
    }
}

// Count files in parallel for a single project
fn count_project_files(path: &Path) -> usize {
    // Use parallel iterator for file counting
    WalkDir::new(path)
        .into_iter()
        .par_bridge()
        .filter_map(|e| e.ok())
        .filter(|entry| {
            entry.path().is_file() && 
            should_analyze_file(&entry.path().to_string_lossy())
        })
        .count()
}

// Process a single project directory (fast version)
fn process_project_directory_fast(path: std::path::PathBuf, cache: &HashMap<String, FileCountCache>) -> Option<ProjectDirectory> {
    let dir_name = path.file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("Unknown")
        .to_string();

    // Skip hidden directories and common non-project directories
    if dir_name.starts_with('.') || 
       ["node_modules", "target", "build", "dist", "vendor", "__pycache__"].contains(&dir_name.as_str()) {
        return None;
    }

    // Check if this looks like a project directory
    if is_project_directory(&path) {
        let is_git_repo = path.join(".git").exists();
        let description = get_project_description(&path);
        let path_str = path.to_string_lossy().to_string();
        
        // Try to get cached count
        let (file_count, is_counting) = if let Some(cached) = cache.get(&path_str) {
            let dir_modified = get_dir_modified_time(&path);
            // Use cache if directory hasn't been modified and cache is recent (within 24 hours)
            let now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)
                .unwrap_or(Duration::from_secs(0))
                .as_secs();
            
            if cached.last_modified >= dir_modified && (now - cached.cached_at) < 86400 {
                (cached.count, false)
            } else {
                // Cache is stale, use estimation and mark for counting
                (estimate_file_count(&path), true)
            }
        } else {
            // No cache, use estimation and mark for counting
            (estimate_file_count(&path), true)
        };

        Some(ProjectDirectory {
            name: dir_name,
            path: path_str,
            is_git_repo,
            file_count,
            description,
            is_counting,
        })
    } else {
        None
    }
}

#[tauri::command]
async fn list_project_directories(root_path: String) -> Result<Vec<ProjectDirectory>, String> {
    let root = Path::new(&root_path);
    if !root.exists() || !root.is_dir() {
        return Err("Invalid root directory".to_string());
    }

    // Load cache
    let cache = load_file_count_cache();

    // Read immediate subdirectories
    let entries: Vec<std::path::PathBuf> = fs::read_dir(root)
        .map_err(|e| format!("Failed to read directory: {}", e))?
        .filter_map(|e| e.ok())
        .filter(|entry| entry.path().is_dir())
        .map(|entry| entry.path())
        .collect();

    // Process all project directories in parallel (fast estimation)
    let mut projects: Vec<ProjectDirectory> = entries
        .par_iter()
        .filter_map(|path| process_project_directory_fast(path.clone(), &cache))
        .collect();

    // Sort by name (case-insensitive)
    projects.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));

    Ok(projects)
}

// Update file count for a specific project (called asynchronously from frontend)
#[tauri::command]
async fn update_project_file_count(project_path: String) -> Result<usize, String> {
    let path = Path::new(&project_path);
    if !path.exists() || !path.is_dir() {
        return Err("Invalid project path".to_string());
    }

    // Count files
    let count = count_project_files(path);
    let last_modified = get_dir_modified_time(path);
    let cached_at = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)
        .unwrap_or(Duration::from_secs(0))
        .as_secs();
    
    // Update cache
    let mut cache = load_file_count_cache();
    cache.insert(project_path.clone(), FileCountCache {
        path: project_path,
        count,
        last_modified,
        cached_at,
    });
    save_file_count_cache(&cache);
    
    Ok(count)
}

// Clear file count cache (utility function)
#[tauri::command]
async fn clear_file_count_cache() -> Result<(), String> {
    let app_dir = dirs::data_local_dir()
        .ok_or("Failed to get app data directory")?
        .join("repomuse");
    
    let cache_path = app_dir.join("file_count_cache.json");
    
    if cache_path.exists() {
        fs::remove_file(cache_path)
            .map_err(|e| format!("Failed to clear cache: {}", e))?;
    }
    
    Ok(())
}

#[tauri::command]
async fn analyze_repository(folder_path: String) -> Result<RepoAnalysis, String> {
    let structure: Arc<Mutex<HashMap<String, Vec<String>>>> = Arc::new(Mutex::new(HashMap::new()));
    let technologies: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
    
    let total_files = Arc::new(Mutex::new(0usize));
    let total_lines = Arc::new(Mutex::new(0usize));
    
    // Collect all valid file paths first
    let valid_files: Vec<_> = WalkDir::new(&folder_path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|entry| entry.path().is_file())
        .filter(|entry| should_analyze_file(&entry.path().to_string_lossy()))
        .map(|entry| entry.path().to_owned())
        .collect();
    
    // Process files in parallel
    let file_infos: Vec<FileInfo> = valid_files
        .par_iter()
        .filter_map(|path| {
            let path_str = path.to_string_lossy().to_string();
            
            // Read file content
            match fs::read_to_string(path) {
                Ok(content) => {
                    let lines = content.lines().count();
                    
                    // Update metrics atomically
                    {
                        let mut total_files = total_files.lock().unwrap();
                        *total_files += 1;
                        let mut total_lines = total_lines.lock().unwrap();
                        *total_lines += lines;
                    }
                    
                    let language = get_language_from_extension(&path_str);
                    
                    // Update technologies atomically
                    {
                        let mut techs = technologies.lock().unwrap();
                        if !techs.contains(&language) && language != "Unknown" {
                            techs.push(language.clone());
                        }
                    }
                    
                    // Only include smaller files in detailed analysis
                    if content.len() < 100000 { // 100KB limit
                        let file_info = FileInfo {
                            path: path_str.clone(),
                            content: if content.len() > 5000 {
                                format!("{}...(truncated)", &content[..5000])
                            } else {
                                content
                            },
                            language,
                            size: path.metadata().map(|m| m.len()).unwrap_or(0),
                        };
                        
                        // Build directory structure atomically
                        if let Some(parent) = path.parent() {
                            let parent_str = parent.to_string_lossy().to_string();
                            let mut structure = structure.lock().unwrap();
                            structure.entry(parent_str).or_insert_with(Vec::new)
                                .push(path.file_name().unwrap().to_string_lossy().to_string());
                        }
                        
                        Some(file_info)
                    } else {
                        None
                    }
                }
                Err(_) => None,
            }
        })
        .collect();
    
    // Extract final values from Arc<Mutex<>>
    let final_metrics = {
        let mut m = HashMap::new();
        m.insert("total_files".to_string(), *total_files.lock().unwrap() as i32);
        m.insert("total_lines".to_string(), *total_lines.lock().unwrap() as i32);
        m.insert("analyzed_files".to_string(), file_infos.len() as i32);
        m
    };
    
    let final_technologies = technologies.lock().unwrap().clone();
    let final_structure = structure.lock().unwrap().clone();
    
    Ok(RepoAnalysis {
        files: file_infos,
        structure: final_structure,
        technologies: final_technologies,
        metrics: final_metrics,
    })
}

#[tauri::command]
async fn load_models(api_url: String, api_key: String) -> Result<Vec<ModelInfo>, String> {
    let client = reqwest::Client::new();
    
    // Try different endpoints for model listing
    let model_endpoints = vec![
        format!("{}/models", api_url.replace("/chat/completions", "")),
        format!("{}/v1/models", api_url.replace("/v1/chat/completions", "").replace("/chat/completions", "")),
    ];
    
    for endpoint in model_endpoints {
        let mut headers = reqwest::header::HeaderMap::new();
        
        if !api_key.is_empty() {
            headers.insert(
                reqwest::header::AUTHORIZATION,
                format!("Bearer {}", api_key).parse().unwrap(),
            );
        }
        
        match client.get(&endpoint).headers(headers).send().await {
            Ok(response) => {
                let status = response.status();
                let response_text = match response.text().await {
                    Ok(t) => t,
                    Err(_) => String::new(),
                };

                if status.is_success() {
                    // Try OpenAI-style models response
                    if let Ok(models_response) = serde_json::from_str::<ModelsResponse>(&response_text) {
                        return Ok(models_response.data);
                    }

                    // Try alternative format (Ollama style)
                    if let Ok(models_json) = serde_json::from_str::<serde_json::Value>(&response_text) {
                        if let Some(models_array) = models_json["models"].as_array() {
                            let models: Vec<ModelInfo> = models_array
                                .iter()
                                .filter_map(|model| {
                                    if let Some(name) = model["name"].as_str() {
                                        Some(ModelInfo {
                                            id: name.to_string(),
                                            name: Some(name.to_string()),
                                            description: model["details"]["parameter_size"]
                                                .as_str()
                                                .map(|s| s.to_string()),
                                        })
                                    } else {
                                        None
                                    }
                                })
                                .collect();
                            if !models.is_empty() {
                                return Ok(models);
                            }
                        }
                    }
                }
            }
            Err(_) => continue,
        }
    }
    
    Err("Unable to load models from API. Please check your API URL and key.".to_string())
}

fn extract_thinking_and_response(content: &str) -> (Option<String>, String) {
    use regex::Regex;
    
    // Look for <think>...</think> tags
    let re = Regex::new(r"(?s)<think>(.*?)</think>(.*)").unwrap();
    
    if let Some(captures) = re.captures(content) {
        let thinking = captures.get(1).map(|m| m.as_str().trim().to_string());
        let response = captures.get(2).map(|m| m.as_str().trim().to_string()).unwrap_or_default();
        (thinking, response)
    } else {
        (None, content.to_string())
    }
}

fn parse_structured_response(content: &str) -> Vec<String> {
    let lines: Vec<&str> = content.lines().collect();
    let mut ideas = Vec::new();
    let mut current_idea = String::new();
    
    for line in lines {
        let line = line.trim();
        
        // Skip empty lines
        if line.is_empty() {
            if !current_idea.trim().is_empty() {
                ideas.push(current_idea.trim().to_string());
                current_idea.clear();
            }
            continue;
        }
        
        // Check if line starts with a number (1., 2., etc.) or bullet point
        let starts_with_number = line.chars().next().map_or(false, |c| c.is_ascii_digit());
        let is_new_idea = (starts_with_number && (line.contains('.') || line.contains(')')))
            || line.starts_with("• ")
            || line.starts_with("- ")
            || line.starts_with("* ");
        
        if is_new_idea && !current_idea.trim().is_empty() {
            ideas.push(current_idea.trim().to_string());
            current_idea.clear();
        }
        
        if is_new_idea {
            // Clean up the line by removing number/bullet prefixes
            let cleaned = line
                .trim_start_matches(char::is_numeric)
                .trim_start_matches('.')
                .trim_start_matches(')')
                .trim_start_matches("• ")
                .trim_start_matches("- ")
                .trim_start_matches("* ")
                .trim();
            current_idea.push_str(cleaned);
        } else if !current_idea.is_empty() {
            // Continue the current idea
            current_idea.push(' ');
            current_idea.push_str(line);
        } else {
            // Start a new idea if we don't have one
            current_idea.push_str(line);
        }
    }
    
    // Add the last idea if there is one
    if !current_idea.trim().is_empty() {
        ideas.push(current_idea.trim().to_string());
    }
    
    // Filter out very short ideas (likely formatting artifacts)
    ideas.into_iter()
        .filter(|idea| idea.len() > 20)
        .collect()
}

#[tauri::command]
async fn generate_ideas(request: IdeaRequest) -> Result<Vec<String>, String> {
    let client = reqwest::Client::new();
    
    // Create structured prompt for better parsing
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

IMPORTANT: Format your response as a numbered list with one idea per line. Each idea should be a complete, actionable suggestion. For example:

1. Implement automated testing suite using Jest for better code reliability
2. Add TypeScript support to improve type safety and developer experience
3. Create a comprehensive README with setup instructions and API documentation

Each idea should be on its own line and clearly numbered.",
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
    
    if !request.settings.api_key.is_empty() {
        headers.insert(
            reqwest::header::AUTHORIZATION,
            format!("Bearer {}", request.settings.api_key).parse().unwrap(),
        );
    }
    headers.insert(
        reqwest::header::CONTENT_TYPE,
        "application/json".parse().unwrap(),
    );

    let body = serde_json::json!({
        "model": request.settings.model,
        "messages": [
            {
                "role": "system",
                "content": "You are a senior software engineer and architect who provides creative, practical development ideas for code repositories. Always format your response as a clear numbered list with one idea per line."
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
                // Handle thinking models by extracting the actual response
                let (_thinking, actual_response) = extract_thinking_and_response(message);
                
                // Parse the structured response into individual ideas
                let ideas = parse_structured_response(&actual_response);
                
                if ideas.is_empty() {
                    // Fallback to simple line splitting if structured parsing fails
                    let fallback_ideas: Vec<String> = actual_response
                        .lines()
                        .filter(|line| !line.trim().is_empty() && line.len() > 20)
                        .map(|line| line.trim().to_string())
                        .collect();
                    
                    return Ok(fallback_ideas);
                }
                
                return Ok(ideas);
            }
        }
    }

    Err("Failed to parse AI response".to_string())
}

#[tauri::command]
async fn save_settings(settings: Settings) -> Result<(), String> {
    let app_dir = dirs::data_local_dir()
        .ok_or("Failed to get app data directory")?
        .join("repomuse");
    
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
    let app_dir = dirs::data_local_dir()
        .ok_or("Failed to get app data directory")?
        .join("repomuse");
    
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

#[tauri::command]
async fn generate_project_summary(request: SummaryRequest) -> Result<ProjectSummary, String> {
    let client = reqwest::Client::new();
    
    // Get more detailed file previews for better context
    let file_previews: Vec<String> = request.analysis.files
        .iter()
        .take(15)
        .map(|f| {
            let preview = if f.content.len() > 300 {
                format!("{}...", &f.content[..300])
            } else {
                f.content.clone()
            };
            format!("File: {} ({})\nContent snippet:\n{}\n", f.path, f.language, preview)
        })
        .collect();
    
    let prompt = format!(
        "Analyze this code repository and create a clear, focused summary that explains what this application/project does.

Repository Analysis:
- Technologies: {}
- Total Files: {}
- Total Lines: {}
- Directory Structure: {} directories analyzed

File Previews:
{}

Please provide a summary that focuses on:

1. **What this application/project does** - Start with a clear explanation of the project's purpose and main functionality. What problem does it solve? What does it enable users to do?

2. **Core Features** - List the main features and capabilities the application provides to end users.

3. **Technical Implementation** - Briefly describe the key technologies, frameworks, and architectural patterns used to build this.

4. **Project Type** - Is this a web app, desktop application, library, API, CLI tool, mobile app, or something else?

Write the summary in clear, accessible language that explains the project's purpose and value. Focus on WHAT the application does from a user/business perspective before diving into HOW it's built.

Format your response in clear paragraphs with a logical flow from purpose to implementation.",
        request.analysis.technologies.join(", "),
        request.analysis.metrics.get("total_files").unwrap_or(&0),
        request.analysis.metrics.get("total_lines").unwrap_or(&0),
        request.analysis.structure.len(),
        file_previews.join("\n---\n")
    );

    let mut headers = reqwest::header::HeaderMap::new();
    
    if !request.settings.api_key.is_empty() {
        headers.insert(
            reqwest::header::AUTHORIZATION,
            format!("Bearer {}", request.settings.api_key).parse().unwrap(),
        );
    }
    headers.insert(
        reqwest::header::CONTENT_TYPE,
        "application/json".parse().unwrap(),
    );

    let body = serde_json::json!({
        "model": request.settings.model,
        "messages": [
            {
                "role": "system",
                "content": "You are a technical documentation specialist who creates clear, concise project summaries."
            },
            {
                "role": "user",
                "content": prompt
            }
        ],
        "max_tokens": 1000,
        "temperature": 0.7
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
                let (_thinking, summary_text) = extract_thinking_and_response(message);
                
                // Extract key features from the summary
                let key_features = extract_key_features(&summary_text);

                let summary = ProjectSummary {
                    project_path: request.project_path,
                    summary: summary_text,
                    generated_at: chrono::Utc::now().to_rfc3339(),
                    technologies: request.analysis.technologies.clone(),
                    key_features,
                };
                
                return Ok(summary);
            }
        }
    }

    Err("Failed to generate summary".to_string())
}

fn extract_key_features(summary: &str) -> Vec<String> {
    let mut features = Vec::new();
    let lines: Vec<&str> = summary.lines().collect();
    
    for line in lines {
        let trimmed = line.trim();
        if trimmed.starts_with("-") || trimmed.starts_with("•") {
            let feature = trimmed
                .trim_start_matches('-')
                .trim_start_matches('•')
                .trim()
                .to_string();
            if !feature.is_empty() && feature.len() < 200 {
                features.push(feature);
            }
        }
    }
    
    features
}

#[tauri::command]
async fn save_project_summary(summary: ProjectSummary) -> Result<(), String> {
    let app_dir = dirs::data_local_dir()
        .ok_or("Failed to get app data directory")?
        .join("repomuse")
        .join("summaries");
    
    if !app_dir.exists() {
        fs::create_dir_all(&app_dir).map_err(|e| format!("Failed to create summaries directory: {}", e))?;
    }
    
    // Create a safe filename from the project path
    let filename = summary.project_path
        .replace("/", "_")
        .replace("\\", "_")
        .replace(":", "");
    let summary_path = app_dir.join(format!("{}.json", filename));
    
    let summary_json = serde_json::to_string_pretty(&summary)
        .map_err(|e| format!("Failed to serialize summary: {}", e))?;
    
    fs::write(summary_path, summary_json)
        .map_err(|e| format!("Failed to save summary: {}", e))?;
    
    Ok(())
}

#[tauri::command]
async fn load_project_summary(project_path: String) -> Result<Option<ProjectSummary>, String> {
    let app_dir = dirs::data_local_dir()
        .ok_or("Failed to get app data directory")?
        .join("repomuse")
        .join("summaries");
    
    // Create a safe filename from the project path
    let filename = project_path
        .replace("/", "_")
        .replace("\\", "_")
        .replace(":", "");
    let summary_path = app_dir.join(format!("{}.json", filename));
    
    if !summary_path.exists() {
        return Ok(None);
    }
    
    let summary_content = fs::read_to_string(summary_path)
        .map_err(|e| format!("Failed to read summary: {}", e))?;
    
    let summary: ProjectSummary = serde_json::from_str(&summary_content)
        .map_err(|e| format!("Failed to parse summary: {}", e))?;
    
    Ok(Some(summary))
}

#[tauri::command]
async fn save_root_folder(root_folder: String) -> Result<(), String> {
    let app_dir = dirs::data_local_dir()
        .ok_or("Failed to get app data directory")?
        .join("repomuse");
    
    if !app_dir.exists() {
        fs::create_dir_all(&app_dir).map_err(|e| format!("Failed to create app directory: {}", e))?;
    }
    
    let root_path = app_dir.join("root_folder.txt");
    fs::write(root_path, root_folder)
        .map_err(|e| format!("Failed to save root folder: {}", e))?;
    
    Ok(())
}

#[tauri::command]
async fn load_root_folder() -> Result<Option<String>, String> {
    let app_dir = dirs::data_local_dir()
        .ok_or("Failed to get app data directory")?
        .join("repomuse");
    
    let root_path = app_dir.join("root_folder.txt");
    
    if !root_path.exists() {
        return Ok(None);
    }
    
    let root_folder = fs::read_to_string(root_path)
        .map_err(|e| format!("Failed to read root folder: {}", e))?;
    
    // Verify the folder still exists
    if Path::new(&root_folder).exists() {
        Ok(Some(root_folder))
    } else {
        Ok(None)
    }
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            list_project_directories,
            analyze_repository,
            generate_ideas,
            save_settings,
            load_settings,
            load_models,
            generate_project_summary,
            save_project_summary,
            load_project_summary,
            save_root_folder,
            load_root_folder,
            update_project_file_count,
            clear_file_count_cache
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}