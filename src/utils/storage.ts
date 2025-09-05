import { invoke } from '@tauri-apps/api/core';
import { Settings } from '../types';

export const saveSettings = async (settings: Settings): Promise<void> => {
  await invoke('save_settings', { settings });
};

export const loadSettings = async (): Promise<Settings> => {
  return await invoke('load_settings');
};