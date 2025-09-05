export interface Settings {
  api_url: string;
  model: string;
  api_key: string;
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