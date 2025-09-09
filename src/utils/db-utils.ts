import { invoke } from '@tauri-apps/api/core';
import { openPath } from '@tauri-apps/plugin-opener';

export interface DatabaseStats {
  total_projects: number;
  total_files: number;
  total_size_bytes: number;
  cached_analyses: number;
  total_tasks: number;
  total_summaries: number;
  database_size_bytes: number;
  database_size_mb: number;
}

export async function getAppDataDirectory(): Promise<string> {
  return await invoke('get_app_data_directory');
}

export async function openAppDataDirectory(): Promise<void> {
  const dir = await getAppDataDirectory();
  await openPath(dir);
}

export async function getDatabaseStats(): Promise<DatabaseStats> {
  return await invoke('get_database_stats');
}

export async function vacuumDatabase(): Promise<string> {
  return await invoke('vacuum_database');
}

export async function clearExpiredCache(): Promise<string> {
  return await invoke('clear_expired_cache');
}

export async function optimizeDatabase(): Promise<string> {
  return await invoke('optimize_database');
}

export async function clearAllData(): Promise<void> {
  return await invoke('clear_all_data');
}

// Helper function to format bytes
export function formatBytes(bytes: number): string {
  if (bytes === 0) return '0 Bytes';
  const k = 1024;
  const sizes = ['Bytes', 'KB', 'MB', 'GB'];
  const i = Math.floor(Math.log(bytes) / Math.log(k));
  return Math.round(bytes / Math.pow(k, i) * 100) / 100 + ' ' + sizes[i];
}
