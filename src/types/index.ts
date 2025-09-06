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
}