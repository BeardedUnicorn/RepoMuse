import React, { useState } from 'react';
import { open } from '@tauri-apps/plugin-dialog';

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
        <div className="bg-white rounded-lg shadow-md p-8">
          <div className="mb-6">
            <svg
              className="mx-auto h-16 w-16 text-gray-400"
              fill="none"
              viewBox="0 0 24 24"
              stroke="currentColor"
            >
              <path
                strokeLinecap="round"
                strokeLinejoin="round"
                strokeWidth={1}
                d="M3 7v10a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-5l-2-2H5a2 2 0 00-2 2z"
              />
            </svg>
          </div>
          
          <h2 className="text-2xl font-bold text-gray-900 mb-4">
            Select Code Repository
          </h2>
          
          <p className="text-gray-600 mb-6">
            Choose a folder containing your code repository to analyze and generate development ideas.
          </p>
          
          <button
            onClick={selectFolder}
            disabled={isSelecting}
            className="w-full bg-blue-600 text-white px-4 py-2 rounded-md hover:bg-blue-700 disabled:opacity-50 disabled:cursor-not-allowed"
          >
            {isSelecting ? 'Selecting...' : 'Choose Folder'}
          </button>
        </div>
      </div>
    </div>
  );
};

export default FolderSelector;