import React, { useState, useEffect } from 'react';
import { open } from '@tauri-apps/plugin-dialog';
import FolderSelector from './components/FolderSelector';
import Settings from './components/Settings';
import ProjectList from './components/ProjectList';
import ProjectAnalyzer from './components/ProjectAnalyzer';
import { Settings as SettingsType, ProjectDirectory } from './types';
import { loadSettings } from './utils/storage';
import { loadRootFolder, saveRootFolder } from './utils/api';
import './index.css';

type View = 'folder' | 'settings' | 'workspace';

const App: React.FC = () => {
  const [currentView, setCurrentView] = useState<View>('folder');
  const [settings, setSettings] = useState<SettingsType | null>(null);
  const [rootPath, setRootPath] = useState<string>('');
  const [selectedProject, setSelectedProject] = useState<ProjectDirectory | null>(null);
  const [isLoadingRoot, setIsLoadingRoot] = useState(true);

  useEffect(() => {
    loadSettings().then(setSettings);
    loadSavedRootFolder();
  }, []);

  const loadSavedRootFolder = async () => {
    setIsLoadingRoot(true);
    try {
      const savedRoot = await loadRootFolder();
      if (savedRoot) {
        setRootPath(savedRoot);
        setCurrentView('workspace');
      }
    } catch (error) {
      console.error('Error loading saved root folder:', error);
    } finally {
      setIsLoadingRoot(false);
    }
  };

  const handleFolderSelected = async (path: string) => {
    setRootPath(path);
    setSelectedProject(null);
    setCurrentView('workspace');
    // Save the root folder for next launch
    try {
      await saveRootFolder(path);
    } catch (error) {
      console.error('Error saving root folder:', error);
    }
  };

  const handleProjectSelect = (project: ProjectDirectory) => {
    setSelectedProject(project);
  };

  const selectNewFolder = async () => {
    try {
      const selected = await open({
        directory: true,
        multiple: false,
      });
      
      if (selected && typeof selected === 'string') {
        handleFolderSelected(selected);
      }
    } catch (error) {
      console.error('Error selecting folder:', error);
    }
  };

  return (
    <div className="h-screen bg-gray-50 flex flex-col">
      {/* Navigation */}
      <nav className="bg-white shadow-sm border-b flex-shrink-0">
        <div className="max-w-full mx-auto px-4 sm:px-6 lg:px-8">
          <div className="flex justify-between h-14">
            <div className="flex items-center">
              <h1 className="text-lg font-bold text-gray-900">RepoMuse</h1>
              {rootPath && (
                <div className="ml-4 text-sm text-gray-600">
                  <span className="text-gray-400">•</span>
                  <span className="ml-2">{rootPath.split(/[/\\]/).pop()}</span>
                </div>
              )}
            </div>
            <div className="flex items-center space-x-2">
              {currentView === 'workspace' && (
                <button
                  onClick={selectNewFolder}
                  className="px-3 py-1.5 text-sm font-medium text-gray-500 hover:text-gray-700 rounded-md"
                >
                  Change Root Folder
                </button>
              )}
              <button
                onClick={() => setCurrentView('settings')}
                className={`px-3 py-1.5 rounded-md text-sm font-medium ${
                  currentView === 'settings'
                    ? 'bg-blue-100 text-blue-700'
                    : 'text-gray-500 hover:text-gray-700'
                }`}
              >
                Settings
              </button>
              {rootPath && currentView !== 'workspace' && (
                <button
                  onClick={() => setCurrentView('workspace')}
                  className="bg-blue-600 text-white px-3 py-1.5 rounded-md text-sm font-medium hover:bg-blue-700"
                >
                  Back to Projects
                </button>
              )}
            </div>
          </div>
        </div>
      </nav>

      {/* Main Content */}
      <div className="flex-1 flex overflow-hidden">
        {currentView === 'folder' && (
          <div className="flex-1 flex items-center justify-center">
            <FolderSelector onFolderSelected={handleFolderSelected} />
          </div>
        )}
        
        {currentView === 'settings' && settings && (
          <div className="flex-1 overflow-y-auto">
            <div className="max-w-4xl mx-auto py-8 px-4">
              <Settings settings={settings} onSettingsUpdated={setSettings} />
            </div>
          </div>
        )}
        
        {currentView === 'workspace' && rootPath && (
          <>
            {/* Left Sidebar - Project List */}
            <div className="w-80 bg-white border-r border-gray-200 flex flex-col">
              <ProjectList
                rootPath={rootPath}
                selectedProject={selectedProject?.path || null}
                onProjectSelect={handleProjectSelect}
              />
            </div>
            
            {/* Right Pane - Project Analyzer */}
            <div className="flex-1 bg-gray-50">
              {settings ? (
                <ProjectAnalyzer
                  selectedProject={selectedProject}
                  settings={settings}
                />
              ) : (
                <div className="flex items-center justify-center h-full">
                  <div className="text-center text-gray-500">
                    <p>Loading settings...</p>
                  </div>
                </div>
              )}
            </div>
          </>
        )}
      </div>
    </div>
  );
};

export default App;