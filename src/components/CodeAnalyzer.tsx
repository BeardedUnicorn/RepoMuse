import React, { useState, useEffect } from 'react';
import { RepoAnalysis } from '../types';
import { analyzeRepository } from '../utils/api';

interface CodeAnalyzerProps {
  folderPath: string;
  onAnalysisComplete: (analysis: RepoAnalysis) => void;
}

const CodeAnalyzer: React.FC<CodeAnalyzerProps> = ({ folderPath, onAnalysisComplete }) => {
  const [analysis, setAnalysis] = useState<RepoAnalysis | null>(null);
  const [isAnalyzing, setIsAnalyzing] = useState(false);
  const [error, setError] = useState<string>('');

  useEffect(() => {
    if (folderPath) {
      analyzeRepo();
    }
  }, [folderPath]);

  const analyzeRepo = async () => {
    setIsAnalyzing(true);
    setError('');
    
    try {
      const result = await analyzeRepository(folderPath);
      setAnalysis(result);
      onAnalysisComplete(result);
    } catch (err) {
      setError(err as string);
    } finally {
      setIsAnalyzing(false);
    }
  };

  if (isAnalyzing) {
    return (
      <div className="text-center py-12">
        <div className="inline-block animate-spin rounded-full h-8 w-8 border-b-2 border-blue-600"></div>
        <p className="mt-4 text-gray-600">Analyzing repository...</p>
      </div>
    );
  }

  if (error) {
    return (
      <div className="max-w-2xl mx-auto">
        <div className="bg-red-50 border border-red-200 rounded-md p-4">
          <h3 className="text-red-800 font-medium">Analysis Error</h3>
          <p className="text-red-600 mt-2">{error}</p>
          <button
            onClick={analyzeRepo}
            className="mt-4 bg-red-600 text-white px-4 py-2 rounded-md hover:bg-red-700"
          >
            Retry Analysis
          </button>
        </div>
      </div>
    );
  }

  if (!analysis) {
    return null;
  }

  return (
    <div className="max-w-4xl mx-auto">
      <div className="bg-white rounded-lg shadow-md p-8">
        <div className="mb-6">
          <h2 className="text-2xl font-bold text-gray-900 mb-2">Repository Analysis</h2>
          <p className="text-gray-600">Analysis of: {folderPath}</p>
        </div>

        <div className="grid grid-cols-1 md:grid-cols-3 gap-6 mb-8">
          <div className="bg-blue-50 rounded-lg p-4">
            <h3 className="font-semibold text-blue-900 mb-2">Total Files</h3>
            <p className="text-3xl font-bold text-blue-600">{analysis.metrics.total_files}</p>
          </div>
          
          <div className="bg-green-50 rounded-lg p-4">
            <h3 className="font-semibold text-green-900 mb-2">Total Lines</h3>
            <p className="text-3xl font-bold text-green-600">{analysis.metrics.total_lines?.toLocaleString()}</p>
          </div>
          
          <div className="bg-purple-50 rounded-lg p-4">
            <h3 className="font-semibold text-purple-900 mb-2">Analyzed Files</h3>
            <p className="text-3xl font-bold text-purple-600">{analysis.metrics.analyzed_files}</p>
          </div>
        </div>

        <div className="grid grid-cols-1 lg:grid-cols-2 gap-8">
          <div>
            <h3 className="text-lg font-semibold text-gray-900 mb-4">Technologies Detected</h3>
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

          <div>
            <h3 className="text-lg font-semibold text-gray-900 mb-4">Project Structure</h3>
            <div className="bg-gray-50 rounded-lg p-4 max-h-64 overflow-y-auto">
              {Object.entries(analysis.structure).slice(0, 10).map(([dir, files]) => (
                <div key={dir} className="mb-2">
                  <div className="font-medium text-gray-700 text-sm">{dir.split('/').pop() || dir}</div>
                  <div className="text-xs text-gray-500 ml-4">
                    {files.length} files
                  </div>
                </div>
              ))}
              {Object.keys(analysis.structure).length > 10 && (
                <div className="text-sm text-gray-500 italic">
                  ...and {Object.keys(analysis.structure).length - 10} more directories
                </div>
              )}
            </div>
          </div>
        </div>

        <div className="mt-8">
          <h3 className="text-lg font-semibold text-gray-900 mb-4">Sample Files</h3>
          <div className="space-y-3 max-h-96 overflow-y-auto">
            {analysis.files.slice(0, 5).map((file) => (
              <div key={file.path} className="border rounded-lg p-4">
                <div className="flex justify-between items-start mb-2">
                  <h4 className="font-medium text-gray-900 text-sm">{file.path.split('/').pop()}</h4>
                  <span className="bg-gray-100 text-gray-600 px-2 py-1 rounded text-xs">
                    {file.language}
                  </span>
                </div>
                <p className="text-xs text-gray-600 mb-2">{file.path}</p>
                <pre className="text-xs text-gray-700 bg-gray-50 p-2 rounded overflow-x-auto">
                  {file.content.substring(0, 300)}{file.content.length > 300 && '...'}
                </pre>
              </div>
            ))}
          </div>
        </div>
      </div>
    </div>
  );
};

export default CodeAnalyzer;