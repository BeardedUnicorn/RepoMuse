use crate::analysis::RepoAnalysis;
use crate::storage::{ProjectSummary, Settings};
use regex::Regex;
use reqwest::header::{HeaderMap, AUTHORIZATION, CONTENT_TYPE};
use serde::{Deserialize, Serialize};
use once_cell::sync::Lazy;
use std::fmt::Write;

// Cached regex patterns
static THINKING_REGEX: Lazy<Regex> = 
    Lazy::new(|| Regex::new(r"(?s)<think>(.*?)</think>(.*)").unwrap());

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

// Optimized: Use cached regex instead of recompiling
fn extract_thinking_and_response(content: &str) -> (Option<String>, String) {
    if let Some(captures) = THINKING_REGEX.captures(content) {
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
    let mut ideas = Vec::with_capacity(10); // Pre-allocate for typical case
    let mut current_idea = String::with_capacity(500); // Pre-allocate
    
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
    let mut features = Vec::with_capacity(10); // Pre-allocate
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

// Smart suggestion engine structures with pre-allocated capacity
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

impl ProjectKeywords {
    fn new() -> Self {
        ProjectKeywords {
            api_related: Vec::with_capacity(20),
            auth_related: Vec::with_capacity(20),
            database_related: Vec::with_capacity(20),
            testing_related: Vec::with_capacity(20),
            cicd_related: Vec::with_capacity(20),
            ui_related: Vec::with_capacity(20),
            performance_related: Vec::with_capacity(20),
            security_related: Vec::with_capacity(20),
        }
    }
}

#[derive(Debug, Clone)]
struct TechnologyProfile {
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
    let mut keywords = ProjectKeywords::new();

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
        frameworks: Vec::with_capacity(10),
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
    let mut suggestions = Vec::with_capacity(20); // Pre-allocate
    
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
    
    suggestions
}

// Optimized: Pre-allocate string capacity and use write! macro
fn build_comprehensive_context(analysis: &RepoAnalysis) -> String {
    // Pre-allocate with reasonable capacity
    let mut context = String::with_capacity(50_000);
    
    // Extract keywords and analyze technology profile
    let keywords = extract_project_keywords(analysis);
    let profile = analyze_technology_profile(analysis, &keywords);
    
    // Use write! macro instead of push_str with format!
    let _ = write!(&mut context, 
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
    );
    
    // Add keyword analysis
    let _ = write!(&mut context, "Detected Project Characteristics:\n");
    if profile.has_api {
        let _ = write!(&mut context, "- API/Backend functionality ({} files)\n", keywords.api_related.len());
    }
    if profile.has_auth {
        let _ = write!(&mut context, "- Authentication system ({} files)\n", keywords.auth_related.len());
    }
    if profile.has_database {
        let _ = write!(&mut context, "- Database operations ({} files)\n", keywords.database_related.len());
    }
    if profile.has_testing {
        let _ = write!(&mut context, "- Testing framework ({} files)\n", keywords.testing_related.len());
    }
    if profile.has_cicd {
        let _ = write!(&mut context, "- CI/CD pipeline ({} files)\n", keywords.cicd_related.len());
    }
    if profile.has_ui {
        let _ = write!(&mut context, "- UI components ({} files)\n", keywords.ui_related.len());
    }
    let _ = write!(&mut context, "\n");
    
    // High-level observations (neutral, non-prescriptive)
    let _ = write!(&mut context, "Observations:\n");
    // Common files presence
    let has_readme = analysis.files.iter().any(|f| f.path.to_lowercase().contains("readme"));
    let has_license = analysis.files.iter().any(|f| f.path.to_lowercase().ends_with("license") || f.path.to_lowercase().contains("license."));
    let has_gitignore = analysis.files.iter().any(|f| f.path.ends_with(".gitignore"));
    let _ = write!(
        &mut context,
        "- Common files: README={} LICENSE={} .gitignore={}\n",
        if has_readme { "yes" } else { "no" },
        if has_license { "yes" } else { "no" },
        if has_gitignore { "yes" } else { "no" }
    );
    // Testing snapshot
    let mut test_file_count = 0usize;
    let mut testing_frameworks: Vec<&'static str> = Vec::new();
    for file in &analysis.files {
        let name = file.path.to_lowercase();
        let is_test_name = name.ends_with(".test.js") || name.ends_with(".test.ts") || name.ends_with(".test.jsx") || name.ends_with(".test.tsx")
            || name.ends_with(".spec.js") || name.ends_with(".spec.ts") || name.ends_with(".spec.jsx") || name.ends_with(".spec.tsx")
            || name.ends_with("_test.py") || name.contains("/test/") || name.contains("/tests/") || name.contains("/__tests__/") || name.contains("/spec/");
        if is_test_name { test_file_count += 1; }
        let lower = file.content.to_lowercase();
        if lower.contains("jest") && !testing_frameworks.contains(&"Jest") { testing_frameworks.push("Jest"); }
        if lower.contains("vitest") && !testing_frameworks.contains(&"Vitest") { testing_frameworks.push("Vitest"); }
        if lower.contains("mocha") && !testing_frameworks.contains(&"Mocha") { testing_frameworks.push("Mocha"); }
        if lower.contains("jasmine") && !testing_frameworks.contains(&"Jasmine") { testing_frameworks.push("Jasmine"); }
        if lower.contains("cypress") && !testing_frameworks.contains(&"Cypress") { testing_frameworks.push("Cypress"); }
        if lower.contains("playwright") && !testing_frameworks.contains(&"Playwright") { testing_frameworks.push("Playwright"); }
    }
    testing_frameworks.sort();
    let _ = write!(
        &mut context,
        "- Testing: frameworks=[{}] test_files={}\n",
        testing_frameworks.join(", "),
        test_file_count
    );
    // CI snapshot
    let _ = write!(
        &mut context,
        "- CI/CD: {} ({} files)\n",
        if profile.has_cicd { "detected" } else { "not_detected" },
        keywords.cicd_related.len()
    );
    // Known gaps
    let mut gaps: Vec<&str> = Vec::new();
    if !profile.has_testing || test_file_count == 0 { gaps.push("No tests detected"); }
    if !profile.has_cicd { gaps.push("No CI configuration detected"); }
    if !has_readme { gaps.push("README missing"); }
    if !has_license { gaps.push("LICENSE missing"); }
    if !gaps.is_empty() {
        let _ = write!(&mut context, "- Known gaps: {}\n", gaps.join(", "));
    }
    let _ = write!(&mut context, "\n");
    
    // Detailed file analysis
    let _ = write!(&mut context, "File Analysis:\n");
    
    // Group files by type/purpose
    let mut config_files = Vec::new();
    let mut source_files = Vec::new();
    let mut test_files = Vec::new();
    
    for file in &analysis.files {
        let path_lower = file.path.to_lowercase();
        
        if path_lower.contains("test") || path_lower.contains("spec") {
            test_files.push(file);
        } else if path_lower.contains("config") || file.path.ends_with(".json") || 
                  file.path.ends_with(".yml") || file.path.ends_with(".yaml") ||
                  file.path.ends_with(".toml") {
            config_files.push(file);
        } else {
            source_files.push(file);
        }
    }
    
    // Provide context about notable files (prefer roles over long previews)
    if !source_files.is_empty() {
        // Select top by size as a simple proxy for centrality
        let mut sorted_sources = source_files.clone();
        sorted_sources.sort_by_key(|f| std::cmp::Reverse(f.size));
        let _ = write!(&mut context, "\nNotable Files (by size):\n");
        for file in sorted_sources.iter().take(5) {
            let _ = write!(&mut context, "- {} ({}, {} bytes)\n", file.path, file.language, file.size);
        }
        // Include short previews for the top 2 only
        let _ = write!(&mut context, "\nContent Previews (top 2):\n");
        for file in sorted_sources.iter().take(2) {
            let preview = if file.content.len() > 300 {
                format!("{}...", &file.content[..300])
            } else {
                file.content.clone()
            };
            let _ = write!(&mut context, "\n{} ({}):\n{}\n", file.path, file.language, preview);
        }
    }
    
    // Directory structure
    let _ = write!(&mut context, "\nProject Structure:\n");
    let mut structure_vec: Vec<(&String, &Vec<String>)> = analysis.structure.iter().collect();
    structure_vec.sort_by_key(|(dir, _)| *dir);
    
    for (dir, files) in structure_vec.iter().take(20) {
        let _ = write!(&mut context, "  {}/: {} files\n", dir, files.len());
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
   - SPECIFIC: Include at least one exact file path or symbol using backticks (e.g., `src/components/Foo.tsx`, function `bar()`).
   - ACTIONABLE: State the key implementation step(s) inline; if adding a dependency, name it and where to add it (e.g., `package.json` devDependencies).
   - VALUABLE: Solve real problems or add meaningful capabilities.
   - UNIQUE: Do not duplicate other items.
   - VERIFIED: If confidence < 60% or evidence is weak, prefix with "Verify:" and state the assumption.

4. Focus categories (cover distinct areas):
   - Missing Features, Performance, Testing Gaps, Security, Developer Experience, User Experience, Technical Debt, Documentation, Integration, Monitoring

RESPONSE FORMAT (STRICT):
- Output ONLY a numbered list 1-10 (no sub-bullets, no nested numbering, no code fences).
- Each item is 2-3 sentences: WHAT, WHY (impact), optional HOW.
- Prepend tags: [Category: ...] [Affected: `path1`, `path2`].
- Append triage: [Impact: H/M/L] [Effort: S/M/L] [Confidence: %].

Start directly with '1.' and end after '10.'.",
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
                "content": "You are a senior software architect and product‑minded engineer. Produce exactly 10 improvement ideas for the provided repository that are specific, actionable, and valuable.\n\nStrict format: Output only a numbered list 1–10. Each item is 2–3 sentences: (1) WHAT to implement, (2) WHY it matters (impact), (3, optional) HOW at a high level. No preamble, no closing, no code fences.\n\nGrounding: Base every idea on the provided repository context. Reference at least one concrete file path, module, component, or symbol you observed (e.g., `src/components/Foo.tsx`, function `bar()`). If you cannot find direct evidence, prefix the item with 'Verify:' and state the assumption.\n\nQuality: Never suggest re‑implementing existing features. Avoid duplication across items and cover different areas (features, performance, testing, security, DX/UX). Prefer high‑ROI changes over trivial tasks.\n\nTriage tags: Append minimal tags per item — [Impact: H/M/L] [Effort: S/M/L] [Confidence: %]."
            },
            { "role": "user", "content": prompt }
        ],
        "max_tokens": request.settings.max_tokens_ideas,
        "temperature": request.settings.temperature_ideas,
        "frequency_penalty": request.settings.frequency_penalty_ideas,
        "presence_penalty": request.settings.presence_penalty_ideas,
        "stop": ["\n11."]
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
        "Analyze this code repository and create a concise, code-grounded summary.

Repository Analysis:
- Technologies: {}
- Total Files: {}
- Total Lines: {}
- Directory Structure: {} directories analyzed

File Previews:
{}

Your summary MUST include sections in this exact order and be extractor-friendly:
- Overview (2–3 sentences)
- Key Features (max 5 bullets, each starting with '- ')
- Architecture (1–3 sentences; patterns, layers, data flow)
- Tech Stack (single line, comma-separated)
- Notable Files (max 5 bullets with key paths and roles)
- Intended Users/Use Cases (1–2 sentences)
- Limitations/Unknowns (bulleted; use 'Unknown' where evidence is absent)

Rules:
- Ground claims in code/configs; reference file paths/symbols with backticks when helpful.
- Avoid speculation or marketing language.
- Total length under ~300 words.
- No preamble, no conclusion, no code fences.",
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
            { "role": "system", "content": "You are a technical documentation specialist. Create a concise, code‑grounded summary of the repository based on the provided context.\n\nOutput only these sections, in order, with brief content:\n- Overview (2–3 sentences)\n- Key Features (bulleted '- ' lines)\n- Architecture (1–3 sentences; patterns, layers, data flow)\n- Tech Stack (comma‑separated)\n- Notable Files (bulleted with key paths and roles)\n- Intended Users/Use Cases (1–2 sentences)\n- Limitations/Unknowns (bulleted; use 'Unknown' where evidence is absent)\n\nGround claims in the code and configs (reference file paths/symbols when helpful). Avoid speculation or marketing language. Keep the total length under ~300 words. No preamble, no closing, no code fences." },
            { "role": "user", "content": prompt }
        ],
        "max_tokens": request.settings.max_tokens_summary,
        "temperature": request.settings.temperature_summary,
        "presence_penalty": request.settings.presence_penalty_summary
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
