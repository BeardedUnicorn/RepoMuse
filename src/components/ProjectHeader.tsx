import React from 'react';
import Badge from './ui/Badge';
import { Folder } from 'lucide-react';

type Props = {
  name: string;
  path: string;
  description?: string | null;
  isGitRepo?: boolean;
  fileCount?: number;
};

const ProjectHeader: React.FC<Props> = ({ name, path, description, isGitRepo, fileCount }) => {
  return (
    <div className="p-6 border-b border-border bg-background-secondary">
      <div className="flex items-center space-x-3">
        <div className="flex-shrink-0">
          <div className="w-10 h-10 bg-primary/10 rounded-lg flex items-center justify-center">
            <Folder className="w-5 h-5 text-primary" />
          </div>
        </div>
        <div className="flex-1">
          <div className="flex items-center space-x-2">
            <h1 className="text-xl font-bold text-foreground">{name}</h1>
            {isGitRepo && <Badge variant="green">Git Repo</Badge>}
          </div>
          {description && <p className="text-foreground-secondary mt-1">{description}</p>}
          <p className="text-sm text-foreground-tertiary mt-1">
            {typeof fileCount === 'number' ? `${fileCount} files • ` : ''}{path}
          </p>
        </div>
      </div>
    </div>
  );
};

export default ProjectHeader;