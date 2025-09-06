import React, { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { ProjectDirectory } from '../types';
import { listProjectDirectories } from '../utils/api';

interface ProjectListProps {
  rootPath: string;
  selectedProject: string | null;
  onProjectSelect: (project: ProjectDirectory) => void;
}

const ProjectList: React.FC<ProjectListProps> = ({ rootPath, selectedProject, onProjectSelect }) => {
  const [projects, setProjects] = useState<ProjectDirectory[]>([]);
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<string>('');

  useEffect(() => {
    if (rootPath) {
      loadProjects();
    }
  }, [rootPath]);

  const loadProjects = async () => {
    setIsLoading(true);
    setError('');
    
    try {
      const projectList = await listProjectDirectories(rootPath);
      setProjects(projectList);
      
      // Asynchronously update file counts for projects that need it
      projectList.forEach(async (project) => {
        if (project.is_counting) {
          updateFileCount(project.path);
        }
      });
    } catch (err) {
      setError(err as string);
    } finally {
      setIsLoading(false);
    }
  };

  const updateFileCount = async (projectPath: string) => {
    try {
      const count = await invoke<number>('update_project_file_count', { projectPath });
      
      // Update the project in state with the new count
      setProjects(prevProjects => 
        prevProjects.map(p => 
          p.path === projectPath 
            ? { ...p, file_count: count, is_counting: false }
            : p
        )
      );
    } catch (err) {
      console.error(`Failed to update file count for ${projectPath}:`, err);
    }
  };

  if (isLoading) {
    return (
      <div className="flex items-center justify-center py-8">
        <div className="inline-block animate-spin rounded-full h-6 w-6 border-b-2 border-blue-600"></div>
        <span className="ml-2 text-gray-600">Loading projects...</span>
      </div>
    );
  }

  if (error) {
    return (
      <div className="p-4">
        <div className="bg-red-50 border border-red-200 rounded-md p-3">
          <p className="text-red-800 text-sm font-medium">Error loading projects</p>
          <p className="text-red-600 text-sm mt-1">{error}</p>
          <button
            onClick={loadProjects}
            className="mt-2 text-xs bg-red-600 text-white px-2 py-1 rounded hover:bg-red-700"
          >
            Retry
          </button>
        </div>
      </div>
    );
  }

  if (projects.length === 0) {
    return (
      <div className="p-4 text-center">
        <div className="text-gray-500">
          <svg className="mx-auto h-12 w-12 text-gray-300 mb-2" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={1} d="M3 7v10a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-5l-2-2H5a2 2 0 00-2 2z" />
          </svg>
          <p className="text-sm">No projects found</p>
          <p className="text-xs text-gray-400 mt-1">
            Projects are identified by files like package.json, Cargo.toml, etc.
          </p>
        </div>
      </div>
    );
  }

  return (
    <div className="h-full overflow-y-auto">
      <div className="p-3 border-b border-gray-200 bg-gray-50">
        <h3 className="font-semibold text-gray-900 text-sm">Projects ({projects.length})</h3>
        <p className="text-xs text-gray-500 mt-1">
          {rootPath.split(/[/\\]/).pop()}
        </p>
      </div>
      
      <div className="divide-y divide-gray-100">
        {projects.map((project) => (
          <div
            key={project.path}
            onClick={() => onProjectSelect(project)}
            className={`p-4 cursor-pointer hover:bg-blue-50 border-l-4 ${
              selectedProject === project.path
                ? 'border-blue-500 bg-blue-50'
                : 'border-transparent'
            }`}
          >
            <div className="flex items-start justify-between">
              <div className="flex-1 min-w-0">
                <div className="flex items-center space-x-2">
                  <h4 className="font-medium text-gray-900 truncate text-sm">
                    {project.name}
                  </h4>
                  {project.is_git_repo && (
                    <svg className="h-3 w-3 text-gray-500" fill="currentColor" viewBox="0 0 24 24">
                      <path d="M12 2C6.477 2 2 6.484 2 12.017c0 4.425 2.865 8.18 6.839 9.504.5.092.682-.217.682-.483 0-.237-.008-.868-.013-1.703-2.782.605-3.369-1.343-3.369-1.343-.454-1.158-1.11-1.466-1.11-1.466-.908-.62.069-.608.069-.608 1.003.07 1.531 1.032 1.531 1.032.892 1.53 2.341 1.088 2.91.832.092-.647.35-1.088.636-1.338-2.22-.253-4.555-1.113-4.555-4.951 0-1.093.39-1.988 1.029-2.688-.103-.253-.446-1.272.098-2.65 0 0 .84-.27 2.75 1.026A9.564 9.564 0 0112 6.844c.85.004 1.705.115 2.504.337 1.909-1.296 2.747-1.027 2.747-1.027.546 1.379.202 2.398.1 2.651.64.7 1.028 1.595 1.028 2.688 0 3.848-2.339 4.695-4.566 4.943.359.309.678.92.678 1.855 0 1.338-.012 2.419-.012 2.747 0 .268.18.58.688.482A10.019 10.019 0 0022 12.017C22 6.484 17.522 2 12 2z"/>
                    </svg>
                  )}
                </div>
                
                {project.description && (
                  <p className="text-xs text-gray-600 mt-1 line-clamp-2">
                    {project.description}
                  </p>
                )}
                
                <div className="flex items-center mt-2 text-xs text-gray-500 space-x-3">
                  <span className="flex items-center">
                    <svg className="h-3 w-3 mr-1" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                      <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
                    </svg>
                    {project.file_count} files
                    {project.is_counting && (
                      <span className="ml-1 text-blue-500 animate-pulse">
                        (counting...)
                      </span>
                    )}
                  </span>
                </div>
              </div>
            </div>
          </div>
        ))}
      </div>
    </div>
  );
};

export default ProjectList;