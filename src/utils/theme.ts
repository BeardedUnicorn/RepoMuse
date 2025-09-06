export type Theme = 'light' | 'dark';
export type ThemePreference = 'light' | 'dark' | 'system';

export const getSystemTheme = (): Theme => {
  if (typeof window === 'undefined') return 'light';
  if (window.matchMedia && window.matchMedia('(prefers-color-scheme: dark)').matches) {
    return 'dark';
  }
  return 'light';
};

export const applyTheme = (theme: Theme): void => {
  const root = document.documentElement;
  if (theme === 'dark') {
    root.classList.add('dark');
  } else {
    root.classList.remove('dark');
  }
};

export const getThemeFromPreference = (preference: ThemePreference): Theme => {
  if (preference === 'system') {
    return getSystemTheme();
  }
  return preference as Theme;
};