import React, { useState } from 'react';
import { RepoAnalysis, Settings, IdeaRequest } from '../types';
import { generateIdeaList } from '../utils/api';

interface IdeaGeneratorProps {
  analysis: RepoAnalysis;
  settings: Settings;
  onIdeasGenerated: (ideas: string[]) => void;
  ideas: string[];
}

const IdeaGenerator: React.FC<IdeaGeneratorProps> = ({
  analysis,
  settings,
  onIdeasGenerated,
  ideas
}) => {
  const [isGenerating, setIsGenerating] = useState(false);
  const [error, setError] = useState<string>('');

  const generateIdeas = async () => {
    setIsGenerating(true);
    setError('');

    try {
      const request: IdeaRequest = {
        analysis,
        settings,
      };

      const generatedIdeas = await generateIdeaList(request);
      onIdeasGenerated(generatedIdeas);
    } catch (err) {
      setError(err as string);
    } finally {
      setIsGenerating(false);
    }
  };

  return (
    <div className="max-w-4xl mx-auto">
      <div className="bg-white rounded-lg shadow-md p-8">
        <div className="mb-6">
          <h2 className="text-2xl font-bold text-gray-900 mb-2">Development Ideas</h2>
          <p className="text-gray-600">AI-powered suggestions for your repository</p>
        </div>

        {ideas.length === 0 && !isGenerating && (
          <div className="text-center py-8">
            <button
              onClick={generateIdeas}
              className="bg-green-600 text-white px-6 py-3 rounded-md hover:bg-green-700 font-medium"
            >
              Generate Ideas
            </button>
          </div>
        )}

        {isGenerating && (
          <div className="text-center py-8">
            <div className="inline-block animate-spin rounded-full h-8 w-8 border-b-2 border-green-600"></div>
            <p className="mt-4 text-gray-600">Generating creative ideas...</p>
            <p className="text-sm text-gray-500">This may take a moment...</p>
          </div>
        )}

        {error && (
          <div className="bg-red-50 border border-red-200 rounded-md p-4 mb-6">
            <h3 className="text-red-800 font-medium">Generation Error</h3>
            <p className="text-red-600 mt-2">{error}</p>
            <button
              onClick={generateIdeas}
              className="mt-4 bg-red-600 text-white px-4 py-2 rounded-md hover:bg-red-700"
            >
              Retry Generation
            </button>
          </div>
        )}

        {ideas.length > 0 && (
          <div>
            <div className="flex justify-between items-center mb-6">
              <h3 className="text-lg font-semibold text-gray-900">
                Generated Ideas ({ideas.length})
              </h3>
              <button
                onClick={generateIdeas}
                disabled={isGenerating}
                className="bg-green-600 text-white px-4 py-2 rounded-md hover:bg-green-700 disabled:opacity-50"
              >
                Regenerate Ideas
              </button>
            </div>

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

            <div className="mt-8 p-4 bg-gray-50 rounded-lg">
              <h4 className="font-medium text-gray-900 mb-2">Repository Summary</h4>
              <div className="grid grid-cols-2 md:grid-cols-4 gap-4 text-sm">
                <div>
                  <span className="text-gray-500">Technologies:</span>
                  <p className="font-medium">{analysis.technologies.join(', ')}</p>
                </div>
                <div>
                  <span className="text-gray-500">Total Files:</span>
                  <p className="font-medium">{analysis.metrics.total_files}</p>
                </div>
                <div>
                  <span className="text-gray-500">Total Lines:</span>
                  <p className="font-medium">{analysis.metrics.total_lines?.toLocaleString()}</p>
                </div>
                <div>
                  <span className="text-gray-500">Analyzed Files:</span>
                  <p className="font-medium">{analysis.metrics.analyzed_files}</p>
                </div>
              </div>
            </div>
          </div>
        )}
      </div>
    </div>
  );
};

export default IdeaGenerator;