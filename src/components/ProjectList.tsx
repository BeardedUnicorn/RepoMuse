import React, { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { ProjectDirectory } from '../types';
import { listProjectDirectories } from '../utils/api';
import Spinner from './ui/Spinner';
import Alert from './ui/Alert';
import EmptyState from './ui/EmptyState';
import { Folder, GitBranch, FileText } from 'lucide-react';
import SidebarListItem from './ui/SidebarListItem';
import { basename } from '../utils/format';

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
      <div className="flex items-center justify-center py-8 text-gray-600">
        <Spinner size="sm" color="blue" />
        <span className="ml-2">Loading projects...</span>
      </div>
    );
  }

  if (error) {
    return (
      <div className="p-4">
        <Alert variant="error" title="Error loading projects">{error}</Alert>
        <div className="mt-2">
          <button onClick={loadProjects} className="text-xs bg-red-600 text-white px-2 py-1 rounded hover:bg-red-700">Retry</button>
        </div>
      </div>
    );
  }

  if (projects.length === 0) {
    return (
      <div className="p-4">
        <EmptyState
          icon={<Folder className="h-12 w-12 text-gray-300" />}
          title="No projects found"
          subtitle="Projects are identified by files like package.json, Cargo.toml, etc."
        />
      </div>
    );
  }

  return (
    <div className="h-full overflow-y-auto">
      <div className="p-3 border-b border-gray-200 bg-gray-50">
        <h3 className="font-semibold text-gray-900 text-sm">Projects ({projects.length})</h3>
        <p className="text-xs text-gray-500 mt-1">{basename(rootPath)}</p>
      </div>
      
      <div className="divide-y divide-gray-100">
        {projects.map((project) => (
          <SidebarListItem
            key={project.path}
            selected={selectedProject === project.path}
            onClick={() => onProjectSelect(project)}
            title={project.name}
            subtitle={project.description}
            left={project.is_git_repo ? <GitBranch className="h-3 w-3 text-gray-500" /> : null}
            meta={
              <span className="flex items-center">
                <FileText className="h-3 w-3 mr-1" />
                {project.file_count} files
                {project.is_counting && (
                  <span className="ml-1 text-blue-500 animate-pulse">(counting...)</span>
                )}
              </span>
            }
          />
        ))}
      </div>
    </div>
  );
};

export default ProjectList;
