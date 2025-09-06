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
import Button from './components/ui/Button';
import HeaderNav from './components/ui/HeaderNav';
import ToastProvider from './components/ui/ToastProvider';
import { basename } from './utils/format';

type View = 'folder' | 'settings' | 'workspace';

const App: React.FC = () => {
  const [currentView, setCurrentView] = useState<View>('folder');
  const [settings, setSettings] = useState<SettingsType | null>(null);
  const [rootPath, setRootPath] = useState<string>('');
  const [selectedProject, setSelectedProject] = useState<ProjectDirectory | null>(null);
  // removed unused isLoadingRoot

  useEffect(() => {
    loadSettings().then(setSettings);
    loadSavedRootFolder();
  }, []);

  const loadSavedRootFolder = async () => {
    try {
      const savedRoot = await loadRootFolder();
      if (savedRoot) {
        setRootPath(savedRoot);
        setCurrentView('workspace');
      }
    } catch (error) {
      console.error('Error loading saved root folder:', error);
    } finally {
      // no-op
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
    <ToastProvider>
    <div className="h-screen bg-gray-50 flex flex-col">
      {/* Navigation */}
      <HeaderNav
        title="RepoMuse"
        subtitle={rootPath ? (<><span className="text-gray-400">•</span><span className="ml-2">{basename(rootPath)}</span></>) : undefined}
        actions={(
          <>
            {currentView === 'workspace' && (
              <Button variant="ghost" size="sm" onClick={selectNewFolder}>
                Change Root Folder
              </Button>
            )}
            <Button
              variant="ghost"
              size="sm"
              className={currentView === 'settings' ? 'bg-blue-100 text-blue-700' : ''}
              onClick={() => setCurrentView('settings')}
            >
              Settings
            </Button>
            {rootPath && currentView !== 'workspace' && (
              <Button variant="primary" size="sm" onClick={() => setCurrentView('workspace')}>
                Back to Projects
              </Button>
            )}
          </>
        )}
      />

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
    </ToastProvider>
  );
};

export default App;
