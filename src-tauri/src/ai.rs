use crate::analysis::RepoAnalysis;
use crate::storage::{ProjectSummary, Settings};
use regex::Regex;
use reqwest::header::{HeaderMap, AUTHORIZATION, CONTENT_TYPE};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ModelInfo {
    pub id: String,
    pub name: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ModelsResponse {
    pub data: Vec<ModelInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IdeaRequest {
    pub analysis: RepoAnalysis,
    pub settings: Settings,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SummaryRequest {
    pub analysis: RepoAnalysis,
    pub settings: Settings,
    pub project_path: String,
}

fn extract_thinking_and_response(content: &str) -> (Option<String>, String) {
    let re = Regex::new(r"(?s)<think>(.*?)</think>(.*)").unwrap();
    if let Some(captures) = re.captures(content) {
        let thinking = captures.get(1).map(|m| m.as_str().trim().to_string());
        let response = captures
            .get(2)
            .map(|m| m.as_str().trim().to_string())
            .unwrap_or_default();
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
        let is_new_idea = line
            .chars()
            .next()
            .map(|c| c.is_numeric())
            .unwrap_or(false) && line.contains('.')
            || line.starts_with("• ")
            || line.starts_with("- ")
            || line.starts_with("* ");
        if is_new_idea && !current_idea.trim().is_empty() {
            ideas.push(current_idea.trim().to_string());
            current_idea.clear();
        }
        if is_new_idea {
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
            current_idea.push(' ');
            current_idea.push_str(line);
        } else {
            current_idea.push_str(line);
        }
    }
    if !current_idea.trim().is_empty() {
        ideas.push(current_idea.trim().to_string());
    }
    ideas.into_iter().filter(|idea| idea.len() > 20).collect()
}

fn extract_key_features(summary: &str) -> Vec<String> {
    let mut features = Vec::new();
    let lines: Vec<&str> = summary.lines().collect();
    for line in lines {
        let trimmed = line.trim();
        if trimmed.starts_with('-') || trimmed.starts_with('•') {
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

fn build_comprehensive_context(analysis: &RepoAnalysis) -> String {
    let mut context = String::new();
    
    // Basic metrics
    context.push_str(&format!(
        "Repository Overview:\n\
        - Total Files: {}\n\
        - Total Lines of Code: {}\n\
        - Analyzed Files: {}\n\
        - Technologies: {}\n\n",
        analysis.metrics.get("total_files").unwrap_or(&0),
        analysis.metrics.get("total_lines").unwrap_or(&0),
        analysis.metrics.get("analyzed_files").unwrap_or(&0),
        analysis.technologies.join(", ")
    ));
    
    // Detailed file analysis with more context
    context.push_str("File Analysis:\n");
    
    // Group files by type/purpose
    let mut config_files = Vec::new();
    let mut source_files = Vec::new();
    let mut test_files = Vec::new();
    let mut doc_files = Vec::new();
    let mut build_files = Vec::new();
    
    for file in &analysis.files {
        let path_lower = file.path.to_lowercase();
        
        if path_lower.contains("test") || path_lower.contains("spec") || 
           file.path.ends_with(".test.ts") || file.path.ends_with(".test.js") ||
           file.path.ends_with(".spec.ts") || file.path.ends_with(".spec.js") {
            test_files.push(file);
        } else if path_lower.contains("readme") || path_lower.contains("doc") || 
                  file.path.ends_with(".md") {
            doc_files.push(file);
        } else if path_lower.contains("config") || file.path.ends_with(".json") || 
                  file.path.ends_with(".yml") || file.path.ends_with(".yaml") ||
                  file.path.ends_with(".toml") || file.path.ends_with(".xml") {
            config_files.push(file);
        } else if file.path.contains("build") || file.path.contains("dist") ||
                  path_lower.contains("makefile") || path_lower.contains("dockerfile") {
            build_files.push(file);
        } else {
            source_files.push(file);
        }
    }
    
    // Provide detailed context about key files
    if !source_files.is_empty() {
        context.push_str("\nSource Files (showing key implementation details):\n");
        for file in source_files.iter().take(15) {
            let preview = if file.content.len() > 800 {
                format!("{}...", &file.content[..800])
            } else {
                file.content.clone()
            };
            
            // Extract important patterns from source files
            let has_exports = preview.contains("export ") || preview.contains("module.exports");
            let has_imports = preview.contains("import ") || preview.contains("require(");
            let has_classes = preview.contains("class ") || preview.contains("interface ");
            let has_functions = preview.contains("function ") || preview.contains("const ") || preview.contains("fn ");
            let has_api_calls = preview.contains("fetch(") || preview.contains("axios") || preview.contains("http");
            let has_database = preview.contains("database") || preview.contains("query") || preview.contains("SELECT");
            
            context.push_str(&format!(
                "\n{} ({}): {} chars\n",
                file.path, file.language, file.content.len()
            ));
            
            if has_exports { context.push_str("  - Exports modules/functions\n"); }
            if has_imports { context.push_str("  - Imports external dependencies\n"); }
            if has_classes { context.push_str("  - Defines classes/interfaces\n"); }
            if has_functions { context.push_str("  - Contains functions/methods\n"); }
            if has_api_calls { context.push_str("  - Makes API/HTTP calls\n"); }
            if has_database { context.push_str("  - Database operations\n"); }
            
            context.push_str(&format!("Content preview:\n{}\n", preview));
        }
    }
    
    if !config_files.is_empty() {
        context.push_str("\nConfiguration Files:\n");
        for file in config_files.iter().take(10) {
            let preview = if file.content.len() > 500 {
                format!("{}...", &file.content[..500])
            } else {
                file.content.clone()
            };
            context.push_str(&format!(
                "\n{}: {}\n",
                file.path, preview
            ));
        }
    }
    
    if !test_files.is_empty() {
        context.push_str(&format!("\nTest Coverage: {} test files found\n", test_files.len()));
        context.push_str("Test files indicate existing functionality:\n");
        for file in test_files.iter().take(5) {
            context.push_str(&format!("  - {}\n", file.path));
        }
    }
    
    if !doc_files.is_empty() {
        context.push_str("\nDocumentation:\n");
        for file in doc_files.iter().take(3) {
            let preview = if file.content.len() > 400 {
                format!("{}...", &file.content[..400])
            } else {
                file.content.clone()
            };
            context.push_str(&format!(
                "\n{}: {}\n",
                file.path, preview
            ));
        }
    }
    
    // Directory structure analysis
    context.push_str("\nProject Structure:\n");
    for (dir, files) in &analysis.structure {
        if files.len() > 0 {
            context.push_str(&format!("  {}/: {} files\n", dir, files.len()));
        }
    }
    
    context
}

#[tauri::command]
pub async fn load_models(api_url: String, api_key: String) -> Result<Vec<ModelInfo>, String> {
    let client = reqwest::Client::new();
    let model_endpoints = vec![
        format!("{}/models", api_url.replace("/chat/completions", "")),
        format!(
            "{}/v1/models",
            api_url
                .replace("/v1/chat/completions", "")
                .replace("/chat/completions", "")
        ),
    ];
    for endpoint in model_endpoints {
        let mut headers = HeaderMap::new();
        if !api_key.is_empty() {
            headers.insert(AUTHORIZATION, format!("Bearer {}", api_key).parse().unwrap());
        }
        match client.get(&endpoint).headers(headers).send().await {
            Ok(response) => {
                let status = response.status();
                let response_text = response.text().await.unwrap_or_default();
                if status.is_success() {
                    if let Ok(models_response) = serde_json::from_str::<ModelsResponse>(&response_text) {
                        return Ok(models_response.data);
                    }
                    if let Ok(models_json) = serde_json::from_str::<serde_json::Value>(&response_text) {
                        if let Some(models_array) = models_json["models"].as_array() {
                            let models: Vec<ModelInfo> = models_array
                                .iter()
                                .filter_map(|model| {
                                    if let Some(name) = model["name"].as_str() {
                                        Some(ModelInfo {
                                            id: name.to_string(),
                                            name: Some(name.to_string()),
                                            description: model["details"]["parameter_size"].as_str().map(|s| s.to_string()),
                                        })
                                    } else {
                                        None
                                    }
                                })
                                .collect();
                            if !models.is_empty() { return Ok(models); }
                        }
                    }
                }
            }
            Err(_) => continue,
        }
    }
    Err("Unable to load models from API. Please check your API URL and key.".to_string())
}

#[tauri::command]
pub async fn generate_ideas(request: IdeaRequest) -> Result<Vec<String>, String> {
    let client = reqwest::Client::new();
    let comprehensive_context = build_comprehensive_context(&request.analysis);
    
    let prompt = format!(
        "Analyze this code repository thoroughly and generate 5-10 creative, actionable development ideas.

{}

IMPORTANT INSTRUCTIONS:
1. Review the existing code files, tests, and configurations carefully
2. Identify what features and functionality are ALREADY IMPLEMENTED
3. Only suggest ideas that are NOT already present in the codebase
4. Focus on genuine improvements, new features, and unimplemented functionality
5. Avoid suggesting features that are clearly already working based on the code analysis

Categories for suggestions (only if not already implemented):
- New features that would enhance the project
- Code improvements and refactoring opportunities (for areas not already optimized)
- Architecture improvements
- Developer experience enhancements
- Performance optimizations (for specific unoptimized areas)
- Testing strategies (for untested areas)
- Documentation improvements (for undocumented features)
- Integration opportunities with external services
- User experience improvements
- Security enhancements

Format your response as a numbered list with one idea per line. Each idea should be specific and actionable.",
        comprehensive_context
    );

    let mut headers = HeaderMap::new();
    if !request.settings.api_key.is_empty() {
        headers.insert(AUTHORIZATION, format!("Bearer {}", request.settings.api_key).parse().unwrap());
    }
    headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());

    let body = serde_json::json!({
        "model": request.settings.model,
        "messages": [
            {
                "role": "system",
                "content": "You are a senior software architect who provides creative, practical development ideas for code repositories. You carefully analyze existing code to understand what's already implemented before suggesting new features. You avoid suggesting features that are clearly already present in the codebase."
            },
            { "role": "user", "content": prompt }
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
                let (_thinking, content) = extract_thinking_and_response(message);
                let ideas = parse_structured_response(&content);
                return Ok(ideas);
            }
        }
    }
    Err("Failed to generate ideas".to_string())
}

#[tauri::command]
pub async fn generate_project_summary(request: SummaryRequest) -> Result<ProjectSummary, String> {
    let client = reqwest::Client::new();
    let file_previews: Vec<String> = request
        .analysis
        .files
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
1. What this application/project does
2. Core features
3. Technical implementation
4. Project type

Write the summary in clear, accessible language.",
        request.analysis.technologies.join(", "),
        request.analysis.metrics.get("total_files").unwrap_or(&0),
        request.analysis.metrics.get("total_lines").unwrap_or(&0),
        request.analysis.structure.len(),
        file_previews.join("\n---\n")
    );

    let mut headers = HeaderMap::new();
    if !request.settings.api_key.is_empty() {
        headers.insert(AUTHORIZATION, format!("Bearer {}", request.settings.api_key).parse().unwrap());
    }
    headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());

    let body = serde_json::json!({
        "model": request.settings.model,
        "messages": [
            { "role": "system", "content": "You are a technical documentation specialist who creates clear, concise project summaries." },
            { "role": "user", "content": prompt }
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