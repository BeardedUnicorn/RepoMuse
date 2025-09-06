export interface Settings {
  api_url: string;
  model: string;
  api_key: string;
}

export interface ModelInfo {
  id: string;
  name?: string;
  description?: string;
}

export interface FileInfo {
  path: string;
  content: string;
  language: string;
  size: number;
}

export interface RepoAnalysis {
  files: FileInfo[];
  structure: Record<string, string[]>;
  technologies: string[];
  metrics: Record<string, number>;
  generated_at?: string;
  from_cache?: boolean;
}

export interface IdeaRequest {
  analysis: RepoAnalysis;
  settings: Settings;
}

export interface ProjectDirectory {
  name: string;
  path: string;
  is_git_repo: boolean;
  file_count: number;
  description?: string;
  is_counting: boolean;
}

export interface ProjectSummary {
  project_path: string;
  summary: string;
  generated_at: string;
  technologies: string[];
  key_features: string[];
}

export interface SummaryRequest {
  analysis: RepoAnalysis;
  settings: Settings;
  project_path?: string;
}

export interface GitStatus {
  is_git_repo: boolean;
  has_uncommitted_changes: boolean;
  uncommitted_files: string[];
  current_branch?: string;
  last_commit_date?: string;
  commit_count?: number;
}

export interface ReadmeInfo {
  exists: boolean;
  is_default: boolean;
  path?: string;
  content_preview?: string;
}

export interface CIInfo {
  has_ci: boolean;
  ci_platforms: string[];
  ci_files: string[];
}

export interface PackageInfo {
  has_package_json: boolean;
  has_cargo_toml: boolean;
  has_requirements_txt: boolean;
  has_gemfile: boolean;
  has_go_mod: boolean;
  missing_common_files: string[];
}

export interface TestingInfo {
  has_testing_framework: boolean;
  testing_frameworks: string[];
  has_test_files: boolean;
  test_file_count: number;
  test_file_patterns: string[];
  source_to_test_ratio?: number;
}

export interface ProjectInsights {
  git_status: GitStatus;
  readme_info: ReadmeInfo;
  ci_info: CIInfo;
  package_info: PackageInfo;
  testing_info: TestingInfo;
}
