import React, { useState } from 'react';
import { Settings as SettingsType, ModelInfo } from '../types';
import { saveSettings } from '../utils/storage';
import { loadModels } from '../utils/api';
import Button from './ui/Button';
import Badge from './ui/Badge';
import TextField from './ui/TextField';
import Select from './ui/Select';
import Fieldset from './ui/Fieldset';
import FormRow from './ui/FormRow';
import { isThinkingModel } from '../utils/models';
import { useToast } from './ui/ToastProvider';

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
  const { toast } = useToast();

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setIsSaving(true);
    try {
      await saveSettings(formData);
      onSettingsUpdated(formData);
      toast({ title: 'Settings saved', variant: 'success' });
    } catch (error) {
      console.error('Error saving settings:', error);
      toast({ title: 'Failed to save settings', description: String(error), variant: 'error' });
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
      toast({ title: `Loaded ${availableModels.length} models`, variant: 'success' });
    } catch (error) {
      console.error('Error loading models:', error);
      toast({ title: 'Failed to load models', description: 'Check API URL and key.', variant: 'error' });
    } finally {
      setIsLoadingModels(false);
    }
  };

  return (
    <div className="max-w-2xl mx-auto">
      <div className="bg-background-secondary rounded-lg shadow-md p-8 border border-border">
        <Fieldset title="API Settings">
          <form onSubmit={handleSubmit} className="space-y-6">
            <FormRow>
              <TextField
                label="API Server URL"
                type="url"
                id="api_url"
                name="api_url"
                value={formData.api_url}
                onChange={handleChange}
                placeholder="http://localhost:11434/v1/chat/completions"
                required
                helpText="OpenAI-compatible API endpoint (e.g., Ollama, OpenAI, Azure OpenAI)"
              />
            </FormRow>

            <FormRow>
              <TextField
                label="API Key"
                type="password"
                id="api_key"
                name="api_key"
                value={formData.api_key}
                onChange={handleChange}
                placeholder="Your API key (leave empty for local APIs like Ollama)"
                helpText="Leave empty if your API doesn't require authentication"
              />
            </FormRow>

            <FormRow>
              <div className="flex items-center justify-between mb-2">
                <label htmlFor="model" className="block text-sm font-medium text-foreground">
                  Model
                </label>
                <Button
                  type="button"
                  variant="secondary"
                  size="sm"
                  onClick={handleLoadModels}
                  disabled={isLoadingModels || !formData.api_url}
                  loading={isLoadingModels}
                >
                  Load Models
                </Button>
              </div>

              {modelsLoaded && models.length > 0 ? (
                <Select
                  label="Model"
                  id="model"
                  name="model"
                  value={formData.model}
                  onChange={handleChange}
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
                </Select>
              ) : (
                <TextField
                  label="Model"
                  type="text"
                  id="model"
                  name="model"
                  value={formData.model}
                  onChange={handleChange}
                  placeholder="llama2, gpt-3.5-turbo, gpt-4, o1-mini, etc."
                  required
                />
              )}
              <p className="mt-1 text-sm text-foreground-secondary">
                {isThinkingModel(formData.model)
                  ? 'This is a thinking model - it will use <think></think> tags for reasoning'
                  : "The model to use for generating ideas. Click 'Load Models' to see available options"}
              </p>
            </FormRow>

            <div className="flex items-center justify-between">
              <Button type="submit" variant="primary" loading={isSaving}>Save Settings</Button>
            </div>
          </form>
        </Fieldset>

        {models.length > 0 && (
          <div className="mt-8 p-4 bg-info/10 dark:bg-info/20 rounded-md">
            <h3 className="text-lg font-medium text-foreground mb-2">Available Models ({models.length})</h3>
            <div className="grid gap-2 text-sm max-h-48 overflow-y-auto">
              {models.map((model) => (
                <div key={model.id} className="flex justify-between items-center p-2 bg-background rounded border border-border">
                  <div>
                    <span className="font-medium text-foreground">{model.name || model.id}</span>
                    {isThinkingModel(model.id) && (
                      <Badge variant="purple" className="ml-2">Thinking Model</Badge>
                    )}
                  </div>
                  {model.description && (
                    <span className="text-foreground-tertiary text-xs">{model.description}</span>
                  )}
                </div>
              ))}
            </div>
          </div>
        )}

        <div className="mt-8 p-4 bg-background-tertiary rounded-md">
          <h3 className="text-lg font-medium text-foreground mb-2">Popular API Configurations</h3>
          <div className="space-y-3 text-sm text-foreground-secondary">
            <div>
              <strong className="text-foreground">Ollama (Local):</strong><br />
              URL: http://localhost:11434/v1/chat/completions<br />
              Models: llama2, codellama, deepseek-r1, etc.<br />
              API Key: (leave empty)
            </div>
            <div>
              <strong className="text-foreground">OpenAI:</strong><br />
              URL: https://api.openai.com/v1/chat/completions<br />
              Models: gpt-3.5-turbo, gpt-4, o1-mini, o1-preview<br />
              API Key: Your OpenAI API key
            </div>
            <div>
              <strong className="text-foreground">DeepSeek:</strong><br />
              URL: https://api.deepseek.com/v1/chat/completions<br />
              Models: deepseek-chat, deepseek-r1 (thinking)<br />
              API Key: Your DeepSeek API key
            </div>
          </div>
        </div>

        <div className="mt-6 p-4 bg-purple-100 dark:bg-purple-900/30 rounded-md">
          <h3 className="text-lg font-medium text-purple-900 dark:text-purple-400 mb-2">🧠 Thinking Models</h3>
          <p className="text-sm text-purple-800 dark:text-purple-300">
            Thinking models like OpenAI's o1, o1-mini, or DeepSeek's R1 use special reasoning patterns. 
            They enclose their reasoning in <span className="font-mono bg-purple-200 dark:bg-purple-800/50 px-1 rounded">&lt;think&gt;...&lt;/think&gt;</span> tags, 
            which this app automatically filters out to show only the final response.
          </p>
        </div>
      </div>
    </div>
  );
};

export default Settings;