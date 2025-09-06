import React, { useState, useEffect } from 'react';
import { ProjectDirectory, RepoAnalysis, Settings, ProjectSummary } from '../types';
import { analyzeRepository, generateIdeaList, generateProjectSummary, saveProjectSummary, loadProjectSummary } from '../utils/api';

// Simple markdown renderer
const renderMarkdown = (text: string): string => {
  let html = text;
  
  // Convert headers
  html = html.replace(/^### (.*$)/gim, '<h3>$1</h3>');
  html = html.replace(/^## (.*$)/gim, '<h2>$1</h2>');
  html = html.replace(/^# (.*$)/gim, '<h1>$1</h1>');
  
  // Convert bold text
  html = html.replace(/\*\*(.*?)\*\*/g, '<strong>$1</strong>');
  html = html.replace(/__(.*?)__/g, '<strong>$1</strong>');
  
  // Convert italic text
  html = html.replace(/\*(.*?)\*/g, '<em>$1</em>');
  html = html.replace(/_(.*?)_/g, '<em>$1</em>');
  
  // Convert inline code
  html = html.replace(/`(.*?)`/g, '<code class="bg-gray-100 px-1 py-0.5 rounded text-sm font-mono">$1</code>');
  
  // Convert unordered lists
  html = html.replace(/^\s*\* (.*$)/gim, '<li>$1</li>');
  html = html.replace(/^\s*- (.*$)/gim, '<li>$1</li>');
  html = html.replace(/^\s*\+ (.*$)/gim, '<li>$1</li>');
  
  // Convert ordered lists
  html = html.replace(/^\s*\d+\.\s+(.*$)/gim, '<li>$1</li>');
  
  // Wrap consecutive list items in ul tags
  html = html.replace(/(<li>.*<\/li>)/gs, (match) => {
    return `<ul class="list-disc list-inside space-y-1 my-2">${match}</ul>`;
  });
  
  // Fix multiple ul tags
  html = html.replace(/<\/ul>\s*<ul[^>]*>/g, '');
  
  // Convert line breaks to paragraphs
  html = html.replace(/\n\n/g, '</p><p>');
  html = `<p>${html}</p>`;
  
  // Clean up empty paragraphs
  html = html.replace(/<p><\/p>/g, '');
  html = html.replace(/<p>\s*<\/p>/g, '');
  
  // Fix paragraphs that contain headers or lists
  html = html.replace(/<p>(<h[1-6]>.*?<\/h[1-6]>)<\/p>/g, '$1');
  html = html.replace(/<p>(<ul.*?<\/ul>)<\/p>/gs, '$1');
  
  return html;
};

interface ProjectAnalyzerProps {
  selectedProject: ProjectDirectory | null;
  settings: Settings;
}

const ProjectAnalyzer: React.FC<ProjectAnalyzerProps> = ({ selectedProject, settings }) => {
  const [analysis, setAnalysis] = useState<RepoAnalysis | null>(null);
  const [ideas, setIdeas] = useState<string[]>([]);
  const [summary, setSummary] = useState<ProjectSummary | null>(null);
  const [isAnalyzing, setIsAnalyzing] = useState(false);
  const [isGeneratingIdeas, setIsGeneratingIdeas] = useState(false);
  const [isGeneratingSummary, setIsGeneratingSummary] = useState(false);
  const [analysisError, setAnalysisError] = useState<string>('');
  const [ideasError, setIdeasError] = useState<string>('');
  const [summaryError, setSummaryError] = useState<string>('');

  useEffect(() => {
    if (selectedProject) {
      setAnalysis(null);
      setIdeas([]);
      setSummary(null);
      setAnalysisError('');
      setIdeasError('');
      setSummaryError('');
      analyzeProject();
      loadSummary();
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
    } catch (err) {
      setIdeasError(err as string);
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

  const generateSummary = async () => {
    if (!analysis || !settings.api_url || !settings.model) {
      setSummaryError('Please configure API settings first');
      return;
    }

    setIsGeneratingSummary(true);
    setSummaryError('');

    try {
      const generatedSummary = await generateProjectSummary({
        analysis,
        settings,
        project_path: selectedProject.path,
      });
      setSummary(generatedSummary);
      // Save the summary for future use
      await saveProjectSummary(generatedSummary);
    } catch (err) {
      setSummaryError(err as string);
    } finally {
      setIsGeneratingSummary(false);
    }
  };

  if (!selectedProject) {
    return (
      <div className="flex items-center justify-center h-full">
        <div className="text-center text-gray-500">
          <svg className="mx-auto h-16 w-16 text-gray-300 mb-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={1} d="M19 11H5m14 0a2 2 0 012 2v6a2 2 0 01-2 2H5a2 2 0 01-2-2v-6a2 2 0 012-2m14 0V9a2 2 0 00-2-2M5 11V9a2 2 0 012-2m0 0V5a2 2 0 012-2h6a2 2 0 012 2v2M7 7h10" />
          </svg>
          <h3 className="text-lg font-medium text-gray-900 mb-2">Select a Project</h3>
          <p className="text-gray-600">Choose a project from the left sidebar to analyze and generate ideas</p>
        </div>
      </div>
    );
  }

  return (
    <div className="h-full flex flex-col">
      {/* Project Header */}
      <div className="p-6 border-b border-gray-200 bg-white">
        <div className="flex items-center space-x-3">
          <div className="flex-shrink-0">
            <div className="w-10 h-10 bg-blue-100 rounded-lg flex items-center justify-center">
              <svg className="w-5 h-5 text-blue-600" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M3 7v10a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-5l-2-2H5a2 2 0 00-2 2z" />
              </svg>
            </div>
          </div>
          <div className="flex-1">
            <div className="flex items-center space-x-2">
              <h1 className="text-xl font-bold text-gray-900">{selectedProject.name}</h1>
              {selectedProject.is_git_repo && (
                <span className="inline-flex items-center px-2 py-0.5 rounded text-xs font-medium bg-green-100 text-green-800">
                  Git Repo
                </span>
              )}
            </div>
            {selectedProject.description && (
              <p className="text-gray-600 mt-1">{selectedProject.description}</p>
            )}
            <p className="text-sm text-gray-500 mt-1">
              {selectedProject.file_count} files • {selectedProject.path}
            </p>
          </div>
        </div>
      </div>

      {/* Content Area */}
      <div className="flex-1 overflow-y-auto">
        {isAnalyzing && (
          <div className="flex items-center justify-center py-12">
            <div className="inline-block animate-spin rounded-full h-8 w-8 border-b-2 border-blue-600"></div>
            <p className="ml-4 text-gray-600">Analyzing project...</p>
          </div>
        )}

        {analysisError && (
          <div className="p-6">
            <div className="bg-red-50 border border-red-200 rounded-md p-4">
              <h3 className="text-red-800 font-medium">Analysis Error</h3>
              <p className="text-red-600 mt-2">{analysisError}</p>
              <button
                onClick={analyzeProject}
                className="mt-4 bg-red-600 text-white px-4 py-2 rounded-md hover:bg-red-700"
              >
                Retry Analysis
              </button>
            </div>
          </div>
        )}

        {analysis && (
          <div className="p-6">
            {/* Analysis Summary */}
            <div className="bg-white rounded-lg border border-gray-200 p-6 mb-6">
              <h2 className="text-lg font-semibold text-gray-900 mb-4">Project Analysis</h2>
              
              <div className="grid grid-cols-1 md:grid-cols-3 gap-4 mb-6">
                <div className="bg-blue-50 rounded-lg p-4">
                  <h3 className="font-semibold text-blue-900 mb-1">Total Files</h3>
                  <p className="text-2xl font-bold text-blue-600">{analysis.metrics.total_files}</p>
                </div>
                
                <div className="bg-green-50 rounded-lg p-4">
                  <h3 className="font-semibold text-green-900 mb-1">Total Lines</h3>
                  <p className="text-2xl font-bold text-green-600">{analysis.metrics.total_lines?.toLocaleString()}</p>
                </div>
                
                <div className="bg-purple-50 rounded-lg p-4">
                  <h3 className="font-semibold text-purple-900 mb-1">Analyzed Files</h3>
                  <p className="text-2xl font-bold text-purple-600">{analysis.metrics.analyzed_files}</p>
                </div>
              </div>

              <div className="mb-6">
                <h3 className="text-md font-semibold text-gray-900 mb-3">Technologies Detected</h3>
                <div className="flex flex-wrap gap-2">
                  {analysis.technologies.map((tech) => (
                    <span
                      key={tech}
                      className="bg-gray-100 text-gray-800 px-3 py-1 rounded-full text-sm"
                    >
                      {tech}
                    </span>
                  ))}
                </div>
              </div>
            </div>

            {/* Project Summary Section */}
            <div className="bg-white rounded-lg border border-gray-200 p-6 mb-6">
              <div className="flex justify-between items-center mb-4">
                <h2 className="text-lg font-semibold text-gray-900">Project Summary</h2>
                {!summary && !isGeneratingSummary && (
                  <button
                    onClick={generateSummary}
                    className="bg-blue-600 text-white px-4 py-2 rounded-md hover:bg-blue-700 font-medium"
                  >
                    Generate Summary
                  </button>
                )}
                {summary && !isGeneratingSummary && (
                  <button
                    onClick={generateSummary}
                    className="bg-blue-600 text-white px-4 py-2 rounded-md hover:bg-blue-700"
                  >
                    Regenerate Summary
                  </button>
                )}
              </div>

              {isGeneratingSummary && (
                <div className="text-center py-8">
                  <div className="inline-block animate-spin rounded-full h-8 w-8 border-b-2 border-blue-600"></div>
                  <p className="mt-4 text-gray-600">Generating AI summary...</p>
                  <p className="text-sm text-gray-500">This may take a moment...</p>
                </div>
              )}

              {summaryError && (
                <div className="bg-red-50 border border-red-200 rounded-md p-4">
                  <h3 className="text-red-800 font-medium">Summary Error</h3>
                  <p className="text-red-600 mt-2">{summaryError}</p>
                  <button
                    onClick={generateSummary}
                    className="mt-4 bg-red-600 text-white px-4 py-2 rounded-md hover:bg-red-700"
                  >
                    Retry Generation
                  </button>
                </div>
              )}

              {!summary && !isGeneratingSummary && !summaryError && (
                <div className="text-center py-8 text-gray-500">
                  <svg className="mx-auto h-12 w-12 text-gray-300 mb-3" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                    <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={1} d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
                  </svg>
                  <p>No summary generated yet</p>
                  <p className="text-sm text-gray-400 mt-1">Click "Generate Summary" to get an AI-powered project summary</p>
                </div>
              )}

              {summary && (
                <div className="space-y-4">
                  <div className="prose prose-sm max-w-none prose-headings:text-gray-900 prose-p:text-gray-700 prose-strong:text-gray-800 prose-ul:text-gray-700 prose-li:text-gray-700">
                    <div dangerouslySetInnerHTML={{ __html: renderMarkdown(summary.summary) }} />
                  </div>
                  
                  {summary.key_features.length > 0 && (
                    <div className="mt-4">
                      <h3 className="text-sm font-semibold text-gray-900 mb-2">Key Features:</h3>
                      <ul className="space-y-1">
                        {summary.key_features.map((feature, index) => (
                          <li key={index} className="flex items-start">
                            <span className="text-blue-500 mr-2">•</span>
                            <span className="text-gray-700 text-sm">{feature}</span>
                          </li>
                        ))}
                      </ul>
                    </div>
                  )}
                  
                  <div className="mt-4 pt-4 border-t border-gray-200">
                    <p className="text-xs text-gray-500">
                      Generated: {new Date(summary.generated_at).toLocaleString()}
                    </p>
                  </div>
                </div>
              )}
            </div>

            {/* Ideas Section */}
            <div className="bg-white rounded-lg border border-gray-200 p-6">
              <div className="flex justify-between items-center mb-4">
                <h2 className="text-lg font-semibold text-gray-900">Development Ideas</h2>
                {ideas.length === 0 && !isGeneratingIdeas && (
                  <button
                    onClick={generateIdeas}
                    className="bg-green-600 text-white px-4 py-2 rounded-md hover:bg-green-700 font-medium"
                  >
                    Generate Ideas
                  </button>
                )}
                {ideas.length > 0 && !isGeneratingIdeas && (
                  <button
                    onClick={generateIdeas}
                    className="bg-green-600 text-white px-4 py-2 rounded-md hover:bg-green-700"
                  >
                    Regenerate Ideas
                  </button>
                )}
              </div>

              {isGeneratingIdeas && (
                <div className="text-center py-8">
                  <div className="inline-block animate-spin rounded-full h-8 w-8 border-b-2 border-green-600"></div>
                  <p className="mt-4 text-gray-600">Generating creative ideas...</p>
                  <p className="text-sm text-gray-500">This may take a moment...</p>
                </div>
              )}

              {ideasError && (
                <div className="bg-red-50 border border-red-200 rounded-md p-4 mb-6">
                  <h3 className="text-red-800 font-medium">Generation Error</h3>
                  <p className="text-red-600 mt-2">{ideasError}</p>
                  <button
                    onClick={generateIdeas}
                    className="mt-4 bg-red-600 text-white px-4 py-2 rounded-md hover:bg-red-700"
                  >
                    Retry Generation
                  </button>
                </div>
              )}

              {ideas.length === 0 && !isGeneratingIdeas && !ideasError && (
                <div className="text-center py-8 text-gray-500">
                  <svg className="mx-auto h-12 w-12 text-gray-300 mb-3" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                    <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={1} d="M9.663 17h4.673M12 3v1m6.364 1.636l-.707.707M21 12h-1M4 12H3m3.343-5.657l-.707-.707m2.828 9.9a5 5 0 117.072 0l-.548.547A3.374 3.374 0 0014 18.469V19a2 2 0 11-4 0v-.531c0-.895-.356-1.754-.988-2.386l-.548-.547z" />
                  </svg>
                  <p>No ideas generated yet</p>
                  <p className="text-sm text-gray-400 mt-1">Click "Generate Ideas" to get AI-powered development suggestions</p>
                </div>
              )}

              {ideas.length > 0 && (
                <div className="space-y-4">
                  {ideas.map((idea, index) => (
                    <div
                      key={index}
                      className="border-l-4 border-green-500 bg-green-50 p-4 rounded-r-md"
                    >
                      <div className="flex items-start">
                        <div className="flex-shrink-0">
                          <span className="bg-green-500 text-white rounded-full w-6 h-6 flex items-center justify-center text-sm font-medium">
                            {index + 1}
                          </span>
                        </div>
                        <div className="ml-3">
                          <p className="text-gray-800 whitespace-pre-line">{idea}</p>
                        </div>
                      </div>
                    </div>
                  ))}
                </div>
              )}
            </div>
          </div>
        )}
      </div>
    </div>
  );
};

export default ProjectAnalyzer;