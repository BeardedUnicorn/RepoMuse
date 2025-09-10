import React, { useState, useEffect, useRef, useCallback } from 'react';
import { ProjectDirectory, RepoAnalysis, Settings, ProjectSummary, ProjectInsights, GitLog, TaskList } from '../types';
import { analyzeRepository, analyzeRepositoryFresh, analyzeRepositoryLazy, triggerFullScan, cancelAnalysis, generateIdeaList, generateProjectSummary, saveProjectSummary, loadProjectSummary, getProjectInsights, getGitLog, loadTaskList } from '../utils/api';
import Spinner from './ui/Spinner';
import Alert from './ui/Alert';
import Card from './ui/Card';
import Button from './ui/Button';
import Badge from './ui/Badge';
import StatTile from './ui/StatTile';
import EmptyState from './ui/EmptyState';
import Tabs from './ui/Tabs';
import { FileText, Lightbulb, TrendingUp, GitBranch, Plus, Focus, HardDrive, FileCode, Star } from 'lucide-react';
import MarkdownRenderer from './MarkdownRenderer';
import ProjectInsightsComponent from './ProjectInsights';
import ProjectHeader from './ProjectHeader';
import TaskListComponent, { createTaskFromIdea } from './TaskList';
import { useToast } from './ui/ToastProvider';
import { basename } from '../utils/format';
import { listen } from '@tauri-apps/api/event';
import { openPath } from '@tauri-apps/plugin-opener';

interface ProjectAnalyzerProps {
  selectedProject: ProjectDirectory | null;
  settings: Settings;
}

interface ProgressUpdate {
  folder_path: string;
  phase: string;
  files_discovered: number;
  files_processed: number;
  total_files: number;
  percentage: number;
  current_file: string | null;
  is_complete: boolean;
  is_favorite: boolean;
  elapsed_ms: number;
  estimated_remaining_ms: number | null;
  bytes_processed: number;
  total_bytes: number | null;
  skipped_filtered?: number;
  dirs_seen?: number;
}

// Helper function to format file size
const formatFileSize = (bytes: number): string => {
  if (bytes < 1024) return `${bytes} B`;
  if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(2)} KB`;
  if (bytes < 1024 * 1024 * 1024) return `${(bytes / (1024 * 1024)).toFixed(2)} MB`;
  return `${(bytes / (1024 * 1024 * 1024)).toFixed(2)} GB`;
};

// Helper function to format size for display
const getSizeDisplay = (sizeKb: number, sizeMb: number): string => {
  if (sizeMb >= 1) {
    return `${sizeMb.toFixed(2)} MB`;
  }
  return `${sizeKb.toFixed(2)} KB`;
};

// Helper function to format time
const formatTime = (ms: number): string => {
  if (ms < 1000) return `${ms}ms`;
  if (ms < 60000) return `${(ms / 1000).toFixed(1)}s`;
  const minutes = Math.floor(ms / 60000);
  const seconds = Math.floor((ms % 60000) / 1000);
  return `${minutes}m ${seconds}s`;
};

// Progress Bar Component
const ProgressBar: React.FC<{ progress: ProgressUpdate }> = React.memo(({ progress }) => {
  const getPhaseColor = (phase: string) => {
    switch (phase) {
      case 'cached': return 'bg-purple-500';
      case 'discovery': return 'bg-blue-500';
      case 'lazy': return 'bg-green-500';
      case 'lazy-streaming': return 'bg-green-500';
      case 'processing': return 'bg-primary';
      case 'cancelled': return 'bg-orange-500';
      case 'complete': return 'bg-success';
      default: return 'bg-gray-500';
    }
  };

  const getPhaseLabel = (phase: string) => {
    switch (phase) {
      case 'cached': return '📦 Loaded from cache';
      case 'discovery': return '🔍 Discovering files...';
      case 'lazy': return '⚡ Quick scan...';
      case 'lazy-streaming': return '⚡ Quick scan...';
      case 'processing': return '🔨 Processing files...';
      case 'cancelled': return '⏹️ Cancelled';
      case 'complete': return '✅ Complete';
      default: return phase;
    }
  };

  return (
    <Card className="p-6 mb-6">
      <div className="space-y-4">
        <div className="flex items-center justify-between">
          <div className="flex items-center space-x-3">
            <h3 className="text-lg font-semibold text-foreground">
              {getPhaseLabel(progress.phase)}
            </h3>
            {progress.is_favorite && (
              <Star className="h-4 w-4 text-yellow-500 fill-yellow-500" />
            )}
          </div>
          <span className="text-2xl font-bold text-primary">
            {progress.percentage.toFixed(1)}%
          </span>
        </div>

        {/* Progress Bar */}
        <div className="space-y-2">
          <div className="w-full h-3 bg-background-tertiary rounded-full overflow-hidden">
            <div
              className={`h-full ${getPhaseColor(progress.phase)} transition-all duration-300 ease-out rounded-full`}
              style={{ width: `${Math.min(100, progress.percentage)}%` }}
            >
              <div className="h-full w-full bg-gradient-to-r from-transparent to-white opacity-20 animate-pulse" />
            </div>
          </div>
          
          {/* File Counter */}
          <div className="flex items-center justify-between text-sm text-foreground-secondary">
            <div className="flex items-center space-x-4">
              <span>
                {progress.files_processed.toLocaleString()} / {progress.total_files.toLocaleString()} files
              </span>
              {progress.total_bytes && (
                <span>
                  {formatFileSize(progress.bytes_processed)} / {formatFileSize(progress.total_bytes)}
                </span>
              )}
              <span>
                {(progress.files_processed / Math.max(1, progress.elapsed_ms / 1000)).toFixed(1)} files/s
              </span>
              {progress.bytes_processed > 0 && (
                <span>
                  {((progress.bytes_processed / (1024 * 1024)) / Math.max(1, progress.elapsed_ms / 1000)).toFixed(2)} MB/s
                </span>
              )}
            </div>
            <div className="flex items-center space-x-4">
              <span>Elapsed: {formatTime(progress.elapsed_ms)}</span>
              {progress.estimated_remaining_ms && progress.estimated_remaining_ms > 1000 && (
                <span className="text-foreground-tertiary">
                  Est. {formatTime(progress.estimated_remaining_ms)} remaining
                </span>
              )}
            </div>
          </div>
        </div>

        {/* Current File */}
        {progress.current_file && (
          <div className="mt-3 p-2 bg-background-tertiary rounded-md">
            <div className="flex items-center justify-between">
              <p className="text-xs text-foreground-tertiary truncate" title={progress.current_file}>
                📄 {basename(progress.current_file)}
              </p>
              <Button size="sm" variant="ghost" onClick={() => progress.current_file && openPath(progress.current_file)}>
                Open
              </Button>
            </div>
          </div>
        )}

        {/* Phase-specific info */}
        {progress.phase === 'discovery' && (
          <div className="flex items-center space-x-2 text-sm text-info">
            <span className="animate-pulse">🔍</span>
            <span>Scanning directory structure...</span>
          </div>
        )}

        {/* Controls */}
        {!progress.is_complete && progress.phase !== 'cancelled' && (
          <div className="flex items-center justify-end pt-2">
            <Button variant="ghost" size="sm" onClick={() => cancelAnalysis(progress.folder_path)}>
              Cancel
            </Button>
          </div>
        )}
      </div>
    </Card>
  );
});

const ProjectAnalyzer: React.FC<ProjectAnalyzerProps> = ({ selectedProject, settings }) => {
  const [analysis, setAnalysis] = useState<RepoAnalysis | null>(null);
  const [ideas, setIdeas] = useState<string[]>([]);
  const [summary, setSummary] = useState<ProjectSummary | null>(null);
  const [insights, setInsights] = useState<ProjectInsights | null>(null);
  const [gitLog, setGitLog] = useState<GitLog | null>(null);
  const [taskList, setTaskList] = useState<TaskList | null>(null);
  const [isAnalyzing, setIsAnalyzing] = useState(false);
  const [isGeneratingIdeas, setIsGeneratingIdeas] = useState(false);
  const [isGeneratingSummary, setIsGeneratingSummary] = useState(false);
  const [isLoadingInsights, setIsLoadingInsights] = useState(false);
  const [isLoadingGitLog, setIsLoadingGitLog] = useState(false);
  const [progress, setProgress] = useState<ProgressUpdate | null>(null);
  const [analysisError, setAnalysisError] = useState<string>('');
  const [ideasError, setIdeasError] = useState<string>('');
  const [summaryError, setSummaryError] = useState<string>('');
  const [insightsError, setInsightsError] = useState<string>('');
  const [gitLogError, setGitLogError] = useState<string>('');
  const [focusArea, setFocusArea] = useState<string>('');
  const [generatedWithFocus, setGeneratedWithFocus] = useState<string>('');
  const [taskUpdateTrigger, setTaskUpdateTrigger] = useState(0);
  const [showSizeDetails, setShowSizeDetails] = useState(false);
  const { toast } = useToast();

  // Parse idea metadata from bracketed tags
  type IdeaMeta = {
    coreText: string;
    category?: string;
    affected?: string[];
    impact?: 'H' | 'M' | 'L';
    effort?: 'S' | 'M' | 'L';
    confidence?: number;
  };

  const parseIdeaMeta = (raw: string): IdeaMeta => {
    let coreText = raw;
    const meta: IdeaMeta = { coreText: raw };

    const matchAndStrip = (pattern: RegExp, onMatch: (m: RegExpMatchArray) => void) => {
      const m = coreText.match(pattern);
      if (m) {
        onMatch(m);
        coreText = coreText.replace(m[0], '').trim();
      }
    };

    matchAndStrip(/\[Category:\s*([^\]]+)\]/i, (m) => {
      meta.category = m[1].trim();
    });
    matchAndStrip(/\[Affected:\s*([^\]]+)\]/i, (m) => {
      const inside = m[1];
      const paths = inside.match(/`([^`]+)`/g) || [];
      meta.affected = paths.map((p) => p.replace(/`/g, ''));
    });
    matchAndStrip(/\[Impact:\s*([HML])\]/i, (m) => {
      meta.impact = m[1].toUpperCase() as any;
    });
    matchAndStrip(/\[Effort:\s*([SML])\]/i, (m) => {
      meta.effort = m[1].toUpperCase() as any;
    });
    matchAndStrip(/\[Confidence:\s*(\d{1,3})%\]/i, (m) => {
      meta.confidence = Math.min(100, parseInt(m[1], 10));
    });

    // Clean up repeated spaces
    meta.coreText = coreText.replace(/\s{2,}/g, ' ').trim();
    return meta;
  };

  // Optimized: Batch progress updates
  const pendingProgressUpdates = useRef<ProgressUpdate[]>([]);
  const progressFlushTimeoutRef = useRef<ReturnType<typeof setTimeout> | null>(null);
  
  const flushProgressUpdates = useCallback(() => {
    if (pendingProgressUpdates.current.length > 0) {
      const latestUpdate = pendingProgressUpdates.current[pendingProgressUpdates.current.length - 1];
      setProgress(latestUpdate);
      pendingProgressUpdates.current = [];
    }
  }, []);
  
  const queueProgressUpdate = useCallback((update: ProgressUpdate) => {
    pendingProgressUpdates.current.push(update);
    
    // Clear existing timeout
    if (progressFlushTimeoutRef.current) {
      clearTimeout(progressFlushTimeoutRef.current);
    }
    
    // Set new timeout to flush updates
    progressFlushTimeoutRef.current = setTimeout(() => {
      flushProgressUpdates();
    }, 500); // Increased from 250ms to 500ms
  }, [flushProgressUpdates]);

  useEffect(() => {
    if (selectedProject) {
      setAnalysis(null);
      setProgress(null);
      setIdeas([]);
      setSummary(null);
      setInsights(null);
      setGitLog(null);
      setTaskList(null);
      setAnalysisError('');
      setIdeasError('');
      setSummaryError('');
      setInsightsError('');
      setGitLogError('');
      setFocusArea('');
      setGeneratedWithFocus('');
      setShowSizeDetails(false);
      analyzeProject();
      loadSummary();
      loadInsights();
      loadTasks();
      if (selectedProject.is_git_repo) {
        loadGitLog();
      }
    }
  }, [selectedProject]);

  // Subscribe to progress events with improved throttling
  useEffect(() => {
    let unlisten: (() => void) | null = null;
    const lastUpdateTime = { current: 0 };
    const MIN_UPDATE_INTERVAL = 500; // Increased from 250ms
    
    async function setupListener() {
      try {
        unlisten = await listen<ProgressUpdate>('analysis:progress', (event) => {
          const progressData = event.payload;
          
          // Only update if it's for the current project
          if (selectedProject && progressData.folder_path === selectedProject.path) {
            const now = Date.now();
            
            // More aggressive throttling
            if (now - lastUpdateTime.current < MIN_UPDATE_INTERVAL && !progressData.is_complete) {
              // Queue the update instead of dropping it
              queueProgressUpdate(progressData);
              return;
            }
            
            lastUpdateTime.current = now;
            setProgress(progressData);
            
            // Hide progress when complete after a delay
            if (progressData.is_complete) {
              setTimeout(() => {
                setProgress(null);
              }, 2000);
            }
          }
        });
      } catch (error) {
        console.error('Failed to setup progress listener:', error);
      }
    }
    
    setupListener();
    
    return () => {
      if (unlisten) {
        unlisten();
      }
      // Clear any pending progress updates
      if (progressFlushTimeoutRef.current) {
        clearTimeout(progressFlushTimeoutRef.current);
      }
    };
  }, [selectedProject?.path, queueProgressUpdate]);

  const analyzeProject = async () => {
    if (!selectedProject) return;

    setIsAnalyzing(true);
    setAnalysisError('');
    setProgress(null);

    try {
      // First attempt a lazy scan for quick preview
      const lazyResult = await analyzeRepositoryLazy(selectedProject.path);
      setAnalysis(lazyResult);
      // Stop blocking UI once quick preview is ready
      setIsAnalyzing(false);

      // If the lazy scan shows incomplete results, kick off full scan in background
      if (lazyResult.scan_progress && !lazyResult.scan_progress.is_complete) {
        const est = lazyResult.scan_progress.estimated_total_files ?? 0;
        if (est < 2000) {
          try {
            const fullResult = await triggerFullScan(selectedProject.path);
            setAnalysis(fullResult);
          } catch (e) {
            console.warn('Background full scan failed:', e);
          }
        }
      }
    } catch (err) {
      // Fallback to standard analysis if lazy scan fails
      try {
        const result = await analyzeRepository(selectedProject.path);
        setAnalysis(result);
      } catch (fallbackErr) {
        setAnalysisError(fallbackErr as string);
        toast({ title: 'Failed to analyze project', description: String(fallbackErr), variant: 'error' });
      }
    } finally {
      // Ensure we clear the analyzing state
      setTimeout(() => setIsAnalyzing(false), 500);
    }
  };

  const refreshAnalysis = async () => {
    if (!selectedProject) return;
    setIsAnalyzing(true);
    setAnalysisError('');
    setProgress(null);
    try {
      const result = await analyzeRepositoryFresh(selectedProject.path);
      setAnalysis(result);
      const total = result?.metrics?.total_files ?? 0;
      toast({ title: 'Analysis refreshed', description: `${total} files analyzed`, variant: 'success' });
    } catch (err) {
      setAnalysisError(err as string);
      toast({ title: 'Failed to refresh analysis', description: String(err), variant: 'error' });
    } finally {
      setIsAnalyzing(false);
    }
  };

  const generateIdeas = async () => {
    if (!analysis || !settings.api_url || !settings.model) {
      setIdeasError('Please configure API settings first');
      return;
    }

    setIsGeneratingIdeas(true);
    setIdeasError('');

    try {
      const generatedIdeas = await generateIdeaList({
        analysis,
        settings,
        focus_area: focusArea || undefined,
      });
      setIdeas(generatedIdeas);
      // Store the focus area that was used during generation
      setGeneratedWithFocus(focusArea);
      toast({ title: 'Ideas generated', description: `${generatedIdeas.length} ideas created`, variant: 'success' });
    } catch (err) {
      setIdeasError(err as string);
      toast({ title: 'Failed to generate ideas', description: String(err), variant: 'error' });
    } finally {
      setIsGeneratingIdeas(false);
    }
  };

  const loadSummary = async () => {
    if (!selectedProject) return;
    
    try {
      const loadedSummary = await loadProjectSummary(selectedProject.path);
      if (loadedSummary) {
        setSummary(loadedSummary);
      }
    } catch (err) {
      console.error('Error loading summary:', err);
    }
  };

  const loadInsights = async () => {
    if (!selectedProject) return;
    
    setIsLoadingInsights(true);
    setInsightsError('');
    
    try {
      const projectInsights = await getProjectInsights(selectedProject.path);
      setInsights(projectInsights);
    } catch (err) {
      setInsightsError(err as string);
    } finally {
      setIsLoadingInsights(false);
    }
  };

  const loadGitLog = async () => {
    if (!selectedProject) return;
    
    setIsLoadingGitLog(true);
    setGitLogError('');
    
    try {
      const log = await getGitLog(selectedProject.path);
      setGitLog(log);
    } catch (err) {
      setGitLogError(err as string);
    } finally {
      setIsLoadingGitLog(false);
    }
  };

  const loadTasks = async () => {
    if (!selectedProject) return;
    
    try {
      const tasks = await loadTaskList(selectedProject.path);
      setTaskList(tasks);
    } catch (err) {
      console.error('Error loading tasks:', err);
    }
  };

  const generateSummary = async () => {
    if (!analysis || !settings.api_url || !settings.model || !selectedProject) {
      setSummaryError('Please configure API settings first');
      return;
    }

    setIsGeneratingSummary(true);
    setSummaryError('');

    try {
      const generatedSummary = await generateProjectSummary({ analysis, settings, project_path: selectedProject.path });
      setSummary(generatedSummary);
      // Save the summary for future use
      await saveProjectSummary(generatedSummary);
      toast({ title: 'Summary generated', variant: 'success' });
    } catch (err) {
      setSummaryError(err as string);
      toast({ title: 'Failed to generate summary', description: String(err), variant: 'error' });
    } finally {
      setIsGeneratingSummary(false);
    }
  };

  const addIdeaToTasks = async (idea: string) => {
    if (!selectedProject) return;
    
    try {
      const result = await createTaskFromIdea(selectedProject.path, idea);
      
      if (result.success) {
        toast({ title: 'Task added', description: 'Idea has been added to your task list', variant: 'success' });
        // Trigger task list refresh
        setTaskUpdateTrigger(prev => prev + 1);
        // Reload tasks to update the badge count
        loadTasks();
      } else if (result.isDuplicate) {
        toast({ 
          title: 'Task already exists', 
          description: 'This idea has already been added to your task list', 
          variant: 'info' 
        });
      }
    } catch (error) {
      toast({ title: 'Failed to add task', description: String(error), variant: 'error' });
    }
  };

  if (!selectedProject) {
    return (
      <div className="flex items-center justify-center h-full">
        <EmptyState
          icon={<FileText className="h-16 w-16 text-foreground-tertiary" />}
          title="Select a Project"
          subtitle="Choose a project from the left sidebar to analyze and generate ideas"
        />
      </div>
    );
  }

  const tabs = [
    {
      id: 'summary',
      label: 'Project Summary',
      content: (
        <div>
          <div className="flex justify-between items-center mb-6">
            <div />
            {!isGeneratingSummary && (
              <Button onClick={generateSummary}>
                {summary ? 'Regenerate Summary' : 'Generate Summary'}
              </Button>
            )}
          </div>

          {isGeneratingSummary && (
            <div className="text-center py-8 text-foreground-secondary">
              <Spinner color="blue" />
              <p className="mt-4">Generating AI summary...</p>
              <p className="text-sm text-foreground-tertiary">This may take a moment...</p>
            </div>
          )}

          {summaryError && (
            <>
              <Alert variant="error" title="Summary Error">{summaryError}</Alert>
              <div className="mt-4">
                <Button onClick={generateSummary}>Retry Generation</Button>
              </div>
            </>
          )}

          {!summary && !isGeneratingSummary && !summaryError && (
            <EmptyState
              icon={<Lightbulb className="h-12 w-12 text-foreground-tertiary" />}
              title="No summary generated yet"
              subtitle='Click "Generate Summary" to get an AI-powered overview of this project'
            />
          )}

          {summary && (
            <div className="space-y-4">
              <div className="prose prose-sm max-w-none prose-headings:text-foreground prose-p:text-foreground-secondary prose-strong:text-foreground prose-ul:text-foreground-secondary prose-li:text-foreground-secondary dark:prose-invert">
                <MarkdownRenderer markdown={summary.summary} />
              </div>
              
              {summary.key_features.length > 0 && (
                <div className="mt-4">
                  <h3 className="text-sm font-semibold text-foreground mb-2">Key Features:</h3>
                  <ul className="space-y-1">
                    {summary.key_features.map((feature, index) => (
                      <li key={index} className="flex items-start">
                        <span className="text-primary mr-2">•</span>
                        <span className="text-foreground-secondary text-sm">{feature}</span>
                      </li>
                    ))}
                  </ul>
                </div>
              )}
              
              <div className="mt-4 pt-4 border-t border-border">
                <p className="text-xs text-foreground-tertiary">
                  Generated: {new Date(summary.generated_at).toLocaleString()}
                </p>
              </div>
            </div>
          )}
        </div>
      ),
      badge: summary ? <Badge variant="green" className="ml-2">✓</Badge> : undefined
    },
    {
      id: 'ideas',
      label: 'Development Ideas',
      content: (
        <div>
          <div className="mb-6 space-y-4">
            {/* Focus area input */}
            <div className="flex gap-2">
              <div className="flex-1">
                <div className="relative">
                  <Focus className="absolute left-3 top-1/2 transform -translate-y-1/2 h-4 w-4 text-foreground-tertiary" />
                  <input
                    type="text"
                    value={focusArea}
                    onChange={(e) => setFocusArea(e.target.value)}
                    onKeyPress={(e) => {
                      if (e.key === 'Enter' && !isGeneratingIdeas) {
                        e.preventDefault();
                        generateIdeas();
                      }
                    }}
                    placeholder="Focus area (e.g., documentation, performance, testing, security)..."
                    className="w-full pl-10 pr-3 py-2 border border-border rounded-md focus:outline-none focus:ring-2 focus:ring-primary focus:border-transparent bg-background-secondary text-foreground placeholder-foreground-tertiary"
                  />
                </div>
                <p className="mt-1 text-xs text-foreground-secondary">
                  Optional: Enter a specific area to focus the generated ideas on (press Enter to generate)
                </p>
              </div>
            </div>

            {/* Generate button */}
            <div className="flex justify-between items-center">
              <div />
              {!isGeneratingIdeas && (
                <Button onClick={generateIdeas}>
                  {ideas.length > 0 ? 'Regenerate Ideas' : 'Generate Ideas'}
                </Button>
              )}
            </div>
          </div>

          {isGeneratingIdeas && (
            <div className="text-center py-8 text-foreground-secondary">
              <Spinner color="green" />
              <p className="mt-4">Generating creative ideas...</p>
              <p className="text-sm text-foreground-tertiary">This may take a moment...</p>
            </div>
          )}

          {ideasError && (
            <div className="mb-6">
              <Alert variant="error" title="Generation Error">{ideasError}</Alert>
              <div className="mt-4">
                <Button onClick={generateIdeas}>Retry Generation</Button>
              </div>
            </div>
          )}

          {ideas.length === 0 && !isGeneratingIdeas && !ideasError && (
            <EmptyState
              icon={<Lightbulb className="h-12 w-12 text-foreground-tertiary" />}
              title="No ideas generated yet"
              subtitle='Click "Generate Ideas" to get AI-powered development suggestions'
            />
          )}

          {ideas.length > 0 && (
            <div className="space-y-4">
              {generatedWithFocus && (
                <div className="mb-4 p-3 bg-info/10 dark:bg-info/20 rounded-md">
                  <p className="text-sm text-info">
                    <strong>Focus:</strong> Ideas generated with emphasis on "{generatedWithFocus}"
                  </p>
                </div>
              )}
              {ideas.map((idea, index) => {
                const meta = parseIdeaMeta(idea);
                return (
                <div
                  key={index}
                  className="border-l-4 border-success bg-success/10 dark:bg-success/20 p-4 rounded-r-md group"
                >
                  <div className="flex items-start">
                    <div className="flex-shrink-0">
                      <span className="bg-success text-white rounded-full w-6 h-6 flex items-center justify-center text-sm font-medium">
                        {index + 1}
                      </span>
                    </div>
                    <div className="ml-3 flex-1">
                      <p className="text-foreground whitespace-pre-line">{meta.coreText}</p>
                      {(meta.category || (meta.affected && meta.affected.length) || meta.impact || meta.effort || typeof meta.confidence === 'number') && (
                        <div className="mt-3 flex flex-wrap items-center gap-2 text-xs">
                          {meta.category && <Badge variant="purple">{meta.category}</Badge>}
                          {meta.impact && (
                            <Badge variant={meta.impact === 'H' ? 'red' : meta.impact === 'M' ? 'blue' : 'gray'}>
                              Impact: {meta.impact}
                            </Badge>
                          )}
                          {meta.effort && (
                            <Badge variant={meta.effort === 'S' ? 'green' : meta.effort === 'M' ? 'blue' : 'red'}>
                              Effort: {meta.effort}
                            </Badge>
                          )}
                          {typeof meta.confidence === 'number' && (
                            <Badge variant="gray">Confidence: {meta.confidence}%</Badge>
                          )}
                          {meta.affected && meta.affected.length > 0 && (
                            <span className="text-foreground-tertiary">Affected:</span>
                          )}
                          {meta.affected && meta.affected.map((p, i) => (
                            <span key={i} className="px-1.5 py-0.5 rounded bg-background-tertiary text-foreground-secondary font-mono">
                              {p}
                            </span>
                          ))}
                        </div>
                      )}
                    </div>
                    <button
                      onClick={() => addIdeaToTasks(idea)}
                      className="ml-3 opacity-0 group-hover:opacity-100 transition-opacity p-2 hover:bg-background-tertiary rounded-md text-foreground-secondary hover:text-primary"
                      title="Add to task list"
                    >
                      <Plus className="h-4 w-4" />
                    </button>
                  </div>
                </div>
              );})}
            </div>
          )}
        </div>
      ),
      badge: ideas.length > 0 ? <Badge variant="green" className="ml-2">{ideas.length}</Badge> : undefined
    },
    {
      id: 'tasks',
      label: 'Task List',
      content: (
        <TaskListComponent 
          key={taskUpdateTrigger} 
          projectPath={selectedProject.path} 
        />
      ),
      badge: taskList && taskList.tasks.filter(t => !t.completed).length > 0 
        ? <Badge variant="blue" className="ml-2">{taskList.tasks.filter(t => !t.completed).length}</Badge> 
        : undefined
    },
    {
      id: 'insights',
      label: 'Insights',
      content: (
        <div>
          {isLoadingInsights && (
            <div className="text-center py-8 text-foreground-secondary">
              <Spinner color="blue" />
              <p className="mt-4">Analyzing project health...</p>
            </div>
          )}

          {insightsError && (
            <>
              <Alert variant="error" title="Insights Error">{insightsError}</Alert>
              <div className="mt-4">
                <Button onClick={loadInsights}>Retry Analysis</Button>
              </div>
            </>
          )}

          {insights && (
            <ProjectInsightsComponent insights={insights} />
          )}

          {!insights && !isLoadingInsights && !insightsError && (
            <EmptyState
              icon={<TrendingUp className="h-12 w-12 text-foreground-tertiary" />}
              title="No insights available"
              subtitle="Unable to analyze project health"
            />
          )}
        </div>
      ),
      badge: insights && (!insights.git_status.is_git_repo || insights.git_status.has_uncommitted_changes || !insights.readme_info.exists || insights.readme_info.is_default || !insights.ci_info.has_ci || !insights.testing_info.has_testing_framework || !insights.testing_info.has_test_files || insights.package_info.missing_common_files.length > 0) 
        ? <Badge variant="red" className="ml-2">!</Badge> 
        : insights ? <Badge variant="green" className="ml-2">✓</Badge> : undefined
    }
  ];

  // Add Git History tab if this is a git repository
  if (selectedProject.is_git_repo) {
    tabs.push({
      id: 'git-history',
      label: 'Git History',
      content: (
        <div>
          {isLoadingGitLog && (
            <div className="text-center py-8 text-foreground-secondary">
              <Spinner color="blue" />
              <p className="mt-4">Loading git history...</p>
            </div>
          )}

          {gitLogError && (
            <>
              <Alert variant="error" title="Git Log Error">{gitLogError}</Alert>
              <div className="mt-4">
                <Button onClick={loadGitLog}>Retry</Button>
              </div>
            </>
          )}

          {gitLog && (
            <div className="space-y-6">
              {/* Git Summary */}
              <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
                <StatTile label="Total Commits" value={gitLog.total_commits.toLocaleString()} color="blue" />
                <StatTile label="Recent Commits" value={gitLog.commits.length} color="green" />
                <StatTile label="Branches" value={gitLog.branches.length} color="purple" />
              </div>

              {/* Current Branch */}
              {gitLog.current_branch && (
                <div className="flex items-center space-x-2 text-sm">
                  <GitBranch className="h-4 w-4 text-foreground-secondary" />
                  <span className="text-foreground-secondary">Current branch:</span>
                  <Badge variant="blue">{gitLog.current_branch}</Badge>
                </div>
              )}

              {/* Commit List */}
              <div>
                <h3 className="text-md font-semibold text-foreground mb-4">Recent Commits</h3>
                <div className="space-y-3">
                  {gitLog.commits.map((commit, index) => (
                    <div key={index} className="border-l-2 border-primary/30 pl-4 py-2 hover:bg-background-tertiary rounded-r transition-colors">
                      <div className="flex items-start justify-between">
                        <div className="flex-1 min-w-0">
                          <p className="text-sm font-medium text-foreground">{commit.message}</p>
                          <div className="flex items-center space-x-4 mt-1 text-xs text-foreground-tertiary">
                            <span>{commit.author}</span>
                            <span>•</span>
                            <span>{new Date(commit.date).toLocaleString()}</span>
                          </div>
                        </div>
                        <code className="text-xs bg-background-tertiary px-2 py-1 rounded font-mono text-foreground-secondary ml-4">
                          {commit.hash.substring(0, 7)}
                        </code>
                      </div>
                    </div>
                  ))}
                </div>

                {gitLog.total_commits > gitLog.commits.length && (
                  <div className="mt-4 text-center">
                    <p className="text-sm text-foreground-tertiary">
                      Showing {gitLog.commits.length} of {gitLog.total_commits.toLocaleString()} total commits
                    </p>
                  </div>
                )}
              </div>
            </div>
          )}

          {!gitLog && !isLoadingGitLog && !gitLogError && (
            <EmptyState
              icon={<GitBranch className="h-12 w-12 text-foreground-tertiary" />}
              title="No git history available"
              subtitle="Unable to load git commits"
            />
          )}
        </div>
      ),
      badge: gitLog ? <Badge variant="green" className="ml-2">{gitLog.commits.length}</Badge> : undefined
    });
  }

  return (
    <div className="h-full flex flex-col">
      {/* Project Header */}
      <ProjectHeader
        name={selectedProject.name}
        path={selectedProject.path}
        description={selectedProject.description || null}
        isGitRepo={selectedProject.is_git_repo}
        fileCount={selectedProject.file_count}
      />

      {/* Content Area */}
      <div className="flex-1 overflow-y-auto">
        {/* Progress Indicator - shown during analysis */}
        {(isAnalyzing || progress) && progress && (
          <div className="p-6">
            <ProgressBar progress={progress} />
          </div>
        )}

        {isAnalyzing && !progress && (
          <div className="flex items-center justify-center py-12 text-foreground-secondary">
            <Spinner color="blue" size="md" />
            <p className="ml-4">Analyzing project...</p>
          </div>
        )}

        {analysisError && (
          <div className="p-6">
            <Alert variant="error" title="Analysis Error">{analysisError}</Alert>
            <div className="mt-4">
              <Button variant="primary" onClick={analyzeProject}>Retry Analysis</Button>
            </div>
          </div>
        )}

        {analysis && !isAnalyzing && (
          <div className="p-6">
            {/* Analysis Summary */}
            <Card className="p-6 mb-6">
              <div className="flex justify-between items-center mb-2">
                <h2 className="text-lg font-semibold text-foreground">Project Analysis</h2>
                <Button variant="secondary" onClick={refreshAnalysis}>Refresh Analysis</Button>
              </div>
              {analysis.generated_at && (
                <p className="text-xs text-foreground-tertiary mb-4">
                  Last analyzed: {new Date(analysis.generated_at).toLocaleString()}
                  {analysis.from_cache ? ' (cached)' : ''}
                </p>
              )}
              <div className="grid grid-cols-1 md:grid-cols-4 gap-4 mb-6">
                <StatTile label="Total Files" value={analysis.metrics.total_files} color="blue" />
                <StatTile label="Total Lines" value={analysis.metrics.total_lines?.toLocaleString()} color="green" />
                <StatTile label="Analyzed Files" value={analysis.metrics.analyzed_files} color="purple" />
                <StatTile 
                  label="Project Size" 
                  value={getSizeDisplay(analysis.size_metrics.total_size_kb, analysis.size_metrics.total_size_mb)} 
                  color="gray" 
                />
              </div>

              {/* Size Details Section */}
              <div className="mb-6">
                <button
                  onClick={() => setShowSizeDetails(!showSizeDetails)}
                  className="flex items-center space-x-2 text-sm font-semibold text-foreground hover:text-primary transition-colors"
                >
                  <HardDrive className="h-4 w-4" />
                  <span>Size Details</span>
                  <span className="text-foreground-tertiary">
                    {showSizeDetails ? '▼' : '▶'}
                  </span>
                </button>
                
                {showSizeDetails && (
                  <div className="mt-4 space-y-4">
                    {/* Size breakdown */}
                    <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                      <Card className="p-4">
                        <h4 className="text-sm font-medium text-foreground mb-2">Size Breakdown</h4>
                        <div className="space-y-2 text-sm">
                          <div className="flex justify-between">
                            <span className="text-foreground-secondary">Total Project Size:</span>
                            <span className="font-medium text-foreground">{formatFileSize(analysis.size_metrics.total_size_bytes)}</span>
                          </div>
                          <div className="flex justify-between">
                            <span className="text-foreground-secondary">Analyzed Content:</span>
                            <span className="font-medium text-foreground">{formatFileSize(analysis.size_metrics.analyzed_size_bytes)}</span>
                          </div>
                          <div className="flex justify-between">
                            <span className="text-foreground-secondary">Analysis Coverage:</span>
                            <span className="font-medium text-foreground">
                              {((analysis.size_metrics.analyzed_size_bytes / analysis.size_metrics.total_size_bytes) * 100).toFixed(1)}%
                            </span>
                          </div>
                        </div>
                      </Card>

                      <Card className="p-4">
                        <h4 className="text-sm font-medium text-foreground mb-2">Size by Language</h4>
                        <div className="space-y-1 text-sm max-h-32 overflow-y-auto">
                          {Object.entries(analysis.size_metrics.size_by_language)
                            .sort(([, a], [, b]) => b - a)
                            .slice(0, 5)
                            .map(([lang, size]) => (
                              <div key={lang} className="flex justify-between">
                                <span className="text-foreground-secondary">{lang}:</span>
                                <span className="font-medium text-foreground">{formatFileSize(size)}</span>
                              </div>
                            ))}
                        </div>
                      </Card>
                    </div>

                    {/* Largest files */}
                    {analysis.size_metrics.largest_files.length > 0 && (
                      <Card className="p-4">
                        <h4 className="text-sm font-medium text-foreground mb-3">Largest Files</h4>
                        <div className="space-y-2">
                          {analysis.size_metrics.largest_files.slice(0, 5).map((file, index) => (
                            <div key={index} className="flex items-center justify-between text-sm">
                              <div className="flex items-center space-x-2 flex-1 min-w-0">
                                <FileCode className="h-3 w-3 text-foreground-tertiary flex-shrink-0" />
                                <span className="text-foreground-secondary truncate" title={file.path}>
                                  {basename(file.path)}
                                </span>
                                <Badge variant="gray" className="text-xs">{file.language}</Badge>
                              </div>
                              <span className="font-medium text-foreground ml-2">{formatFileSize(file.size_bytes)}</span>
                            </div>
                          ))}
                        </div>
                      </Card>
                    )}
                  </div>
                )}
              </div>

              <div className="mb-6">
                <h3 className="text-md font-semibold text-foreground mb-3">Technologies Detected</h3>
                <div className="flex flex-wrap gap-2">
                  {analysis.technologies.map((tech) => (
                    <span key={tech} className="bg-background-tertiary text-foreground px-3 py-1 rounded-full text-sm">
                      {tech}
                    </span>
                  ))}
                </div>
              </div>
            </Card>

            {/* Tabbed Interface */}
            <Tabs tabs={tabs} defaultTab="summary" />
          </div>
        )}
      </div>
    </div>
  );
};

export default ProjectAnalyzer;
