import { invoke } from '@tauri-apps/api/core';
import { RepoAnalysis, IdeaRequest, ModelInfo, ProjectDirectory, ProjectSummary, SummaryRequest, ProjectInsights } from '../types';

export async function listProjectDirectories(rootPath: string): Promise<ProjectDirectory[]> {
  return await invoke('list_project_directories', { rootPath });
}

export async function analyzeRepository(folderPath: string): Promise<RepoAnalysis> {
  return await invoke('analyze_repository', { folderPath });
}

export async function analyzeRepositoryFresh(folderPath: string): Promise<RepoAnalysis> {
  return await invoke('analyze_repository_fresh', { folderPath });
}

export async function generateIdeaList(request: IdeaRequest): Promise<string[]> {
  return await invoke('generate_ideas', { request });
}

export async function loadModels(apiUrl: string, apiKey: string): Promise<ModelInfo[]> {
  return await invoke('load_models', { apiUrl, apiKey });
}

export async function generateProjectSummary(request: SummaryRequest): Promise<ProjectSummary> {
  return await invoke('generate_project_summary', { request });
}

export async function saveProjectSummary(summary: ProjectSummary): Promise<void> {
  return await invoke('save_project_summary', { summary });
}

export async function loadProjectSummary(projectPath: string): Promise<ProjectSummary | null> {
  return await invoke('load_project_summary', { projectPath });
}

export async function saveRootFolder(rootFolder: string): Promise<void> {
  return await invoke('save_root_folder', { rootFolder });
}

export async function loadRootFolder(): Promise<string | null> {
  return await invoke('load_root_folder');
}

export async function getProjectInsights(projectPath: string): Promise<ProjectInsights> {
  return await invoke('get_project_insights', { projectPath });
}
