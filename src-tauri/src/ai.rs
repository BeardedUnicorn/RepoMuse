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

IMPORTANT: Format your response as a numbered list with one idea per line.",
        request.analysis.technologies.join(", "),
        request.analysis.metrics.get("total_files").unwrap_or(&0),
        request.analysis.metrics.get("total_lines").unwrap_or(&0),
        request.analysis.structure.len(),
        request
            .analysis
            .files
            .iter()
            .take(10)
            .map(|f| format!(
                "{} ({}): {}",
                f.path,
                f.language,
                if f.content.len() > 200 {
                    format!("{}...", &f.content[..200])
                } else {
                    f.content.clone()
                }
            ))
            .collect::<Vec<_>>()
            .join("\n\n")
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
                "content": "You are a senior software engineer and architect who provides creative, practical development ideas for code repositories. Always format your response as a clear numbered list with one idea per line."
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

