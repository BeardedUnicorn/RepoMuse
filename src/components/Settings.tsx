import React, { useState, useEffect } from 'react';
import { Settings as SettingsType, ModelInfo } from '../types';
import { saveSettings } from '../utils/storage';
import { loadModels } from '../utils/api';
import { 
  openAppDataDirectory, 
  getDatabaseStats, 
  vacuumDatabase, 
  clearExpiredCache, 
  optimizeDatabase,
  formatBytes,
  DatabaseStats 
} from '../utils/db-utils';
import Button from './ui/Button';
import TextField from './ui/TextField';
import Select from './ui/Select';
import Fieldset from './ui/Fieldset';
import FormRow from './ui/FormRow';
import Card from './ui/Card';
import StatTile from './ui/StatTile';
import { isThinkingModel } from '../utils/models';
import { useToast } from './ui/ToastProvider';
import { Database, HardDrive, Zap, Trash2, FolderOpen, RefreshCw } from 'lucide-react';

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
  const [dbStats, setDbStats] = useState<DatabaseStats | null>(null);
  const [isLoadingStats, setIsLoadingStats] = useState(false);
  const [isPerformingMaintenance, setIsPerformingMaintenance] = useState(false);
  const { toast } = useToast();

  useEffect(() => {
    loadDatabaseStats();
  }, []);

  const loadDatabaseStats = async () => {
    setIsLoadingStats(true);
    try {
      const stats = await getDatabaseStats();
      setDbStats(stats);
    } catch (error) {
      console.error('Error loading database stats:', error);
      toast({ title: 'Failed to load database statistics', variant: 'error' });
    } finally {
      setIsLoadingStats(false);
    }
  };

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

  const numberFields = new Set([
    'temperature_ideas',
    'frequency_penalty_ideas',
    'presence_penalty_ideas',
    'max_tokens_ideas',
    'temperature_summary',
    'presence_penalty_summary',
    'max_tokens_summary',
  ]);

  const handleChange = (e: React.ChangeEvent<HTMLInputElement | HTMLSelectElement>) => {
    const { name, value, type } = e.target;
    let newValue: any = value;
    if (numberFields.has(name)) {
      newValue = type === 'number' ? (name.includes('tokens') ? parseInt(value || '0', 10) : parseFloat(value || '0')) : value;
      if (Number.isNaN(newValue)) newValue = 0;
    }
    setFormData({
      ...formData,
      [name]: newValue,
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

  const handleOpenDataDirectory = async () => {
    try {
      await openAppDataDirectory();
      toast({ title: 'Opened app data directory', variant: 'success' });
    } catch (error) {
      console.error('Error opening directory:', error);
      toast({ title: 'Failed to open directory', variant: 'error' });
    }
  };

  const handleVacuumDatabase = async () => {
    setIsPerformingMaintenance(true);
    try {
      const result = await vacuumDatabase();
      toast({ title: 'Database vacuumed', description: result, variant: 'success' });
      await loadDatabaseStats();
    } catch (error) {
      console.error('Error vacuuming database:', error);
      toast({ title: 'Vacuum failed', description: String(error), variant: 'error' });
    } finally {
      setIsPerformingMaintenance(false);
    }
  };

  const handleClearExpiredCache = async () => {
    setIsPerformingMaintenance(true);
    try {
      const result = await clearExpiredCache();
      toast({ title: 'Cache cleared', description: result, variant: 'success' });
      await loadDatabaseStats();
    } catch (error) {
      console.error('Error clearing cache:', error);
      toast({ title: 'Failed to clear cache', variant: 'error' });
    } finally {
      setIsPerformingMaintenance(false);
    }
  };

  const handleOptimizeDatabase = async () => {
    setIsPerformingMaintenance(true);
    try {
      const result = await optimizeDatabase();
      toast({ title: 'Database optimized', description: result, variant: 'success' });
      await loadDatabaseStats();
    } catch (error) {
      console.error('Error optimizing database:', error);
      toast({ title: 'Optimization failed', variant: 'error' });
    } finally {
      setIsPerformingMaintenance(false);
    }
  };

  return (
    <div className="max-w-4xl mx-auto space-y-8">
      {/* API Settings */}
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
            </FormRow>

            <div className="flex items-center justify-between">
              <Button type="submit" variant="primary" loading={isSaving}>Save Settings</Button>
            </div>
          </form>
        </Fieldset>

        {/* Model Parameters */}
        <Fieldset title="Model Parameters" description="Tune generation behavior for ideas and summaries.">
          <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
            <div className="space-y-4">
              <h3 className="text-md font-semibold text-foreground">Idea Generation</h3>
              <FormRow>
                <TextField
                  label="Temperature"
                  type="number"
                  step="0.1"
                  min={0}
                  max={2}
                  id="temperature_ideas"
                  name="temperature_ideas"
                  value={formData.temperature_ideas}
                  onChange={handleChange}
                  placeholder="0.6"
                />
                <TextField
                  label="Frequency Penalty"
                  type="number"
                  step="0.1"
                  min={0}
                  max={2}
                  id="frequency_penalty_ideas"
                  name="frequency_penalty_ideas"
                  value={formData.frequency_penalty_ideas}
                  onChange={handleChange}
                  placeholder="0.3"
                />
              </FormRow>
              <FormRow>
                <TextField
                  label="Presence Penalty"
                  type="number"
                  step="0.1"
                  min={0}
                  max={2}
                  id="presence_penalty_ideas"
                  name="presence_penalty_ideas"
                  value={formData.presence_penalty_ideas}
                  onChange={handleChange}
                  placeholder="0.1"
                />
                <TextField
                  label="Max Tokens"
                  type="number"
                  step="1"
                  min={256}
                  max={32768}
                  id="max_tokens_ideas"
                  name="max_tokens_ideas"
                  value={formData.max_tokens_ideas}
                  onChange={handleChange}
                  placeholder="1500"
                />
              </FormRow>
              <div className="flex items-center space-x-2">
                <input
                  type="checkbox"
                  id="use_stop_ideas"
                  name="use_stop_ideas"
                  checked={Boolean(formData.use_stop_ideas)}
                  onChange={(e) => setFormData({ ...formData, use_stop_ideas: e.target.checked })}
                  className="h-4 w-4 rounded border-border text-primary focus:ring-primary"
                />
                <label htmlFor="use_stop_ideas" className="text-sm text-foreground">
                  Stop after 10 items (send stop sequence)
                </label>
              </div>
            </div>

            <div className="space-y-4">
              <h3 className="text-md font-semibold text-foreground">Project Summary</h3>
              <FormRow>
                <TextField
                  label="Temperature"
                  type="number"
                  step="0.1"
                  min={0}
                  max={2}
                  id="temperature_summary"
                  name="temperature_summary"
                  value={formData.temperature_summary}
                  onChange={handleChange}
                  placeholder="0.4"
                />
                <TextField
                  label="Presence Penalty"
                  type="number"
                  step="0.1"
                  min={0}
                  max={2}
                  id="presence_penalty_summary"
                  name="presence_penalty_summary"
                  value={formData.presence_penalty_summary}
                  onChange={handleChange}
                  placeholder="0.1"
                />
              </FormRow>
              <FormRow>
                <TextField
                  label="Max Tokens"
                  type="number"
                  step="1"
                  min={256}
                  max={32768}
                  id="max_tokens_summary"
                  name="max_tokens_summary"
                  value={formData.max_tokens_summary}
                  onChange={handleChange}
                  placeholder="1200"
                />
              </FormRow>
            </div>
          </div>

          
        </Fieldset>
      </div>

      {/* Database Statistics */}
      <div className="bg-background-secondary rounded-lg shadow-md p-8 border border-border">
        <div className="flex items-center justify-between mb-6">
          <div className="flex items-center space-x-2">
            <Database className="h-5 w-5 text-foreground-secondary" />
            <h2 className="text-lg font-semibold text-foreground">Database Statistics</h2>
          </div>
          <Button
            variant="ghost"
            size="sm"
            onClick={loadDatabaseStats}
            disabled={isLoadingStats}
          >
            <RefreshCw className={`h-4 w-4 ${isLoadingStats ? 'animate-spin' : ''}`} />
          </Button>
        </div>

        {dbStats ? (
          <div className="space-y-6">
            <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
              <StatTile 
                label="Projects" 
                value={dbStats.total_projects.toLocaleString()} 
                color="blue" 
              />
              <StatTile 
                label="Files Indexed" 
                value={dbStats.total_files.toLocaleString()} 
                color="green" 
              />
              <StatTile 
                label="Database Size" 
                value={`${dbStats.database_size_mb.toFixed(2)} MB`} 
                color="purple" 
              />
            </div>

            <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
              <Card className="p-4">
                <h4 className="text-sm font-medium text-foreground-secondary mb-2">Cached Analyses</h4>
                <p className="text-2xl font-bold text-foreground">{dbStats.cached_analyses}</p>
              </Card>
              <Card className="p-4">
                <h4 className="text-sm font-medium text-foreground-secondary mb-2">Tasks</h4>
                <p className="text-2xl font-bold text-foreground">{dbStats.total_tasks}</p>
              </Card>
              <Card className="p-4">
                <h4 className="text-sm font-medium text-foreground-secondary mb-2">Summaries</h4>
                <p className="text-2xl font-bold text-foreground">{dbStats.total_summaries}</p>
              </Card>
            </div>

            <Card className="p-4 bg-info/10 dark:bg-info/20">
              <div className="flex items-start space-x-3">
                <HardDrive className="h-5 w-5 text-info flex-shrink-0 mt-0.5" />
                <div className="flex-1">
                  <h4 className="text-sm font-medium text-info mb-1">Storage Details</h4>
                  <div className="text-sm text-info/80 space-y-1">
                    <p>Total content indexed: {formatBytes(dbStats.total_size_bytes)}</p>
                    <p>Database file size: {formatBytes(dbStats.database_size_bytes)}</p>
                    <p>Compression ratio: {((dbStats.total_size_bytes / Math.max(1, dbStats.database_size_bytes)) * 100).toFixed(1)}%</p>
                  </div>
                </div>
              </div>
            </Card>
          </div>
        ) : (
          <div className="text-center py-8 text-foreground-secondary">
            Loading statistics...
          </div>
        )}
      </div>

      {/* Database Maintenance */}
      <div className="bg-background-secondary rounded-lg shadow-md p-8 border border-border">
        <div className="flex items-center space-x-2 mb-6">
          <Zap className="h-5 w-5 text-foreground-secondary" />
          <h2 className="text-lg font-semibold text-foreground">Database Maintenance</h2>
        </div>

        <div className="space-y-4">
          <div className="flex items-center justify-between p-4 bg-background-tertiary rounded-md">
            <div>
              <h4 className="font-medium text-foreground">Clear Expired Cache</h4>
              <p className="text-sm text-foreground-secondary mt-1">
                Remove analysis cache entries that have expired
              </p>
            </div>
            <Button
              variant="secondary"
              onClick={handleClearExpiredCache}
              disabled={isPerformingMaintenance}
            >
              <Trash2 className="h-4 w-4 mr-2" />
              Clear
            </Button>
          </div>

          <div className="flex items-center justify-between p-4 bg-background-tertiary rounded-md">
            <div>
              <h4 className="font-medium text-foreground">Optimize Database</h4>
              <p className="text-sm text-foreground-secondary mt-1">
                Update statistics and optimize query performance
              </p>
            </div>
            <Button
              variant="secondary"
              onClick={handleOptimizeDatabase}
              disabled={isPerformingMaintenance}
            >
              <Zap className="h-4 w-4 mr-2" />
              Optimize
            </Button>
          </div>

          <div className="flex items-center justify-between p-4 bg-background-tertiary rounded-md">
            <div>
              <h4 className="font-medium text-foreground">Vacuum Database</h4>
              <p className="text-sm text-foreground-secondary mt-1">
                Reclaim unused space and defragment the database file
              </p>
            </div>
            <Button
              variant="secondary"
              onClick={handleVacuumDatabase}
              disabled={isPerformingMaintenance}
            >
              <Database className="h-4 w-4 mr-2" />
              Vacuum
            </Button>
          </div>

          <div className="flex items-center justify-between p-4 bg-background-tertiary rounded-md">
            <div>
              <h4 className="font-medium text-foreground">Open Data Directory</h4>
              <p className="text-sm text-foreground-secondary mt-1">
                View database and application files in your file explorer
              </p>
            </div>
            <Button
              variant="secondary"
              onClick={handleOpenDataDirectory}
            >
              <FolderOpen className="h-4 w-4 mr-2" />
              Open
            </Button>
          </div>
        </div>
      </div>

      {/* API Configuration Examples */}
      <div className="bg-background-tertiary rounded-md p-6">
        <h3 className="text-lg font-medium text-foreground mb-4">Popular API Configurations</h3>
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
    </div>
  );
};

export default Settings;
