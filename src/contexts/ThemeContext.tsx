import React, { createContext, useContext, useEffect, useState, useCallback } from 'react';
import { invoke } from '@tauri-apps/api/core';

type Theme = 'light' | 'dark';
type ThemePreference = 'light' | 'dark' | 'system';

interface ThemeContextType {
  theme: Theme;
  themePreference: ThemePreference;
  toggleTheme: () => void;
  setTheme: (theme: Theme) => void;
  setThemePreference: (preference: ThemePreference) => void;
}

const ThemeContext = createContext<ThemeContextType | undefined>(undefined);

export const useTheme = () => {
  const context = useContext(ThemeContext);
  if (!context) {
    throw new Error('useTheme must be used within a ThemeProvider');
  }
  return context;
};

interface ThemeProviderProps {
  children: React.ReactNode;
}

// Get system theme preference
const getSystemTheme = (): Theme => {
  if (window.matchMedia && window.matchMedia('(prefers-color-scheme: dark)').matches) {
    return 'dark';
  }
  return 'light';
};

export const ThemeProvider: React.FC<ThemeProviderProps> = ({ children }) => {
  const [themePreference, setThemePreferenceState] = useState<ThemePreference>('system');
  const [theme, setThemeState] = useState<Theme>(() => getSystemTheme());
  const [isInitialized, setIsInitialized] = useState(false);

  // Load theme preference from Tauri storage on mount
  useEffect(() => {
    const loadThemePreference = async () => {
      try {
        const savedTheme = await invoke<string | null>('load_theme_preference');
        if (savedTheme) {
          const preference = savedTheme as ThemePreference;
          setThemePreferenceState(preference);
          
          if (preference === 'system') {
            setThemeState(getSystemTheme());
          } else {
            setThemeState(preference as Theme);
          }
        } else {
          // Fallback to localStorage for backward compatibility
          const localStorageTheme = localStorage.getItem('theme-preference') as ThemePreference | null;
          if (localStorageTheme) {
            setThemePreferenceState(localStorageTheme);
            if (localStorageTheme === 'system') {
              setThemeState(getSystemTheme());
            } else {
              setThemeState(localStorageTheme as Theme);
            }
          }
        }
      } catch (error) {
        console.error('Error loading theme preference:', error);
        // Fallback to localStorage
        const localStorageTheme = localStorage.getItem('theme-preference') as ThemePreference | null;
        if (localStorageTheme) {
          setThemePreferenceState(localStorageTheme);
          if (localStorageTheme === 'system') {
            setThemeState(getSystemTheme());
          } else {
            setThemeState(localStorageTheme as Theme);
          }
        }
      } finally {
        setIsInitialized(true);
      }
    };

    loadThemePreference();
  }, []);

  // Listen for system theme changes
  useEffect(() => {
    if (!window.matchMedia) return;

    const mediaQuery = window.matchMedia('(prefers-color-scheme: dark)');
    
    const handleChange = (e: MediaQueryListEvent) => {
      if (themePreference === 'system') {
        setThemeState(e.matches ? 'dark' : 'light');
      }
    };

    // Modern browsers
    if (mediaQuery.addEventListener) {
      mediaQuery.addEventListener('change', handleChange);
      return () => mediaQuery.removeEventListener('change', handleChange);
    } 
    // Older browsers
    else if (mediaQuery.addListener) {
      mediaQuery.addListener(handleChange);
      return () => mediaQuery.removeListener(handleChange);
    }
  }, [themePreference]);

  // Apply theme to document and save to storage
  useEffect(() => {
    if (!isInitialized) return;

    const root = document.documentElement;
    
    // Update document class
    if (theme === 'dark') {
      root.classList.add('dark');
    } else {
      root.classList.remove('dark');
    }

    // Save to both localStorage (for quick access) and Tauri storage (for persistence)
    localStorage.setItem('theme-preference', themePreference);
    localStorage.setItem('theme-current', theme);
    
    // Save to Tauri storage
    invoke('save_theme_preference', { theme: themePreference })
      .catch(error => console.error('Error saving theme preference:', error));
  }, [theme, themePreference, isInitialized]);

  const toggleTheme = useCallback(() => {
    // Cycle through: light → dark → system → light
    let newPreference: ThemePreference;
    
    switch (themePreference) {
      case 'light':
        newPreference = 'dark';
        break;
      case 'dark':
        newPreference = 'system';
        break;
      case 'system':
        newPreference = 'light';
        break;
      default:
        newPreference = 'dark';
    }
    
    setThemePreferenceState(newPreference);
    
    if (newPreference === 'system') {
      setThemeState(getSystemTheme());
    } else {
      setThemeState(newPreference as Theme);
    }
  }, [themePreference]);

  const setTheme = useCallback((newTheme: Theme) => {
    setThemePreferenceState(newTheme);
    setThemeState(newTheme);
  }, []);

  const setThemePreference = useCallback((preference: ThemePreference) => {
    setThemePreferenceState(preference);
    if (preference === 'system') {
      setThemeState(getSystemTheme());
    } else {
      setThemeState(preference as Theme);
    }
  }, []);

  return (
    <ThemeContext.Provider value={{ 
      theme, 
      themePreference, 
      toggleTheme, 
      setTheme, 
      setThemePreference 
    }}>
      {children}
    </ThemeContext.Provider>
  );
};