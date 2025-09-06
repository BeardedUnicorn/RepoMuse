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
    pub focus_area: Option<String>,
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

// Smart suggestion engine structures
#[derive(Debug, Clone)]
struct ProjectKeywords {
    api_related: Vec<String>,
    auth_related: Vec<String>,
    database_related: Vec<String>,
    testing_related: Vec<String>,
    cicd_related: Vec<String>,
    ui_related: Vec<String>,
    performance_related: Vec<String>,
    security_related: Vec<String>,
}

#[derive(Debug, Clone)]
struct TechnologyProfile {
    primary_tech: Vec<String>,
    frameworks: Vec<String>,
    has_api: bool,
    has_auth: bool,
    has_database: bool,
    has_testing: bool,
    has_cicd: bool,
    has_ui: bool,
    project_type: String,
}

fn extract_project_keywords(analysis: &RepoAnalysis) -> ProjectKeywords {
    let mut keywords = ProjectKeywords {
        api_related: Vec::new(),
        auth_related: Vec::new(),
        database_related: Vec::new(),
        testing_related: Vec::new(),
        cicd_related: Vec::new(),
        ui_related: Vec::new(),
        performance_related: Vec::new(),
        security_related: Vec::new(),
    };

    // Analyze all file contents for keywords
    for file in &analysis.files {
        let content_lower = file.content.to_lowercase();
        
        // API keywords
        if content_lower.contains("api") || content_lower.contains("endpoint") || 
           content_lower.contains("rest") || content_lower.contains("graphql") ||
           content_lower.contains("swagger") || content_lower.contains("openapi") {
            keywords.api_related.push(file.path.clone());
        }
        
        // Authentication keywords
        if content_lower.contains("auth") || content_lower.contains("login") ||
           content_lower.contains("jwt") || content_lower.contains("oauth") ||
           content_lower.contains("session") || content_lower.contains("password") {
            keywords.auth_related.push(file.path.clone());
        }
        
        // Database keywords
        if content_lower.contains("database") || content_lower.contains("sql") ||
           content_lower.contains("mongo") || content_lower.contains("redis") ||
           content_lower.contains("postgres") || content_lower.contains("mysql") ||
           content_lower.contains("migration") || content_lower.contains("schema") {
            keywords.database_related.push(file.path.clone());
        }
        
        // Testing keywords
        if content_lower.contains("test") || content_lower.contains("spec") ||
           content_lower.contains("jest") || content_lower.contains("mocha") ||
           content_lower.contains("vitest") || content_lower.contains("cypress") {
            keywords.testing_related.push(file.path.clone());
        }
        
        // CI/CD keywords
        if file.path.contains(".github/workflows") || file.path.contains("gitlab-ci") ||
           content_lower.contains("pipeline") || content_lower.contains("deploy") ||
           content_lower.contains("docker") || content_lower.contains("kubernetes") {
            keywords.cicd_related.push(file.path.clone());
        }
        
        // UI keywords
        if content_lower.contains("component") || content_lower.contains("react") ||
           content_lower.contains("vue") || content_lower.contains("angular") ||
           content_lower.contains("tailwind") || content_lower.contains("css") {
            keywords.ui_related.push(file.path.clone());
        }
        
        // Performance keywords
        if content_lower.contains("cache") || content_lower.contains("optimize") ||
           content_lower.contains("performance") || content_lower.contains("lazy") ||
           content_lower.contains("memoize") || content_lower.contains("throttle") {
            keywords.performance_related.push(file.path.clone());
        }
        
        // Security keywords
        if content_lower.contains("security") || content_lower.contains("encrypt") ||
           content_lower.contains("cors") || content_lower.contains("xss") ||
           content_lower.contains("csrf") || content_lower.contains("sanitize") {
            keywords.security_related.push(file.path.clone());
        }
    }
    
    // Deduplicate
    keywords.api_related.sort();
    keywords.api_related.dedup();
    keywords.auth_related.sort();
    keywords.auth_related.dedup();
    keywords.database_related.sort();
    keywords.database_related.dedup();
    keywords.testing_related.sort();
    keywords.testing_related.dedup();
    keywords.cicd_related.sort();
    keywords.cicd_related.dedup();
    keywords.ui_related.sort();
    keywords.ui_related.dedup();
    keywords.performance_related.sort();
    keywords.performance_related.dedup();
    keywords.security_related.sort();
    keywords.security_related.dedup();
    
    keywords
}

fn analyze_technology_profile(analysis: &RepoAnalysis, keywords: &ProjectKeywords) -> TechnologyProfile {
    let mut profile = TechnologyProfile {
        primary_tech: analysis.technologies.clone(),
        frameworks: Vec::new(),
        has_api: !keywords.api_related.is_empty(),
        has_auth: !keywords.auth_related.is_empty(),
        has_database: !keywords.database_related.is_empty(),
        has_testing: !keywords.testing_related.is_empty(),
        has_cicd: !keywords.cicd_related.is_empty(),
        has_ui: !keywords.ui_related.is_empty(),
        project_type: String::new(),
    };
    
    // Detect frameworks from file analysis
    for file in &analysis.files {
        let content = &file.content;
        
        // React/Next.js
        if content.contains("import React") || content.contains("from 'react'") {
            profile.frameworks.push("React".to_string());
        }
        if content.contains("from 'next'") || content.contains("next/") {
            profile.frameworks.push("Next.js".to_string());
        }
        
        // Vue
        if content.contains("from 'vue'") || content.contains("Vue.") {
            profile.frameworks.push("Vue".to_string());
        }
        
        // Express/Node
        if content.contains("express()") || content.contains("from 'express'") {
            profile.frameworks.push("Express".to_string());
        }
        
        // Django/Flask
        if content.contains("from django") || content.contains("django.") {
            profile.frameworks.push("Django".to_string());
        }
        if content.contains("from flask") || content.contains("Flask(") {
            profile.frameworks.push("Flask".to_string());
        }
        
        // Spring Boot
        if content.contains("@SpringBoot") || content.contains("springframework") {
            profile.frameworks.push("Spring Boot".to_string());
        }
        
        // Tauri
        if content.contains("tauri::") || file.path.contains("tauri") {
            profile.frameworks.push("Tauri".to_string());
        }
    }
    
    profile.frameworks.sort();
    profile.frameworks.dedup();
    
    // Determine project type
    profile.project_type = if profile.frameworks.contains(&"Tauri".to_string()) {
        "Desktop Application".to_string()
    } else if profile.has_ui && profile.has_api {
        "Full-Stack Web Application".to_string()
    } else if profile.has_ui {
        "Frontend Application".to_string()
    } else if profile.has_api {
        "Backend API Service".to_string()
    } else if keywords.testing_related.len() > 5 {
        "Library/Package".to_string()
    } else {
        "General Application".to_string()
    };
    
    profile
}

fn generate_smart_suggestions(profile: &TechnologyProfile, keywords: &ProjectKeywords) -> Vec<String> {
    let mut suggestions = Vec::new();
    
    // API-specific suggestions
    if profile.has_api {
        if !keywords.api_related.iter().any(|p| p.contains("rate") || p.contains("limit")) {
            suggestions.push("API rate limiting to prevent abuse".to_string());
        }
        if !keywords.api_related.iter().any(|p| p.contains("version")) {
            suggestions.push("API versioning strategy for backward compatibility".to_string());
        }
        if !keywords.api_related.iter().any(|p| p.contains("doc") || p.contains("swagger")) {
            suggestions.push("API documentation with OpenAPI/Swagger".to_string());
        }
    }
    
    // Authentication-specific suggestions
    if profile.has_auth {
        if !keywords.auth_related.iter().any(|p| p.contains("2fa") || p.contains("mfa")) {
            suggestions.push("Two-factor authentication (2FA) implementation".to_string());
        }
        if !keywords.auth_related.iter().any(|p| p.contains("refresh")) {
            suggestions.push("Token refresh mechanism for better security".to_string());
        }
        if !keywords.auth_related.iter().any(|p| p.contains("rbac") || p.contains("role")) {
            suggestions.push("Role-based access control (RBAC) system".to_string());
        }
    }
    
    // Database-specific suggestions
    if profile.has_database {
        if !keywords.database_related.iter().any(|p| p.contains("index")) {
            suggestions.push("Database indexing strategy for query optimization".to_string());
        }
        if !keywords.database_related.iter().any(|p| p.contains("backup")) {
            suggestions.push("Automated database backup and recovery system".to_string());
        }
        if !keywords.database_related.iter().any(|p| p.contains("pool")) {
            suggestions.push("Connection pooling for database performance".to_string());
        }
    }
    
    // Testing-specific suggestions
    if profile.has_testing {
        if !keywords.testing_related.iter().any(|p| p.contains("e2e")) {
            suggestions.push("End-to-end (E2E) testing suite".to_string());
        }
        if !keywords.testing_related.iter().any(|p| p.contains("coverage")) {
            suggestions.push("Code coverage reporting and thresholds".to_string());
        }
        if !keywords.testing_related.iter().any(|p| p.contains("mock")) {
            suggestions.push("Mock data generation for testing".to_string());
        }
    }
    
    // CI/CD-specific suggestions
    if profile.has_cicd {
        if !keywords.cicd_related.iter().any(|p| p.contains("semantic")) {
            suggestions.push("Semantic versioning automation".to_string());
        }
        if !keywords.cicd_related.iter().any(|p| p.contains("security") || p.contains("scan")) {
            suggestions.push("Security scanning in CI pipeline".to_string());
        }
    } else {
        suggestions.push("CI/CD pipeline setup for automated testing and deployment".to_string());
    }
    
    // UI-specific suggestions
    if profile.has_ui {
        if !keywords.ui_related.iter().any(|p| p.contains("responsive")) {
            suggestions.push("Responsive design improvements for mobile devices".to_string());
        }
        if !keywords.ui_related.iter().any(|p| p.contains("accessibility") || p.contains("a11y")) {
            suggestions.push("Accessibility (a11y) compliance and screen reader support".to_string());
        }
        if !keywords.ui_related.iter().any(|p| p.contains("theme") || p.contains("dark")) {
            suggestions.push("Dark mode theme support".to_string());
        }
    }
    
    // Performance-specific suggestions
    if !keywords.performance_related.is_empty() || profile.has_api || profile.has_database {
        if !keywords.performance_related.iter().any(|p| p.contains("cache")) {
            suggestions.push("Caching strategy for improved performance".to_string());
        }
        if profile.has_ui && !keywords.performance_related.iter().any(|p| p.contains("lazy")) {
            suggestions.push("Lazy loading for better initial load times".to_string());
        }
    }
    
    // Security-specific suggestions
    if !keywords.security_related.iter().any(|p| p.contains("audit")) {
        suggestions.push("Security audit logging system".to_string());
    }
    if profile.has_api && !keywords.security_related.iter().any(|p| p.contains("cors")) {
        suggestions.push("CORS configuration for API security".to_string());
    }
    
    // Technology-specific suggestions
    for tech in &profile.primary_tech {
        match tech.as_str() {
            "TypeScript" => {
                if !keywords.testing_related.iter().any(|p| p.contains("type")) {
                    suggestions.push("Strict TypeScript configuration for better type safety".to_string());
                }
            },
            "Python" => {
                if !keywords.testing_related.iter().any(|p| p.contains("pytest")) {
                    suggestions.push("Pytest configuration with fixtures and parametrization".to_string());
                }
            },
            "Rust" => {
                if !keywords.performance_related.iter().any(|p| p.contains("bench")) {
                    suggestions.push("Benchmarking suite with criterion for performance tracking".to_string());
                }
            },
            _ => {}
        }
    }
    
    // Framework-specific suggestions
    for framework in &profile.frameworks {
        match framework.as_str() {
            "React" => {
                if !keywords.ui_related.iter().any(|p| p.contains("memo") || p.contains("useMemo")) {
                    suggestions.push("React performance optimization with memo and useMemo".to_string());
                }
            },
            "Next.js" => {
                if !keywords.performance_related.iter().any(|p| p.contains("ssg") || p.contains("ssr")) {
                    suggestions.push("Static Site Generation (SSG) for better performance".to_string());
                }
            },
            "Express" => {
                if !keywords.security_related.iter().any(|p| p.contains("helmet")) {
                    suggestions.push("Helmet.js integration for Express security headers".to_string());
                }
            },
            "Tauri" => {
                if !keywords.security_related.iter().any(|p| p.contains("ipc")) {
                    suggestions.push("IPC command validation for Tauri security".to_string());
                }
            },
            _ => {}
        }
    }
    
    suggestions
}

fn build_comprehensive_context(analysis: &RepoAnalysis) -> String {
    let mut context = String::new();
    
    // Extract keywords and analyze technology profile
    let keywords = extract_project_keywords(analysis);
    let profile = analyze_technology_profile(analysis, &keywords);
    let smart_suggestions = generate_smart_suggestions(&profile, &keywords);
    
    // Add smart analysis to context
    context.push_str(&format!(
        "Project Type: {}\n\
        Technologies: {}\n\
        Frameworks: {}\n\
        Total Files: {}\n\
        Total Lines: {}\n\n",
        profile.project_type,
        analysis.technologies.join(", "),
        profile.frameworks.join(", "),
        analysis.metrics.get("total_files").unwrap_or(&0),
        analysis.metrics.get("total_lines").unwrap_or(&0),
    ));
    
    // Add keyword analysis
    context.push_str("Detected Project Characteristics:\n");
    if profile.has_api {
        context.push_str(&format!("- API/Backend functionality ({} files)\n", keywords.api_related.len()));
    }
    if profile.has_auth {
        context.push_str(&format!("- Authentication system ({} files)\n", keywords.auth_related.len()));
    }
    if profile.has_database {
        context.push_str(&format!("- Database operations ({} files)\n", keywords.database_related.len()));
    }
    if profile.has_testing {
        context.push_str(&format!("- Testing framework ({} files)\n", keywords.testing_related.len()));
    }
    if profile.has_cicd {
        context.push_str(&format!("- CI/CD pipeline ({} files)\n", keywords.cicd_related.len()));
    }
    if profile.has_ui {
        context.push_str(&format!("- UI components ({} files)\n", keywords.ui_related.len()));
    }
    context.push_str("\n");
    
    // Add smart suggestions
    if !smart_suggestions.is_empty() {
        context.push_str("Smart Suggestions Based on Project Analysis:\n");
        for suggestion in &smart_suggestions {
            context.push_str(&format!("- {}\n", suggestion));
        }
        context.push_str("\n");
    }
    
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
        for file in source_files.iter().take(20) {
            let preview = if file.content.len() > 1000 {
                format!("{}...", &file.content[..1000])
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
            let has_state_management = preview.contains("useState") || preview.contains("Redux") || preview.contains("Vuex");
            let has_routing = preview.contains("Router") || preview.contains("Route") || preview.contains("navigation");
            
            context.push_str(&format!(
                "\n{} ({}): {} chars\n",
                file.path, file.language, file.content.len()
            ));
            
            context.push_str("  Features detected:\n");
            if has_exports { context.push_str("  - Exports modules/functions\n"); }
            if has_imports { context.push_str("  - Imports external dependencies\n"); }
            if has_classes { context.push_str("  - Defines classes/interfaces\n"); }
            if has_functions { context.push_str("  - Contains functions/methods\n"); }
            if has_api_calls { context.push_str("  - Makes API/HTTP calls\n"); }
            if has_database { context.push_str("  - Database operations\n"); }
            if has_state_management { context.push_str("  - State management\n"); }
            if has_routing { context.push_str("  - Routing/navigation\n"); }
            
            context.push_str(&format!("Content preview:\n{}\n", preview));
        }
    }
    
    if !config_files.is_empty() {
        context.push_str("\nConfiguration Files (dependencies and settings):\n");
        for file in config_files.iter().take(10) {
            let preview = if file.content.len() > 600 {
                format!("{}...", &file.content[..600])
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
        context.push_str("Test files (helps understand what's already tested):\n");
        for file in test_files.iter().take(8) {
            let preview = if file.content.len() > 400 {
                format!("{}...", &file.content[..400])
            } else {
                file.content.clone()
            };
            context.push_str(&format!("  - {}\n    Preview: {}\n", file.path, 
                preview.lines().next().unwrap_or("")));
        }
    }
    
    if !doc_files.is_empty() {
        context.push_str("\nDocumentation (existing docs to understand project):\n");
        for file in doc_files.iter().take(5) {
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
    
    // Directory structure analysis with insights
    context.push_str("\nProject Structure Analysis:\n");
    let mut structure_insights = Vec::new();
    
    for (dir, files) in &analysis.structure {
        if files.len() > 0 {
            let dir_lower = dir.to_lowercase();
            let insight = if dir_lower.contains("component") {
                "UI components"
            } else if dir_lower.contains("service") || dir_lower.contains("api") {
                "API/Service layer"
            } else if dir_lower.contains("model") || dir_lower.contains("schema") {
                "Data models"
            } else if dir_lower.contains("util") || dir_lower.contains("helper") {
                "Utility functions"
            } else if dir_lower.contains("test") || dir_lower.contains("spec") {
                "Test files"
            } else if dir_lower.contains("style") || dir_lower.contains("css") {
                "Styling"
            } else if dir_lower.contains("asset") || dir_lower.contains("public") {
                "Static assets"
            } else if dir_lower.contains("config") {
                "Configuration"
            } else {
                "General"
            };
            
            structure_insights.push(format!("  {}/: {} files ({})", dir, files.len(), insight));
        }
    }
    
    for insight in structure_insights.iter().take(20) {
        context.push_str(&format!("{}\n", insight));
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
    
    // Build focus-specific instructions
    let focus_instructions = if let Some(ref focus) = request.focus_area {
        format!(
            "\n\nIMPORTANT FOCUS AREA: The user specifically wants ideas focused on '{}'.\n\
            Please generate ALL ideas with a strong emphasis on this area. For example:\n\
            - If the focus is 'documentation': suggest README improvements, API docs, code comments, documentation generators, etc.\n\
            - If the focus is 'testing': suggest unit tests, integration tests, test coverage, testing frameworks, etc.\n\
            - If the focus is 'performance': suggest optimization opportunities, caching, lazy loading, database indexing, etc.\n\
            - If the focus is 'security': suggest authentication, authorization, input validation, encryption, security audits, etc.\n\
            - If the focus is 'UI/UX': suggest interface improvements, accessibility, responsive design, user flows, etc.\n\
            Make sure at least 80% of the ideas directly relate to '{}'.",
            focus, focus
        )
    } else {
        String::new()
    };
    
    let prompt = format!(
        "REPOSITORY CONTEXT:\n{}\n\n{}

TASK: Generate exactly 10 development ideas for this repository.

REQUIREMENTS:
1. Analyze the provided code to understand what EXISTS:
   - Current features and implementations
   - Existing tests and coverage
   - Documentation status
   - Architecture patterns

2. DO NOT suggest already implemented features:
   - ✗ If theme switching exists, don't suggest adding themes
   - ✗ If authentication exists, don't suggest adding auth
   - ✗ If a component has tests, don't suggest testing it
   - ✗ If responsive design exists, don't suggest making it responsive

3. Each idea MUST be:
   - SPECIFIC: Include exact file names, component names, or code locations
   - ACTIONABLE: Clear implementation steps, not vague suggestions
   - VALUABLE: Solve real problems or add meaningful capabilities
   - UNIQUE: Not duplicate existing functionality

4. Focus categories:
   - Missing Features: New functionality that adds value
   - Performance: Specific optimizations with measurable impact
   - Testing Gaps: Untested critical paths or edge cases
   - Security: Concrete vulnerabilities to address
   - Developer Experience: Tooling, debugging, or workflow improvements
   - User Experience: Specific UI/UX enhancements
   - Technical Debt: Refactoring opportunities with clear benefits
   - Documentation: Critical undocumented areas
   - Integration: External services or tools to integrate
   - Monitoring: Observability and error tracking

RESPONSE FORMAT:
Return EXACTLY 10 ideas as a numbered list (1-10).
Each idea should be 2-3 sentences:
- First sentence: WHAT to implement
- Second sentence: WHY it's valuable and expected impact
- Optional third sentence: HOW to implement (key approach)

Start directly with '1.' - no introduction.
End after '10.' - no conclusion.

Example format:
1. Implement request caching in `src/utils/api.ts` for the `analyzeRepository` function using a Map-based cache with 5-minute TTL. This would reduce redundant API calls by 40% when users switch between projects frequently. Use the existing cache pattern from `src-tauri/src/cache.rs` as a reference.

2. Add keyboard shortcuts (Cmd/Ctrl+K for search, Cmd/Ctrl+G for generate) to improve power user productivity. This would speed up common workflows by eliminating mouse navigation for frequent actions. Implement using a global keyboard event listener with a shortcuts registry pattern.",
        comprehensive_context, focus_instructions
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
                "content": "You are an expert software architect and developer with deep knowledge of modern development practices, testing strategies, performance optimization, and user experience design. You excel at analyzing codebases to identify improvement opportunities that are specific, actionable, and valuable. You never suggest reimplementing existing features."
            },
            { "role": "user", "content": prompt }
        ],
        "max_tokens": 50000,
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
1. What this application/project does (main purpose and functionality)
2. Core features and capabilities
3. Technical architecture and implementation approach
4. Target users or use cases
5. Notable design patterns or architectural decisions

Write the summary in clear, accessible language that both technical and non-technical readers can understand.
Focus on what the project ACTUALLY does based on the code, not what it could do.",
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
            { "role": "system", "content": "You are a technical documentation specialist who creates clear, accurate, and insightful project summaries. You analyze code to understand what projects actually do and explain it clearly." },
            { "role": "user", "content": prompt }
        ],
        "max_tokens": 50000,
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