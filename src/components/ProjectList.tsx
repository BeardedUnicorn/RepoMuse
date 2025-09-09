import React, { useState, useEffect, useMemo } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { ProjectDirectory } from '../types';
import { listProjectDirectories, loadFavoriteProjects, saveFavoriteProjects } from '../utils/api';
import Spinner from './ui/Spinner';
import Alert from './ui/Alert';
import EmptyState from './ui/EmptyState';
import { Folder, GitBranch, FileText, Search, X, Star } from 'lucide-react';
import SidebarListItem from './ui/SidebarListItem';
import { basename } from '../utils/format';

interface ProjectListProps {
  rootPath: string;
  selectedProject: string | null;
  onProjectSelect: (project: ProjectDirectory) => void;
}

const ProjectList: React.FC<ProjectListProps> = ({ rootPath, selectedProject, onProjectSelect }) => {
  const [projects, setProjects] = useState<ProjectDirectory[]>([]);
  const [favorites, setFavorites] = useState<Set<string>>(new Set());
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<string>('');
  const [searchQuery, setSearchQuery] = useState('');

  // Load both projects and favorites on mount or when rootPath changes
  useEffect(() => {
    const initializeData = async () => {
      // Load favorites first (globally)
      try {
        const favs = await loadFavoriteProjects();
        console.log('[ProjectList] Loaded favorites:', favs);
        setFavorites(new Set(favs));
      } catch (err) {
        console.error('[ProjectList] Error loading favorites:', err);
        setFavorites(new Set());
      }

      // Then load projects if we have a root path
      if (rootPath) {
        await loadProjects();
      }
    };

    initializeData();
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

  const toggleFavorite = async (projectPath: string, event: React.MouseEvent) => {
    event.stopPropagation(); // Prevent project selection
    
    const newFavorites = new Set(favorites);
    const wasFavorite = newFavorites.has(projectPath);
    
    if (wasFavorite) {
      newFavorites.delete(projectPath);
      console.log('[ProjectList] Removing favorite:', projectPath);
    } else {
      newFavorites.add(projectPath);
      console.log('[ProjectList] Adding favorite:', projectPath);
    }
    
    // Update UI immediately for responsiveness
    setFavorites(newFavorites);
    
    const favArray = Array.from(newFavorites);
    console.log('[ProjectList] Saving favorites array:', favArray);
    
    try {
      await saveFavoriteProjects(favArray);
      console.log('[ProjectList] Favorites saved successfully');
    } catch (err) {
      console.error('[ProjectList] Error saving favorites:', err);
      // Revert on error
      const revertedFavorites = new Set(favorites);
      if (wasFavorite) {
        revertedFavorites.add(projectPath);
      } else {
        revertedFavorites.delete(projectPath);
      }
      setFavorites(revertedFavorites);
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

  // Sort and filter projects
  const sortedAndFilteredProjects = useMemo(() => {
    let filtered = projects;
    
    // Apply search filter if there's a query
    if (searchQuery.trim()) {
      const query = searchQuery.toLowerCase();
      filtered = projects.filter(project => {
        const nameMatch = project.name.toLowerCase().includes(query);
        const descriptionMatch = project.description?.toLowerCase().includes(query);
        const pathMatch = project.path.toLowerCase().includes(query);
        return nameMatch || descriptionMatch || pathMatch;
      });
    }
    
    // Sort: favorites first (alphabetically), then non-favorites (alphabetically)
    const favoriteProjects = filtered.filter(p => favorites.has(p.path))
      .sort((a, b) => a.name.toLowerCase().localeCompare(b.name.toLowerCase()));
    
    const nonFavoriteProjects = filtered.filter(p => !favorites.has(p.path))
      .sort((a, b) => a.name.toLowerCase().localeCompare(b.name.toLowerCase()));
    
    return [...favoriteProjects, ...nonFavoriteProjects];
  }, [projects, favorites, searchQuery]);

  const clearSearch = () => {
    setSearchQuery('');
  };

  if (isLoading) {
    return (
      <div className="flex items-center justify-center py-8 text-foreground-secondary">
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
          <button onClick={loadProjects} className="text-xs bg-error text-error-foreground px-2 py-1 rounded hover:bg-error/90">Retry</button>
        </div>
      </div>
    );
  }

  if (projects.length === 0 && rootPath) {
    return (
      <div className="p-4">
        <EmptyState
          icon={<Folder className="h-12 w-12 text-foreground-tertiary" />}
          title="No projects found"
          subtitle="Projects are identified by files like package.json, Cargo.toml, etc."
        />
      </div>
    );
  }

  const favoriteCount = sortedAndFilteredProjects.filter(p => favorites.has(p.path)).length;

  return (
    <div className="h-full flex flex-col">
      {/* Header with project count */}
      <div className="p-3 border-b border-border bg-background-tertiary flex-shrink-0">
        <h3 className="font-semibold text-foreground text-sm">
          Projects ({sortedAndFilteredProjects.length}{sortedAndFilteredProjects.length !== projects.length ? ` of ${projects.length}` : ''})
        </h3>
        <p className="text-xs text-foreground-secondary mt-1">
          {basename(rootPath)}
          {favoriteCount > 0 && ` • ${favoriteCount} favorite${favoriteCount !== 1 ? 's' : ''}`}
        </p>
      </div>

      {/* Search Input */}
      <div className="p-3 border-b border-border bg-background flex-shrink-0">
        <div className="relative">
          <Search className="absolute left-3 top-1/2 transform -translate-y-1/2 h-4 w-4 text-foreground-tertiary" />
          <input
            type="text"
            value={searchQuery}
            onChange={(e) => setSearchQuery(e.target.value)}
            placeholder="Search projects..."
            className="w-full pl-9 pr-8 py-2 text-sm border border-border rounded-md focus:outline-none focus:ring-2 focus:ring-primary focus:border-transparent bg-background-secondary text-foreground placeholder-foreground-tertiary"
          />
          {searchQuery && (
            <button
              onClick={clearSearch}
              className="absolute right-2 top-1/2 transform -translate-y-1/2 p-1 hover:bg-background-tertiary rounded text-foreground-tertiary hover:text-foreground transition-colors"
              title="Clear search"
            >
              <X className="h-3 w-3" />
            </button>
          )}
        </div>
      </div>

      {/* Project List */}
      <div className="flex-1 overflow-y-auto">
        {sortedAndFilteredProjects.length === 0 && searchQuery ? (
          <div className="p-4">
            <EmptyState
              icon={<Search className="h-12 w-12 text-foreground-tertiary" />}
              title="No projects found"
              subtitle={`No projects match "${searchQuery}"`}
              action={
                <button
                  onClick={clearSearch}
                  className="text-sm text-primary hover:text-primary-hover underline"
                >
                  Clear search
                </button>
              }
            />
          </div>
        ) : sortedAndFilteredProjects.length > 0 ? (
          <div className="divide-y divide-border">
            {sortedAndFilteredProjects.map((project, index) => {
              const isFavorite = favorites.has(project.path);
              const isFirstNonFavorite = index > 0 && 
                favorites.has(sortedAndFilteredProjects[index - 1].path) && 
                !isFavorite;
              
              return (
                <React.Fragment key={project.path}>
                  {isFirstNonFavorite && favoriteCount > 0 && (
                    <div className="px-4 py-2 bg-background-tertiary">
                      <span className="text-xs font-medium text-foreground-tertiary uppercase tracking-wide">
                        Other Projects
                      </span>
                    </div>
                  )}
                  <div className="relative group">
                    <SidebarListItem
                      selected={selectedProject === project.path}
                      onClick={() => onProjectSelect(project)}
                      title={project.name}
                      subtitle={project.description}
                      left={
                        <div className="flex items-center gap-1">
                          {isFavorite && <Star className="h-3 w-3 text-yellow-500 fill-yellow-500" />}
                          {project.is_git_repo && <GitBranch className="h-3 w-3 text-foreground-tertiary" />}
                        </div>
                      }
                      meta={
                        <span className="flex items-center">
                          <FileText className="h-3 w-3 mr-1" />
                          {project.file_count} files
                          {project.is_counting && (
                            <span className="ml-1 text-primary animate-pulse">(counting...)</span>
                          )}
                        </span>
                      }
                    />
                    <button
                      onClick={(e) => toggleFavorite(project.path, e)}
                      className={`absolute right-4 top-1/2 transform -translate-y-1/2 p-1.5 rounded-md transition-all ${
                        isFavorite 
                          ? 'text-yellow-500 hover:text-yellow-600 dark:hover:text-yellow-400' 
                          : 'text-foreground-tertiary opacity-0 group-hover:opacity-100 hover:text-yellow-500 hover:bg-background-tertiary'
                      }`}
                      title={isFavorite ? 'Remove from favorites' : 'Add to favorites'}
                    >
                      <Star className={`h-4 w-4 ${isFavorite ? 'fill-current' : ''}`} />
                    </button>
                  </div>
                </React.Fragment>
              );
            })}
          </div>
        ) : null}
      </div>
    </div>
  );
};

export default ProjectList;