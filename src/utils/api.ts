import { invoke } from '@tauri-apps/api/core';
import { RepoAnalysis, IdeaRequest } from '../types';

export const analyzeRepository = async (folderPath: string): Promise<RepoAnalysis> => {
  return await invoke('analyze_repository', { folderPath });
};

export const generateIdeaList = async (request: IdeaRequest): Promise<string[]> => {
  return await invoke('generate_ideas', { request });
};