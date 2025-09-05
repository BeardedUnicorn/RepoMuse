import React, { useState, useEffect } from 'react';
import FolderSelector from './components/FolderSelector';
import Settings from './components/Settings';
import CodeAnalyzer from './components/CodeAnalyzer';
import IdeaGenerator from './components/IdeaGenerator';
import { Settings as SettingsType, RepoAnalysis } from './types';
import { loadSettings } from './utils/storage';
import './styles/index.css';

type View = 'folder' | 'settings' | 'analyzer' | 'ideas';

const App: React.FC = () => {
  const [currentView, setCurrentView] = useState<View>('folder');
  const [settings, setSettings] = useState<SettingsType | null>(null);
  const [selectedFolder, setSelectedFolder] = useState<string>('');
  const [analysis, setAnalysis] = useState<RepoAnalysis | null>(null);
  const [ideas, setIdeas] = useState<string[]>([]);

  useEffect(() => {
    loadSettings().then(setSettings);
  }, []);

  const handleFolderSelected = (path: string) => {
    setSelectedFolder(path);
    setCurrentView('analyzer');
  };

  const handleAnalysisComplete = (analysisResult: RepoAnalysis) => {
    setAnalysis(analysisResult);
    setCurrentView('ideas');
  };

  const handleIdeasGenerated = (generatedIdeas: string[]) => {
    setIdeas(generatedIdeas);
  };

  return (
    <div className="min-h-screen bg-gray-50">
      {/* Navigation */}
      <nav className="bg-white shadow-sm border-b">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
          <div className="flex justify-between h-16">
            <div className="flex items-center">
              <h1 className="text-xl font-bold text-gray-900">Code Repository Analyzer</h1>
            </div>
            <div className="flex items-center space-x-4">
              <button
                onClick={() => setCurrentView('folder')}
                className={`px-3 py-2 rounded-md text-sm font-medium ${
                  currentView === 'folder'
                    ? 'bg-blue-100 text-blue-700'
                    : 'text-gray-500 hover:text-gray-700'
                }`}
              >
                Select Folder
              </button>
              <button
                onClick={() => setCurrentView('settings')}
                className={`px-3 py-2 rounded-md text-sm font-medium ${
                  currentView === 'settings'
                    ? 'bg-blue-100 text-blue-700'
                    : 'text-gray-500 hover:text-gray-700'
                }`}
              >
                Settings
              </button>
              {analysis && (
                <button
                  onClick={() => setCurrentView('analyzer')}
                  className={`px-3 py-2 rounded-md text-sm font-medium ${
                    currentView === 'analyzer'
                      ? 'bg-blue-100 text-blue-700'
                      : 'text-gray-500 hover:text-gray-700'
                  }`}
                >
                  Analysis
                </button>
              )}
              {ideas.length > 0 && (
                <button
                  onClick={() => setCurrentView('ideas')}
                  className={`px-3 py-2 rounded-md text-sm font-medium ${
                    currentView === 'ideas'
                      ? 'bg-blue-100 text-blue-700'
                      : 'text-gray-500 hover:text-gray-700'
                  }`}
                >
                  Ideas
                </button>
              )}
            </div>
          </div>
        </div>
      </nav>

      {/* Main Content */}
      <main className="max-w-7xl mx-auto py-6 sm:px-6 lg:px-8">
        {currentView === 'folder' && (
          <FolderSelector onFolderSelected={handleFolderSelected} />
        )}
        
        {currentView === 'settings' && settings && (
          <Settings settings={settings} onSettingsUpdated={setSettings} />
        )}
        
        {currentView === 'analyzer' && selectedFolder && (
          <CodeAnalyzer
            folderPath={selectedFolder}
            onAnalysisComplete={handleAnalysisComplete}
          />
        )}
        
        {currentView === 'ideas' && analysis && settings && (
          <IdeaGenerator
            analysis={analysis}
            settings={settings}
            onIdeasGenerated={handleIdeasGenerated}
            ideas={ideas}
          />
        )}
      </main>
    </div>
  );
};

export default App;