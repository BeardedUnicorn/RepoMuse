import React, { useState } from 'react';
import { open } from '@tauri-apps/plugin-dialog';
import { Folder } from 'lucide-react';

interface FolderSelectorProps {
  onFolderSelected: (path: string) => void;
}

const FolderSelector: React.FC<FolderSelectorProps> = ({ onFolderSelected }) => {
  const [isSelecting, setIsSelecting] = useState(false);

  const selectFolder = async () => {
    setIsSelecting(true);
    try {
      const selected = await open({
        directory: true,
        multiple: false,
      });
      
      if (selected && typeof selected === 'string') {
        onFolderSelected(selected);
      }
    } catch (error) {
      console.error('Error selecting folder:', error);
    } finally {
      setIsSelecting(false);
    }
  };

  return (
    <div className="text-center py-12">
      <div className="max-w-md mx-auto">
        <div className="bg-background-secondary rounded-lg shadow-md p-8 border border-border">
          <div className="mb-6">
            <Folder className="mx-auto h-16 w-16 text-foreground-tertiary" />
          </div>
          
          <h2 className="text-2xl font-bold text-foreground mb-4">
            Select Code Repository
          </h2>
          
          <p className="text-foreground-secondary mb-6">
            Choose a folder containing your code repository to analyze and generate development ideas.
          </p>
          
          <button
            onClick={selectFolder}
            disabled={isSelecting}
            className="w-full bg-primary text-primary-foreground px-4 py-2 rounded-md hover:bg-primary-hover disabled:opacity-50 disabled:cursor-not-allowed transition-colors"
          >
            {isSelecting ? 'Selecting...' : 'Choose Folder'}
          </button>
        </div>
      </div>
    </div>
  );
};

export default FolderSelector;