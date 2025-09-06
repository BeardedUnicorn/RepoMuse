import React, { useState } from 'react';
import { Moon, Sun, Monitor } from 'lucide-react';
import { useTheme } from '../../contexts/ThemeContext';

interface ThemeToggleProps {
  className?: string;
  showLabel?: boolean;
}

const ThemeToggle: React.FC<ThemeToggleProps> = ({ className = '', showLabel = false }) => {
  const { theme, themePreference, toggleTheme, setThemePreference } = useTheme();
  const [showMenu, setShowMenu] = useState(false);

  const handleToggle = (e: React.MouseEvent) => {
    if (e.shiftKey) {
      // Shift+click to show theme preference menu
      setShowMenu(!showMenu);
    } else {
      toggleTheme();
      setShowMenu(false);
    }
  };

  const selectPreference = (preference: 'light' | 'dark' | 'system') => {
    setThemePreference(preference);
    setShowMenu(false);
  };

  const getIcon = () => {
    switch (themePreference) {
      case 'system':
        return <Monitor className="h-5 w-5" />;
      case 'dark':
        return <Moon className="h-5 w-5" />;
      case 'light':
        return <Sun className="h-5 w-5" />;
      default:
        return theme === 'light' ? <Sun className="h-5 w-5" /> : <Moon className="h-5 w-5" />;
    }
  };

  const getLabel = () => {
    switch (themePreference) {
      case 'system':
        return 'Auto';
      case 'dark':
        return 'Dark';
      case 'light':
        return 'Light';
      default:
        return theme === 'light' ? 'Light' : 'Dark';
    }
  };

  return (
    <div className="relative">
      <button
        onClick={handleToggle}
        className={`flex items-center gap-2 p-2 rounded-md hover:bg-background-tertiary text-foreground-secondary hover:text-foreground transition-colors ${className}`}
        aria-label={`Current theme: ${getLabel()}. Click to toggle, Shift+click for options`}
        title="Click to toggle theme, Shift+click for options"
      >
        {getIcon()}
        {showLabel && <span className="text-sm">{getLabel()}</span>}
      </button>
      
      {showMenu && (
        <>
          <div 
            className="fixed inset-0 z-40" 
            onClick={() => setShowMenu(false)}
          />
          <div className="absolute right-0 mt-2 w-48 rounded-md shadow-lg bg-background-secondary border border-border z-50">
            <div className="py-1">
              <button
                onClick={() => selectPreference('light')}
                className={`flex items-center w-full px-4 py-2 text-sm hover:bg-background-tertiary text-foreground ${
                  themePreference === 'light' ? 'bg-primary/10' : ''
                }`}
              >
                <Sun className="h-4 w-4 mr-3" />
                Light
                {themePreference === 'light' && (
                  <span className="ml-auto text-primary">✓</span>
                )}
              </button>
              <button
                onClick={() => selectPreference('dark')}
                className={`flex items-center w-full px-4 py-2 text-sm hover:bg-background-tertiary text-foreground ${
                  themePreference === 'dark' ? 'bg-primary/10' : ''
                }`}
              >
                <Moon className="h-4 w-4 mr-3" />
                Dark
                {themePreference === 'dark' && (
                  <span className="ml-auto text-primary">✓</span>
                )}
              </button>
              <button
                onClick={() => selectPreference('system')}
                className={`flex items-center w-full px-4 py-2 text-sm hover:bg-background-tertiary text-foreground ${
                  themePreference === 'system' ? 'bg-primary/10' : ''
                }`}
              >
                <Monitor className="h-4 w-4 mr-3" />
                System
                {themePreference === 'system' && (
                  <span className="ml-auto text-primary">✓</span>
                )}
              </button>
            </div>
          </div>
        </>
      )}
    </div>
  );
};

export default ThemeToggle;