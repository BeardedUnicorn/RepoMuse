export interface Settings {
  api_url: string;
  model: string;
  api_key: string;
  // Idea generation parameters
  temperature_ideas: number;
  frequency_penalty_ideas: number;
  presence_penalty_ideas: number;
  max_tokens_ideas: number;
  // Summary generation parameters
  temperature_summary: number;
  presence_penalty_summary: number;
  max_tokens_summary: number;
  use_stop_ideas: boolean;
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

export interface FileSizeInfo {
  path: string;
  size_bytes: number;
  size_kb: number;
  language: string;
}

export interface SizeMetrics {
  total_size_bytes: number;
  total_size_kb: number;
  total_size_mb: number;
  analyzed_size_bytes: number;
  analyzed_size_kb: number;
  analyzed_size_mb: number;
  largest_files: FileSizeInfo[];
  size_by_language: Record<string, number>;
}

export interface ScanProgress {
  files_scanned: number;
  scan_limit: number;
  is_complete: boolean;
  estimated_total_files?: number;
}

export interface RepoAnalysis {
  files: FileInfo[];
  structure: Record<string, string[]>;
  technologies: string[];
  metrics: Record<string, number>;
  size_metrics: SizeMetrics;
  generated_at?: string;
  from_cache?: boolean;
  is_lazy_scan?: boolean;
  scan_progress?: ScanProgress;
}

export interface IdeaRequest {
  analysis: RepoAnalysis;
  settings: Settings;
  focus_area?: string;
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

export interface GitRemote {
  name: string;
  url: string;
}

export interface GitStatus {
  is_git_repo: boolean;
  has_uncommitted_changes: boolean;
  uncommitted_files: string[];
  current_branch?: string;
  last_commit_date?: string;
  commit_count?: number;
  remotes: GitRemote[];
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

export interface GitCommit {
  hash: string;
  author: string;
  date: string;
  message: string;
}

export interface GitLog {
  commits: GitCommit[];
  total_commits: number;
  branches: string[];
  current_branch?: string;
}

export interface Task {
  id: string;
  text: string;
  completed: boolean;
  created_at: string;
  completed_at?: string;
}

export interface TaskList {
  project_path: string;
  tasks: Task[];
  updated_at: string;
}
