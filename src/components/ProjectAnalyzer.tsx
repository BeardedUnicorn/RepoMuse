import React, { useState, useEffect } from 'react';
import { ProjectDirectory, RepoAnalysis, Settings, ProjectSummary, ProjectInsights, GitLog, TaskList } from '../types';
import { analyzeRepository, analyzeRepositoryFresh, generateIdeaList, generateProjectSummary, saveProjectSummary, loadProjectSummary, getProjectInsights, getGitLog, loadTaskList } from '../utils/api';
import Spinner from './ui/Spinner';
import Alert from './ui/Alert';
import Card from './ui/Card';
import Button from './ui/Button';
import Badge from './ui/Badge';
import StatTile from './ui/StatTile';
import EmptyState from './ui/EmptyState';
import Tabs from './ui/Tabs';
import { FileText, Lightbulb, TrendingUp, GitBranch, Plus, Focus } from 'lucide-react';
import MarkdownRenderer from './MarkdownRenderer';
import ProjectInsightsComponent from './ProjectInsights';
import ProjectHeader from './ProjectHeader';
import TaskListComponent, { createTaskFromIdea } from './TaskList';
import { useToast } from './ui/ToastProvider';

interface ProjectAnalyzerProps {
  selectedProject: ProjectDirectory | null;
  settings: Settings;
}

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
  const [analysisError, setAnalysisError] = useState<string>('');
  const [ideasError, setIdeasError] = useState<string>('');
  const [summaryError, setSummaryError] = useState<string>('');
  const [insightsError, setInsightsError] = useState<string>('');
  const [gitLogError, setGitLogError] = useState<string>('');
  const [focusArea, setFocusArea] = useState<string>('');
  const [generatedWithFocus, setGeneratedWithFocus] = useState<string>('');
  const [taskUpdateTrigger, setTaskUpdateTrigger] = useState(0);
  const { toast } = useToast();

  useEffect(() => {
    if (selectedProject) {
      setAnalysis(null);
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
      analyzeProject();
      loadSummary();
      loadInsights();
      loadTasks();
      if (selectedProject.is_git_repo) {
        loadGitLog();
      }
    }
  }, [selectedProject]);

  const analyzeProject = async () => {
    if (!selectedProject) return;

    setIsAnalyzing(true);
    setAnalysisError('');

    try {
      const result = await analyzeRepository(selectedProject.path);
      setAnalysis(result);
    } catch (err) {
      setAnalysisError(err as string);
      toast({ title: 'Failed to analyze project', description: String(err), variant: 'error' });
    } finally {
      setIsAnalyzing(false);
    }
  };

  const refreshAnalysis = async () => {
    if (!selectedProject) return;
    setIsAnalyzing(true);
    setAnalysisError('');
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
              {ideas.map((idea, index) => (
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
                      <p className="text-foreground whitespace-pre-line">{idea}</p>
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
              ))}
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
        {isAnalyzing && (
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

        {analysis && (
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
              <div className="grid grid-cols-1 md:grid-cols-3 gap-4 mb-6">
                <StatTile label="Total Files" value={analysis.metrics.total_files} color="blue" />
                <StatTile label="Total Lines" value={analysis.metrics.total_lines?.toLocaleString()} color="green" />
                <StatTile label="Analyzed Files" value={analysis.metrics.analyzed_files} color="purple" />
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