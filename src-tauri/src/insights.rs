use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use std::process::Command;
// use ignore::WalkBuilder; // using helper walkers in fs_utils
use crate::fs_utils::walker_with_depth;

#[derive(Debug, Serialize, Deserialize)]
pub struct GitRemote {
  pub name: String,
  pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GitStatus {
  pub is_git_repo: bool,
  pub has_uncommitted_changes: bool,
  pub uncommitted_files: Vec<String>,
  pub current_branch: Option<String>,
  pub last_commit_date: Option<String>,
  pub commit_count: Option<usize>,
  pub remotes: Vec<GitRemote>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReadmeInfo {
  pub exists: bool,
  pub is_default: bool,
  pub path: Option<String>,
  pub content_preview: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CIInfo {
  pub has_ci: bool,
  pub ci_platforms: Vec<String>,
  pub ci_files: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PackageInfo {
  pub has_package_json: bool,
  pub has_cargo_toml: bool,
  pub has_requirements_txt: bool,
  pub has_gemfile: bool,
  pub has_go_mod: bool,
  pub missing_common_files: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TestingInfo {
  pub has_testing_framework: bool,
  pub testing_frameworks: Vec<String>,
  pub has_test_files: bool,
  pub test_file_count: usize,
  pub test_file_patterns: Vec<String>,
  pub source_to_test_ratio: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectInsights {
  pub git_status: GitStatus,
  pub readme_info: ReadmeInfo,
  pub ci_info: CIInfo,
  pub package_info: PackageInfo,
  pub testing_info: TestingInfo,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GitCommit {
  pub hash: String,
  pub author: String,
  pub date: String,
  pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GitLog {
  pub commits: Vec<GitCommit>,
  pub total_commits: usize,
  pub branches: Vec<String>,
  pub current_branch: Option<String>,
}

fn get_git_status(path: &Path) -> GitStatus {
  let is_git_repo = path.join(".git").exists();
  
  let mut remotes = Vec::new();
  if is_git_repo {
    // Get git remotes
    if let Ok(output) = Command::new("git")
      .args(&["remote", "-v"])
      .current_dir(path)
      .output() 
    {
      if output.status.success() {
        let output_str = String::from_utf8_lossy(&output.stdout);
        let mut seen_remotes = std::collections::HashSet::new();
        
        for line in output_str.lines() {
          let parts: Vec<&str> = line.split_whitespace().collect();
          if parts.len() >= 2 {
            let name = parts[0].to_string();
            let url = parts[1].to_string();
            
            // Only add each remote once (git remote -v shows fetch and push)
            if seen_remotes.insert(name.clone()) {
              remotes.push(GitRemote { name, url });
            }
          }
        }
      }
    }
  }
  
  GitStatus {
    is_git_repo,
    has_uncommitted_changes: false,
    uncommitted_files: vec![],
    current_branch: None,
    last_commit_date: None,
    commit_count: None,
    remotes,
  }
}

fn get_readme_info(path: &Path) -> ReadmeInfo {
  let candidates = ["README.md", "README.txt", "readme.md", "readme.txt"]; 
  for name in candidates.iter() {
    let p = path.join(name);
    if p.exists() {
      let preview = fs::read_to_string(&p).ok().map(|s| s.chars().take(200).collect());
      return ReadmeInfo { exists: true, is_default: false, path: Some(p.to_string_lossy().to_string()), content_preview: preview };
    }
  }
  ReadmeInfo { exists: false, is_default: false, path: None, content_preview: None }
}

fn get_ci_info(path: &Path) -> CIInfo {
  let mut ci_platforms = Vec::new();
  let mut ci_files = Vec::new();
  if path.join(".github").join("workflows").exists() {
    ci_platforms.push("GitHub Actions".to_string());
    if let Ok(entries) = fs::read_dir(path.join(".github/workflows")) {
      for e in entries.flatten() {
        ci_files.push(format!(".github/workflows/{}", e.file_name().to_string_lossy()));
      }
    }
  }
  if path.join(".gitlab-ci.yml").exists() { ci_platforms.push("GitLab CI".to_string()); ci_files.push(".gitlab-ci.yml".to_string()); }
  if path.join(".travis.yml").exists() { ci_platforms.push("Travis CI".to_string()); ci_files.push(".travis.yml".to_string()); }
  if path.join(".circleci").join("config.yml").exists() { ci_platforms.push("CircleCI".to_string()); ci_files.push(".circleci/config.yml".to_string()); }
  if path.join("Jenkinsfile").exists() { ci_platforms.push("Jenkins".to_string()); ci_files.push("Jenkinsfile".to_string()); }
  if path.join("azure-pipelines.yml").exists() { ci_platforms.push("Azure Pipelines".to_string()); ci_files.push("azure-pipelines.yml".to_string()); }
  if path.join(".buildkite").exists() { ci_platforms.push("Buildkite".to_string()); ci_files.push(".buildkite/".to_string()); }
  CIInfo { has_ci: !ci_platforms.is_empty(), ci_platforms, ci_files }
}

fn get_package_info(path: &Path) -> PackageInfo {
  let has_package_json = path.join("package.json").exists();
  let has_cargo_toml = path.join("Cargo.toml").exists();
  let has_requirements_txt = path.join("requirements.txt").exists();
  let has_gemfile = path.join("Gemfile").exists();
  let has_go_mod = path.join("go.mod").exists();
  let mut missing = Vec::new();
  for (file, exists) in [
    ("README.md", path.join("README.md").exists()),
    ("LICENSE", path.join("LICENSE").exists()),
    (".gitignore", path.join(".gitignore").exists()),
  ] {
    if !exists { missing.push(file.to_string()); }
  }
  PackageInfo { has_package_json, has_cargo_toml, has_requirements_txt, has_gemfile, has_go_mod, missing_common_files: missing }
}

fn get_testing_info(path: &Path) -> TestingInfo {
  let mut frameworks = Vec::new();
  if let Ok(package_json) = fs::read_to_string(path.join("package.json")) {
    if let Ok(json) = serde_json::from_str::<serde_json::Value>(&package_json) {
      let mut check = |deps: &serde_json::Value| {
        if let Some(obj) = deps.as_object() {
          for (key, _) in obj {
            match key.as_str() {
              "jest" => frameworks.push("Jest".to_string()),
              "vitest" => frameworks.push("Vitest".to_string()),
              "mocha" => frameworks.push("Mocha".to_string()),
              "jasmine" => frameworks.push("Jasmine".to_string()),
              "cypress" => frameworks.push("Cypress".to_string()),
              "playwright" => frameworks.push("Playwright".to_string()),
              _ => {}
            }
          }
        }
      };
      if let Some(deps) = json["dependencies"].as_object() { check(&serde_json::Value::Object(deps.clone())); }
      if let Some(dev) = json["devDependencies"].as_object() { check(&serde_json::Value::Object(dev.clone())); }
    }
  }

  let mut test_file_count = 0usize;
  let mut source_file_count = 0usize;
  let mut patterns: Vec<String> = Vec::new();
  {
    for result in walker_with_depth(path, Some(4)) {
      let entry = match result { Ok(e) => e, Err(_) => continue };
      if entry.file_type().map_or(false, |ft| ft.is_file()) {
        let path_str = entry.path().to_string_lossy();
        let name = entry.path().file_name().unwrap_or_default().to_string_lossy().to_string();
        if name.ends_with(".test.js") || name.ends_with(".test.ts") || name.ends_with(".test.jsx") || name.ends_with(".test.tsx")
          || name.ends_with(".spec.js") || name.ends_with(".spec.ts") || name.ends_with(".spec.jsx") || name.ends_with(".spec.tsx")
          || name.starts_with("test_") && name.ends_with(".py") || name.ends_with("_test.py")
          || name.ends_with("_test.go") || name.ends_with("_spec.rb") || name.ends_with("_test.rb")
          || path_str.contains("/test/") || path_str.contains("/tests/") || path_str.contains("/__tests__/") || path_str.contains("/spec/") {
          test_file_count += 1;
          if !patterns.contains(&name) { patterns.push(name.clone()); }
        } else {
          let is_source = name.ends_with(".js") || name.ends_with(".ts") || name.ends_with(".jsx") || name.ends_with(".tsx") || name.ends_with(".py") || name.ends_with(".rs") || name.ends_with(".go") || name.ends_with(".rb") || name.ends_with(".java") || name.ends_with(".cs") || name.ends_with(".php") || name.ends_with(".cpp") || name.ends_with(".c");
          if is_source { source_file_count += 1; }
        }
      }
    }
  }

  let ratio = if test_file_count > 0 { Some(source_file_count as f64 / test_file_count as f64) } else { None };
  frameworks.sort(); frameworks.dedup();
  patterns.sort(); patterns.dedup(); patterns.truncate(10);

  TestingInfo { has_testing_framework: !frameworks.is_empty(), testing_frameworks: frameworks, has_test_files: test_file_count>0, test_file_count, test_file_patterns: patterns, source_to_test_ratio: ratio }
}

#[tauri::command]
pub async fn get_project_insights(project_path: String) -> Result<ProjectInsights, String> {
  let path = Path::new(&project_path);
  if !path.exists() || !path.is_dir() { return Err("Invalid project path".to_string()); }
  let git_status = get_git_status(path);
  let readme_info = get_readme_info(path);
  let ci_info = get_ci_info(path);
  let package_info = get_package_info(path);
  let testing_info = get_testing_info(path);
  Ok(ProjectInsights { git_status, readme_info, ci_info, package_info, testing_info })
}

#[tauri::command]
pub async fn get_git_log(project_path: String) -> Result<GitLog, String> {
  let path = Path::new(&project_path);
  if !path.exists() || !path.is_dir() {
    return Err("Invalid project path".to_string());
  }

  if !path.join(".git").exists() {
    return Err("Not a git repository".to_string());
  }

  // Get current branch
  let current_branch = Command::new("git")
    .args(&["rev-parse", "--abbrev-ref", "HEAD"])
    .current_dir(path)
    .output()
    .ok()
    .and_then(|output| {
      if output.status.success() {
        String::from_utf8(output.stdout)
          .ok()
          .map(|s| s.trim().to_string())
      } else {
        None
      }
    });

  // Get all branches
  let branches_output = Command::new("git")
    .args(&["branch", "-a"])
    .current_dir(path)
    .output()
    .map_err(|e| format!("Failed to get branches: {}", e))?;

  let branches: Vec<String> = if branches_output.status.success() {
    String::from_utf8_lossy(&branches_output.stdout)
      .lines()
      .map(|line| line.trim().trim_start_matches("* ").to_string())
      .filter(|line| !line.is_empty())
      .collect()
  } else {
    Vec::new()
  };

  // Get git log (last 100 commits)
  let log_output = Command::new("git")
    .args(&[
      "log",
      "-100",
      "--pretty=format:%H%n%an%n%aI%n%s%n---COMMIT-SEPARATOR---"
    ])
    .current_dir(path)
    .output()
    .map_err(|e| format!("Failed to get git log: {}", e))?;

  if !log_output.status.success() {
    return Err("Failed to retrieve git log".to_string());
  }

  let log_text = String::from_utf8_lossy(&log_output.stdout);
  let mut commits = Vec::new();

  for commit_block in log_text.split("---COMMIT-SEPARATOR---") {
    let lines: Vec<&str> = commit_block.trim().lines().collect();
    if lines.len() >= 4 {
      commits.push(GitCommit {
        hash: lines[0].to_string(),
        author: lines[1].to_string(),
        date: lines[2].to_string(),
        message: lines[3..].join("\n"),
      });
    }
  }

  // Get total commit count
  let count_output = Command::new("git")
    .args(&["rev-list", "--count", "HEAD"])
    .current_dir(path)
    .output()
    .ok()
    .and_then(|output| {
      if output.status.success() {
        String::from_utf8(output.stdout)
          .ok()
          .and_then(|s| s.trim().parse::<usize>().ok())
      } else {
        None
      }
    })
    .unwrap_or(commits.len());

  Ok(GitLog {
    commits,
    total_commits: count_output,
    branches,
    current_branch,
  })
}
