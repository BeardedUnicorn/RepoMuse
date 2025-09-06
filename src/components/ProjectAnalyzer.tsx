import React, { useState, useEffect } from 'react';
import { ProjectDirectory, RepoAnalysis, Settings, ProjectSummary, ProjectInsights } from '../types';
import { analyzeRepository, analyzeRepositoryFresh, generateIdeaList, generateProjectSummary, saveProjectSummary, loadProjectSummary, getProjectInsights } from '../utils/api';
import Spinner from './ui/Spinner';
import Alert from './ui/Alert';
import Card from './ui/Card';
import Button from './ui/Button';
import Badge from './ui/Badge';
import StatTile from './ui/StatTile';
import EmptyState from './ui/EmptyState';
import Tabs from './ui/Tabs';
import { FileText, Lightbulb, TrendingUp } from 'lucide-react';
import MarkdownRenderer from './MarkdownRenderer';
import ProjectInsightsComponent from './ProjectInsights';
import ProjectHeader from './ProjectHeader';
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
  const [isAnalyzing, setIsAnalyzing] = useState(false);
  const [isGeneratingIdeas, setIsGeneratingIdeas] = useState(false);
  const [isGeneratingSummary, setIsGeneratingSummary] = useState(false);
  const [isLoadingInsights, setIsLoadingInsights] = useState(false);
  const [analysisError, setAnalysisError] = useState<string>('');
  const [ideasError, setIdeasError] = useState<string>('');
  const [summaryError, setSummaryError] = useState<string>('');
  const [insightsError, setInsightsError] = useState<string>('');
  const { toast } = useToast();

  useEffect(() => {
    if (selectedProject) {
      setAnalysis(null);
      setIdeas([]);
      setSummary(null);
      setInsights(null);
      setAnalysisError('');
      setIdeasError('');
      setSummaryError('');
      setInsightsError('');
      analyzeProject();
      loadSummary();
      loadInsights();
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
      });
      setIdeas(generatedIdeas);
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
            <Tabs
              tabs={[
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
                      <div className="flex justify-between items-center mb-6">
                        <div />
                        {!isGeneratingIdeas && (
                          <Button onClick={generateIdeas}>
                            {ideas.length > 0 ? 'Regenerate Ideas' : 'Generate Ideas'}
                          </Button>
                        )}
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
                          {ideas.map((idea, index) => (
                            <div
                              key={index}
                              className="border-l-4 border-success bg-success/10 dark:bg-success/20 p-4 rounded-r-md"
                            >
                              <div className="flex items-start">
                                <div className="flex-shrink-0">
                                  <span className="bg-success text-white rounded-full w-6 h-6 flex items-center justify-center text-sm font-medium">
                                    {index + 1}
                                  </span>
                                </div>
                                <div className="ml-3">
                                  <p className="text-foreground whitespace-pre-line">{idea}</p>
                                </div>
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
              ]}
              defaultTab="summary"
            />
          </div>
        )}
      </div>
    </div>
  );
};

export default ProjectAnalyzer;