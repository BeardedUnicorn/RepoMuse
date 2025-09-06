import React, { useState } from 'react';
import { Settings as SettingsType, ModelInfo } from '../types';
import { saveSettings } from '../utils/storage';
import { loadModels } from '../utils/api';

interface SettingsProps {
  settings: SettingsType;
  onSettingsUpdated: (settings: SettingsType) => void;
}

const Settings: React.FC<SettingsProps> = ({ settings, onSettingsUpdated }) => {
  const [formData, setFormData] = useState(settings);
  const [isSaving, setIsSaving] = useState(false);
  const [isLoadingModels, setIsLoadingModels] = useState(false);
  const [models, setModels] = useState<ModelInfo[]>([]);
  const [modelsLoaded, setModelsLoaded] = useState(false);
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

  const handleChange = (e: React.ChangeEvent<HTMLInputElement | HTMLSelectElement>) => {
    setFormData({
      ...formData,
      [e.target.name]: e.target.value,
    });
  };

  const handleLoadModels = async () => {
    if (!formData.api_url) {
      alert('Please enter an API URL first');
      return;
    }

    setIsLoadingModels(true);
    try {
      const availableModels = await loadModels(formData.api_url, formData.api_key);
      setModels(availableModels);
      setModelsLoaded(true);
    } catch (error) {
      console.error('Error loading models:', error);
      alert('Failed to load models. Please check your API URL and key.');
    } finally {
      setIsLoadingModels(false);
    }
  };

  const isThinkingModel = (modelId: string): boolean => {
    const thinkingModels = ['o1', 'o1-mini', 'o1-preview', 'deepseek-r1'];
    return thinkingModels.some(name => modelId.toLowerCase().includes(name));
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

          <div>
            <div className="flex items-center justify-between mb-2">
              <label htmlFor="model" className="block text-sm font-medium text-gray-700">
                Model
              </label>
              <button
                type="button"
                onClick={handleLoadModels}
                disabled={isLoadingModels || !formData.api_url}
                className="text-sm bg-gray-100 hover:bg-gray-200 px-3 py-1 rounded-md disabled:opacity-50 disabled:cursor-not-allowed"
              >
                {isLoadingModels ? 'Loading...' : 'Load Models'}
              </button>
            </div>
            
            {modelsLoaded && models.length > 0 ? (
              <select
                id="model"
                name="model"
                value={formData.model}
                onChange={handleChange}
                className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent"
                required
              >
                <option value="">Select a model...</option>
                {models.map((model) => (
                  <option key={model.id} value={model.id}>
                    {model.name || model.id}
                    {isThinkingModel(model.id) && ' (Thinking Model)'}
                    {model.description && ` - ${model.description}`}
                  </option>
                ))}
              </select>
            ) : (
              <input
                type="text"
                id="model"
                name="model"
                value={formData.model}
                onChange={handleChange}
                className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent"
                placeholder="llama2, gpt-3.5-turbo, gpt-4, o1-mini, etc."
                required
              />
            )}
            
            <p className="mt-1 text-sm text-gray-500">
              {isThinkingModel(formData.model) 
                ? "This is a thinking model - it will use <think></think> tags for reasoning"
                : "The model to use for generating ideas. Click 'Load Models' to see available options"
              }
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

        {models.length > 0 && (
          <div className="mt-8 p-4 bg-blue-50 rounded-md">
            <h3 className="text-lg font-medium text-blue-900 mb-2">Available Models ({models.length})</h3>
            <div className="grid gap-2 text-sm max-h-48 overflow-y-auto">
              {models.map((model) => (
                <div key={model.id} className="flex justify-between items-center p-2 bg-white rounded border">
                  <div>
                    <span className="font-medium">{model.name || model.id}</span>
                    {isThinkingModel(model.id) && (
                      <span className="ml-2 inline-flex items-center px-2 py-0.5 rounded text-xs font-medium bg-purple-100 text-purple-800">
                        Thinking Model
                      </span>
                    )}
                  </div>
                  {model.description && (
                    <span className="text-gray-500 text-xs">{model.description}</span>
                  )}
                </div>
              ))}
            </div>
          </div>
        )}

        <div className="mt-8 p-4 bg-gray-50 rounded-md">
          <h3 className="text-lg font-medium text-gray-900 mb-2">Popular API Configurations</h3>
          <div className="space-y-3 text-sm">
            <div>
              <strong>Ollama (Local):</strong><br />
              URL: http://localhost:11434/v1/chat/completions<br />
              Models: llama2, codellama, deepseek-r1, etc.<br />
              API Key: (leave empty)
            </div>
            <div>
              <strong>OpenAI:</strong><br />
              URL: https://api.openai.com/v1/chat/completions<br />
              Models: gpt-3.5-turbo, gpt-4, o1-mini, o1-preview<br />
              API Key: Your OpenAI API key
            </div>
            <div>
              <strong>DeepSeek:</strong><br />
              URL: https://api.deepseek.com/v1/chat/completions<br />
              Models: deepseek-chat, deepseek-r1 (thinking)<br />
              API Key: Your DeepSeek API key
            </div>
          </div>
        </div>

        <div className="mt-6 p-4 bg-purple-50 rounded-md">
          <h3 className="text-lg font-medium text-purple-900 mb-2">🧠 Thinking Models</h3>
          <p className="text-sm text-purple-800">
            Thinking models like OpenAI's o1, o1-mini, or DeepSeek's R1 use special reasoning patterns. 
            They enclose their reasoning in <span className="font-mono bg-purple-100 px-1 rounded">&lt;think&gt;...&lt;/think&gt;</span> tags, 
            which this app automatically filters out to show only the final response.
          </p>
        </div>
      </div>
    </div>
  );
};

export default Settings;