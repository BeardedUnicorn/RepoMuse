import React, { useState } from 'react';
import { Settings as SettingsType } from '../types';
import { saveSettings } from '../utils/storage';

interface SettingsProps {
  settings: SettingsType;
  onSettingsUpdated: (settings: SettingsType) => void;
}

const Settings: React.FC<SettingsProps> = ({ settings, onSettingsUpdated }) => {
  const [formData, setFormData] = useState(settings);
  const [isSaving, setIsSaving] = useState(false);
  const [saveStatus, setSaveStatus] = useState<'idle' | 'success' | 'error'>('idle');

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setIsSaving(true);
    setSaveStatus('idle');

    try {
      await saveSettings(formData);
      onSettingsUpdated(formData);
      setSaveStatus('success');
      setTimeout(() => setSaveStatus('idle'), 3000);
    } catch (error) {
      console.error('Error saving settings:', error);
      setSaveStatus('error');
    } finally {
      setIsSaving(false);
    }
  };

  const handleChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    setFormData({
      ...formData,
      [e.target.name]: e.target.value,
    });
  };

  return (
    <div className="max-w-2xl mx-auto">
      <div className="bg-white rounded-lg shadow-md p-8">
        <h2 className="text-2xl font-bold text-gray-900 mb-6">API Settings</h2>
        
        <form onSubmit={handleSubmit} className="space-y-6">
          <div>
            <label htmlFor="api_url" className="block text-sm font-medium text-gray-700 mb-2">
              API Server URL
            </label>
            <input
              type="url"
              id="api_url"
              name="api_url"
              value={formData.api_url}
              onChange={handleChange}
              className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent"
              placeholder="http://localhost:11434/v1/chat/completions"
              required
            />
            <p className="mt-1 text-sm text-gray-500">
              OpenAI-compatible API endpoint (e.g., Ollama, OpenAI, Azure OpenAI)
            </p>
          </div>

          <div>
            <label htmlFor="model" className="block text-sm font-medium text-gray-700 mb-2">
              Model Name
            </label>
            <input
              type="text"
              id="model"
              name="model"
              value={formData.model}
              onChange={handleChange}
              className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent"
              placeholder="llama2, gpt-3.5-turbo, gpt-4, etc."
              required
            />
            <p className="mt-1 text-sm text-gray-500">
              The model to use for generating ideas
            </p>
          </div>

          <div>
            <label htmlFor="api_key" className="block text-sm font-medium text-gray-700 mb-2">
              API Key
            </label>
            <input
              type="password"
              id="api_key"
              name="api_key"
              value={formData.api_key}
              onChange={handleChange}
              className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent"
              placeholder="Your API key (leave empty for local APIs like Ollama)"
            />
            <p className="mt-1 text-sm text-gray-500">
              Leave empty if your API doesn't require authentication
            </p>
          </div>

          <div className="flex items-center justify-between">
            <button
              type="submit"
              disabled={isSaving}
              className="bg-blue-600 text-white px-4 py-2 rounded-md hover:bg-blue-700 disabled:opacity-50 disabled:cursor-not-allowed"
            >
              {isSaving ? 'Saving...' : 'Save Settings'}
            </button>

            {saveStatus === 'success' && (
              <span className="text-green-600 text-sm font-medium">Settings saved successfully!</span>
            )}

            {saveStatus === 'error' && (
              <span className="text-red-600 text-sm font-medium">Error saving settings</span>
            )}
          </div>
        </form>

        <div className="mt-8 p-4 bg-gray-50 rounded-md">
          <h3 className="text-lg font-medium text-gray-900 mb-2">Popular API Configurations</h3>
          <div className="space-y-3 text-sm">
            <div>
              <strong>Ollama (Local):</strong><br />
              URL: http://localhost:11434/v1/chat/completions<br />
              Model: llama2, codellama, or any installed model<br />
              API Key: (leave empty)
            </div>
            <div>
              <strong>OpenAI:</strong><br />
              URL: https://api.openai.com/v1/chat/completions<br />
              Model: gpt-3.5-turbo, gpt-4, etc.<br />
              API Key: Your OpenAI API key
            </div>
          </div>
        </div>
      </div>
    </div>
  );
};

export default Settings;