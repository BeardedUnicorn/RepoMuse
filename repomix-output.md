This file is a merged representation of the entire codebase, combined into a single document by Repomix.

# File Summary

## Purpose
This file contains a packed representation of the entire repository's contents.
It is designed to be easily consumable by AI systems for analysis, code review,
or other automated processes.

## File Format
The content is organized as follows:
1. This summary section
2. Repository information
3. Directory structure
4. Repository files (if enabled)
5. Multiple file entries, each consisting of:
  a. A header with the file path (## File: path/to/file)
  b. The full contents of the file in a code block

## Usage Guidelines
- This file should be treated as read-only. Any changes should be made to the
  original repository files, not this packed version.
- When processing this file, use the file path to distinguish
  between different files in the repository.
- Be aware that this file may contain sensitive information. Handle it with
  the same level of security as you would the original repository.

## Notes
- Some files may have been excluded based on .gitignore rules and Repomix's configuration
- Binary files are not included in this packed representation. Please refer to the Repository Structure section for a complete list of file paths, including binary files
- Files matching patterns in .gitignore are excluded
- Files matching default ignore patterns are excluded
- Files are sorted by Git change count (files with more changes are at the bottom)

# Directory Structure
```
src/
  components/
    ui/
      Alert.tsx
      Badge.tsx
      Button.tsx
      Card.tsx
      EmptyState.tsx
      Fieldset.tsx
      FormHelp.tsx
      FormLabel.tsx
      FormRow.tsx
      HeaderNav.tsx
      SectionHeader.tsx
      Select.tsx
      SidebarListItem.tsx
      Spinner.tsx
      StatTile.tsx
      Tabs.tsx
      TextField.tsx
      ToastProvider.tsx
    FolderSelector.tsx
    MarkdownRenderer.tsx
    ProjectAnalyzer.tsx
    ProjectHeader.tsx
    ProjectInsights.tsx
    ProjectList.tsx
    Settings.tsx
  types/
    index.ts
  utils/
    api.ts
    format.ts
    models.ts
    storage.ts
  App.tsx
  index.css
  main.tsx
  vite-env.d.ts
src-tauri/
  capabilities/
    default.json
  src/
    ai.rs
    analysis.rs
    cache.rs
    fs_utils.rs
    insights.rs
    lib.rs
    main.rs
    projects.rs
    storage.rs
  .gitignore
  build.rs
  Cargo.toml
  tauri.conf.json
.gitignore
AGENTS.md
index.html
package.json
postcss.config.js
README.md
tailwind.config.js
tsconfig.json
tsconfig.node.json
vite.config.ts
```

# Files

## File: src/components/ui/Alert.tsx
```typescript
import React from 'react';

type Variant = 'error' | 'info' | 'success';

type AlertProps = {
  variant?: Variant;
  title?: string;
  children?: React.ReactNode;
  className?: string;
};

const styles: Record<Variant, { container: string; title: string; text: string }> = {
  error: {
    container: 'bg-red-50 border border-red-200',
    title: 'text-red-800',
    text: 'text-red-600',
  },
  info: {
    container: 'bg-blue-50 border border-blue-200',
    title: 'text-blue-800',
    text: 'text-blue-700',
  },
  success: {
    container: 'bg-green-50 border border-green-200',
    title: 'text-green-800',
    text: 'text-green-700',
  },
};

const Alert: React.FC<AlertProps> = ({ variant = 'info', title, children, className = '' }) => {
  const s = styles[variant];
  return (
    <div className={`${s.container} rounded-md p-4 ${className}`}>
      {title && <h3 className={`${s.title} font-medium`}>{title}</h3>}
      {children && <div className={`${s.text} mt-2`}>{children}</div>}
    </div>
  );
};

export default Alert;
```

## File: src/components/ui/Badge.tsx
```typescript
import React from 'react';

type Variant = 'green' | 'purple' | 'gray' | 'blue' | 'red';

type BadgeProps = {
  variant?: Variant;
  children: React.ReactNode;
  className?: string;
};

const variants: Record<Variant, string> = {
  green: 'bg-green-100 text-green-800',
  purple: 'bg-purple-100 text-purple-800',
  gray: 'bg-gray-100 text-gray-800',
  blue: 'bg-blue-100 text-blue-800',
  red: 'bg-red-100 text-red-800',
};

const Badge: React.FC<BadgeProps> = ({ variant = 'gray', children, className = '' }) => (
  <span className={`inline-flex items-center px-2 py-0.5 rounded text-xs font-medium ${variants[variant]} ${className}`}>
    {children}
  </span>
);

export default Badge;
```

## File: src/components/ui/Button.tsx
```typescript
import React from 'react';

type Variant = 'primary' | 'secondary' | 'ghost';
type Size = 'sm' | 'md';

type ButtonProps = React.ButtonHTMLAttributes<HTMLButtonElement> & {
  variant?: Variant;
  size?: Size;
  loading?: boolean;
};

const variantClasses: Record<Variant, string> = {
  primary: 'bg-blue-600 text-white hover:bg-blue-700 disabled:opacity-50 disabled:cursor-not-allowed',
  secondary: 'bg-gray-100 hover:bg-gray-200 text-gray-900 disabled:opacity-50 disabled:cursor-not-allowed',
  ghost: 'text-gray-500 hover:text-gray-700 disabled:opacity-50 disabled:cursor-not-allowed',
};

const sizeClasses: Record<Size, string> = {
  sm: 'px-3 py-1 text-sm',
  md: 'px-4 py-2 text-sm',
};

const Button: React.FC<ButtonProps> = ({
  variant = 'primary',
  size = 'md',
  loading = false,
  className = '',
  children,
  disabled,
  ...rest
}) => {
  return (
    <button
      className={`${variantClasses[variant]} ${sizeClasses[size]} rounded-md font-medium ${className}`}
      disabled={disabled || loading}
      {...rest}
    >
      {loading ? 'Loading...' : children}
    </button>
  );
};

export default Button;
```

## File: src/components/ui/Card.tsx
```typescript
import React from 'react';

type CardProps = {
  children: React.ReactNode;
  className?: string;
};

const Card: React.FC<CardProps> = ({ children, className = '' }) => {
  return (
    <div className={`bg-white rounded-lg border border-gray-200 ${className}`}>
      {children}
    </div>
  );
};

export default Card;
```

## File: src/components/ui/EmptyState.tsx
```typescript
import React from 'react';

type EmptyStateProps = {
  icon?: React.ReactNode;
  title: string;
  subtitle?: string;
  action?: React.ReactNode;
  className?: string;
};

const EmptyState: React.FC<EmptyStateProps> = ({ icon, title, subtitle, action, className = '' }) => {
  return (
    <div className={`text-center py-8 text-gray-500 ${className}`}>
      {icon && <div className="mb-3 flex justify-center">{icon}</div>}
      <p className="font-medium">{title}</p>
      {subtitle && <p className="text-sm text-gray-400 mt-1">{subtitle}</p>}
      {action && <div className="mt-4">{action}</div>}
    </div>
  );
};

export default EmptyState;
```

## File: src/components/ui/Fieldset.tsx
```typescript
import React from 'react';

type FieldsetProps = {
  title: string;
  description?: React.ReactNode;
  actions?: React.ReactNode;
  children: React.ReactNode;
  className?: string;
};

const Fieldset: React.FC<FieldsetProps> = ({ title, description, actions, children, className = '' }) => {
  return (
    <section className={`mb-6 ${className}`}>
      <div className="flex items-start justify-between mb-3">
        <div>
          <h2 className="text-lg font-semibold text-gray-900">{title}</h2>
          {description && <p className="text-sm text-gray-600 mt-1">{description}</p>}
        </div>
        {actions && <div className="ml-4">{actions}</div>}
      </div>
      <div className="space-y-4">{children}</div>
    </section>
  );
};

export default Fieldset;
```

## File: src/components/ui/FormHelp.tsx
```typescript
import React from 'react';

const FormHelp: React.FC<{ children: React.ReactNode; className?: string }> = ({ children, className = '' }) => (
  <p className={`mt-1 text-sm text-gray-500 ${className}`}>{children}</p>
);

export default FormHelp;
```

## File: src/components/ui/FormLabel.tsx
```typescript
import React from 'react';

const FormLabel: React.FC<React.LabelHTMLAttributes<HTMLLabelElement>> = ({ children, className = '', ...rest }) => (
  <label className={`block text-sm font-medium text-gray-700 ${className}`} {...rest}>
    {children}
  </label>
);

export default FormLabel;
```

## File: src/components/ui/FormRow.tsx
```typescript
import React from 'react';

type FormRowProps = {
  children: React.ReactNode;
  inline?: boolean;
  className?: string;
};

const FormRow: React.FC<FormRowProps> = ({ children, inline = false, className = '' }) => {
  if (inline) {
    return <div className={`flex items-end gap-3 ${className}`}>{children}</div>;
  }
  return <div className={`space-y-2 ${className}`}>{children}</div>;
};

export default FormRow;
```

## File: src/components/ui/HeaderNav.tsx
```typescript
import React from 'react';

type HeaderNavProps = {
  title: string;
  subtitle?: React.ReactNode;
  actions?: React.ReactNode;
  className?: string;
};

const HeaderNav: React.FC<HeaderNavProps> = ({ title, subtitle, actions, className = '' }) => {
  return (
    <nav className={`bg-white shadow-sm border-b flex-shrink-0 ${className}`}>
      <div className="max-w-full mx-auto px-4 sm:px-6 lg:px-8">
        <div className="flex justify-between h-14">
          <div className="flex items-center">
            <h1 className="text-lg font-bold text-gray-900">{title}</h1>
            {subtitle && (
              <div className="ml-4 text-sm text-gray-600">{subtitle}</div>
            )}
          </div>
          <div className="flex items-center space-x-2">{actions}</div>
        </div>
      </div>
    </nav>
  );
};

export default HeaderNav;
```

## File: src/components/ui/SectionHeader.tsx
```typescript
import React from 'react';

type SectionHeaderProps = {
  title: string;
  actions?: React.ReactNode;
  className?: string;
};

const SectionHeader: React.FC<SectionHeaderProps> = ({ title, actions, className = '' }) => {
  return (
    <div className={`flex justify-between items-center mb-4 ${className}`}>
      <h2 className="text-lg font-semibold text-gray-900">{title}</h2>
      {actions}
    </div>
  );
};

export default SectionHeader;
```

## File: src/components/ui/Select.tsx
```typescript
import React from 'react';

export type Option = { value: string; label: string };

type SelectProps = React.SelectHTMLAttributes<HTMLSelectElement> & {
  label: string;
  helpText?: string;
  options?: Option[];
};

const Select: React.FC<SelectProps> = ({ label, id, helpText, className = '', options, children, ...rest }) => {
  const selectId = id || rest.name || Math.random().toString(36).slice(2);
  return (
    <div>
      <label htmlFor={selectId} className="block text-sm font-medium text-gray-700 mb-2">
        {label}
      </label>
      <select
        id={selectId}
        className={`w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent ${className}`}
        {...rest}
      >
        {options ? options.map((o) => (
          <option key={o.value} value={o.value}>{o.label}</option>
        )) : children}
      </select>
      {helpText && <p className="mt-1 text-sm text-gray-500">{helpText}</p>}
    </div>
  );
};

export default Select;
```

## File: src/components/ui/SidebarListItem.tsx
```typescript
import React from 'react';

type SidebarListItemProps = {
  selected?: boolean;
  onClick?: () => void;
  title: string;
  subtitle?: React.ReactNode;
  meta?: React.ReactNode;
  left?: React.ReactNode;
};

const SidebarListItem: React.FC<SidebarListItemProps> = ({ selected, onClick, title, subtitle, meta, left }) => {
  return (
    <div
      onClick={onClick}
      className={`${
        selected ? 'border-blue-500 bg-blue-50' : 'border-transparent'
      } p-4 cursor-pointer hover:bg-blue-50 border-l-4`}
    >
      <div className="flex items-start justify-between">
        <div className="flex-1 min-w-0">
          <div className="flex items-center space-x-2">
            <h4 className="font-medium text-gray-900 truncate text-sm">{title}</h4>
            {left}
          </div>
          {subtitle && (
            <p className="text-xs text-gray-600 mt-1 line-clamp-2">{subtitle}</p>
          )}
          {meta && <div className="flex items-center mt-2 text-xs text-gray-500 space-x-3">{meta}</div>}
        </div>
      </div>
    </div>
  );
};

export default SidebarListItem;
```

## File: src/components/ui/Spinner.tsx
```typescript
import React from 'react';

type SpinnerProps = {
  size?: 'sm' | 'md' | 'lg';
  color?: 'blue' | 'green' | 'gray';
  className?: string;
};

const sizeMap = {
  sm: 'h-5 w-5 border-b-2',
  md: 'h-8 w-8 border-b-2',
  lg: 'h-10 w-10 border-b-2',
};

const colorMap = {
  blue: 'border-blue-600',
  green: 'border-green-600',
  gray: 'border-gray-500',
};

const Spinner: React.FC<SpinnerProps> = ({ size = 'md', color = 'gray', className = '' }) => {
  return (
    <div
      className={`inline-block animate-spin rounded-full ${sizeMap[size]} ${colorMap[color]} ${className}`}
    />
  );
};

export default Spinner;
```

## File: src/components/ui/StatTile.tsx
```typescript
import React from 'react';

type Color = 'blue' | 'green' | 'purple' | 'gray';

type StatTileProps = {
  label: string;
  value: React.ReactNode;
  color?: Color;
};

const bgMap: Record<Color, string> = {
  blue: 'bg-blue-50 text-blue-900',
  green: 'bg-green-50 text-green-900',
  purple: 'bg-purple-50 text-purple-900',
  gray: 'bg-gray-50 text-gray-900',
};

const valueMap: Record<Color, string> = {
  blue: 'text-blue-600',
  green: 'text-green-600',
  purple: 'text-purple-600',
  gray: 'text-gray-700',
};

const StatTile: React.FC<StatTileProps> = ({ label, value, color = 'gray' }) => {
  return (
    <div className={`${bgMap[color]} rounded-lg p-4`}>
      <h3 className="font-semibold mb-1">{label}</h3>
      <p className={`text-2xl font-bold ${valueMap[color]}`}>{value}</p>
    </div>
  );
};

export default StatTile;
```

## File: src/components/ui/Tabs.tsx
```typescript
import React, { useState, useMemo } from 'react';

type Tab = {
  id: string;
  label: string;
  content: React.ReactNode;
  badge?: React.ReactNode;
};

type TabsProps = {
  tabs: Tab[];
  defaultTab?: string;
  className?: string;
};

const Tabs: React.FC<TabsProps> = ({ tabs, defaultTab, className = '' }) => {
  const initial = useMemo(() => defaultTab || (tabs[0] && tabs[0].id) || '', [defaultTab, tabs]);
  const [active, setActive] = useState<string>(initial);

  const activeTab = tabs.find(t => t.id === active) || tabs[0];

  return (
    <div className={className}>
      <div className="border-b border-gray-200">
        <nav className="-mb-px flex space-x-4" aria-label="Tabs">
          {tabs.map((t) => {
            const isActive = t.id === activeTab.id;
            return (
              <button
                key={t.id}
                onClick={() => setActive(t.id)}
                className={`px-3 py-2 border-b-2 text-sm font-medium flex items-center gap-2 ${
                  isActive
                    ? 'border-blue-600 text-blue-600'
                    : 'border-transparent text-gray-500 hover:text-gray-700 hover:border-gray-300'
                }`}
              >
                <span>{t.label}</span>
                {t.badge}
              </button>
            );
          })}
        </nav>
      </div>
      <div className="mt-4">{activeTab?.content}</div>
    </div>
  );
};

export default Tabs;
```

## File: src/components/ui/TextField.tsx
```typescript
import React from 'react';

type TextFieldProps = React.InputHTMLAttributes<HTMLInputElement> & {
  label: string;
  helpText?: string;
};

const TextField: React.FC<TextFieldProps> = ({ label, id, helpText, className = '', ...rest }) => {
  const inputId = id || rest.name || Math.random().toString(36).slice(2);
  return (
    <div>
      <label htmlFor={inputId} className="block text-sm font-medium text-gray-700 mb-2">
        {label}
      </label>
      <input
        id={inputId}
        className={`w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent ${className}`}
        {...rest}
      />
      {helpText && <p className="mt-1 text-sm text-gray-500">{helpText}</p>}
    </div>
  );
};

export default TextField;
```

## File: src/components/ui/ToastProvider.tsx
```typescript
import React, { createContext, useCallback, useContext, useMemo, useState } from 'react';

type ToastVariant = 'success' | 'error' | 'info';

type Toast = {
  id: string;
  title: string;
  description?: string;
  variant?: ToastVariant;
  duration?: number; // ms
};

type ToastContextValue = {
  toast: (t: Omit<Toast, 'id'>) => void;
};

const ToastContext = createContext<ToastContextValue | undefined>(undefined);

export const useToast = () => {
  const ctx = useContext(ToastContext);
  if (!ctx) throw new Error('useToast must be used within ToastProvider');
  return ctx;
};

const variantMap: Record<ToastVariant, string> = {
  success: 'bg-green-600',
  error: 'bg-red-600',
  info: 'bg-blue-600',
};

const ToastProvider: React.FC<{ children: React.ReactNode } & { position?: 'bottom-right' | 'top-right' | 'bottom-left' | 'top-left' }> = ({ children, position = 'bottom-right' }) => {
  const [toasts, setToasts] = useState<Toast[]>([]);

  const toast = useCallback((t: Omit<Toast, 'id'>) => {
    const id = Math.random().toString(36).slice(2);
    const item: Toast = { id, duration: 3500, variant: 'info', ...t };
    setToasts((prev) => [...prev, item]);
    window.setTimeout(() => {
      setToasts((prev) => prev.filter((x) => x.id !== id));
    }, item.duration);
  }, []);

  const value = useMemo(() => ({ toast }), [toast]);

  const posClass = useMemo(() => {
    const base = 'fixed z-50 space-y-2 p-4';
    switch (position) {
      case 'top-right':
        return `${base} top-0 right-0`;
      case 'top-left':
        return `${base} top-0 left-0`;
      case 'bottom-left':
        return `${base} bottom-0 left-0`;
      default:
        return `${base} bottom-0 right-0`;
    }
  }, [position]);

  return (
    <ToastContext.Provider value={value}>
      {children}
      <div className={posClass}>
        {toasts.map((t) => (
          <div key={t.id} className="shadow-lg rounded-md overflow-hidden min-w-[260px] max-w-sm">
            <div className={`${variantMap[t.variant || 'info']} text-white px-3 py-2 text-sm font-medium`}>{t.title}</div>
            {t.description && (
              <div className="bg-white px-3 py-2 text-sm text-gray-700 border border-t-0 border-gray-200">{t.description}</div>
            )}
          </div>
        ))}
      </div>
    </ToastContext.Provider>
  );
};

export default ToastProvider;
```

## File: src/components/MarkdownRenderer.tsx
```typescript
import React from 'react';

// Very lightweight markdown renderer; safe for short, trusted content.
function toHtml(text: string): string {
  let html = text;

  // Headers
  html = html.replace(/^### (.*$)/gim, '<h3>$1</h3>');
  html = html.replace(/^## (.*$)/gim, '<h2>$1</h2>');
  html = html.replace(/^# (.*$)/gim, '<h1>$1</h1>');

  // Bold / Italic
  html = html.replace(/\*\*(.*?)\*\*/g, '<strong>$1</strong>');
  html = html.replace(/__(.*?)__/g, '<strong>$1</strong>');
  html = html.replace(/\*(.*?)\*/g, '<em>$1</em>');
  html = html.replace(/_(.*?)_/g, '<em>$1</em>');

  // Inline code
  html = html.replace(/`(.*?)`/g, '<code class="bg-gray-100 px-1 py-0.5 rounded text-sm font-mono">$1</code>');

  // Lists
  html = html.replace(/^\s*\* (.*$)/gim, '<li>$1</li>');
  html = html.replace(/^\s*- (.*$)/gim, '<li>$1</li>');
  html = html.replace(/^\s*\+ (.*$)/gim, '<li>$1</li>');
  html = html.replace(/^\s*\d+\.\s+(.*$)/gim, '<li>$1</li>');

  // Wrap consecutive list items
  html = html.replace(/(<li>.*<\/li>)/gs, (match) => {
    return `<ul class="list-disc list-inside space-y-1 my-2">${match}</ul>`;
  });
  html = html.replace(/<\/ul>\s*<ul[^>]*>/g, '');

  // Paragraphs
  html = html.replace(/\n\n/g, '</p><p>');
  html = `<p>${html}</p>`;
  html = html.replace(/<p><\/p>/g, '');
  html = html.replace(/<p>\s*<\/p>/g, '');
  html = html.replace(/<p>(<h[1-6]>.*?<\/h[1-6]>)<\/p>/g, '$1');
  html = html.replace(/<p>(<ul.*?<\/ul>)<\/p>/gs, '$1');

  return html;
}

type MarkdownRendererProps = {
  markdown: string;
  className?: string;
};

const MarkdownRenderer: React.FC<MarkdownRendererProps> = ({ markdown, className }) => {
  const html = toHtml(markdown || '');
  return <div className={className} dangerouslySetInnerHTML={{ __html: html }} />;
};

export default MarkdownRenderer;
```

## File: src/components/ProjectHeader.tsx
```typescript
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
    <div className="p-6 border-b border-gray-200 bg-white">
      <div className="flex items-center space-x-3">
        <div className="flex-shrink-0">
          <div className="w-10 h-10 bg-blue-100 rounded-lg flex items-center justify-center">
            <Folder className="w-5 h-5 text-blue-600" />
          </div>
        </div>
        <div className="flex-1">
          <div className="flex items-center space-x-2">
            <h1 className="text-xl font-bold text-gray-900">{name}</h1>
            {isGitRepo && <Badge variant="green">Git Repo</Badge>}
          </div>
          {description && <p className="text-gray-600 mt-1">{description}</p>}
          <p className="text-sm text-gray-500 mt-1">
            {typeof fileCount === 'number' ? `${fileCount} files • ` : ''}{path}
          </p>
        </div>
      </div>
    </div>
  );
};

export default ProjectHeader;
```

## File: src/components/ProjectInsights.tsx
```typescript
import React from 'react';
import Card from './ui/Card';
import Badge from './ui/Badge';
import { ProjectInsights } from '../types';

type Props = {
  insights: ProjectInsights;
};

const Section: React.FC<{ title: string; children: React.ReactNode }> = ({ title, children }) => (
  <Card className="p-4">
    <h3 className="text-sm font-semibold text-gray-900 mb-3">{title}</h3>
    {children}
  </Card>
);

const Row: React.FC<{ label: string; value: React.ReactNode }> = ({ label, value }) => (
  <div className="flex justify-between text-sm py-1">
    <span className="text-gray-600">{label}</span>
    <span className="text-gray-900">{value}</span>
  </div>
);

const ProjectInsightsComponent: React.FC<Props> = ({ insights }) => {
  const { git_status, readme_info, ci_info, package_info, testing_info } = insights;

  return (
    <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
      <Section title="Git">
        <Row label="Repository" value={git_status.is_git_repo ? <Badge variant="green">Yes</Badge> : <Badge variant="gray">No</Badge>} />
        {git_status.current_branch && <Row label="Branch" value={git_status.current_branch} />}
        {typeof git_status.commit_count === 'number' && <Row label="Commits" value={git_status.commit_count} />}
      </Section>

      <Section title="README">
        <Row label="Exists" value={readme_info.exists ? <Badge variant="green">Yes</Badge> : <Badge variant="red">No</Badge>} />
        {readme_info.path && <Row label="Path" value={<span className="truncate max-w-[220px] inline-block align-bottom" title={readme_info.path}>{readme_info.path}</span>} />}
      </Section>

      <Section title="CI">
        <Row label="Configured" value={ci_info.has_ci ? <Badge variant="green">Yes</Badge> : <Badge variant="red">No</Badge>} />
        {ci_info.ci_platforms.length > 0 && (
          <div className="mt-1 flex flex-wrap gap-2">
            {ci_info.ci_platforms.map((p) => (
              <Badge key={p} variant="blue">{p}</Badge>
            ))}
          </div>
        )}
      </Section>

      <Section title="Packages">
        <div className="flex flex-wrap gap-2 text-xs">
          {package_info.has_package_json && <Badge variant="gray">package.json</Badge>}
          {package_info.has_cargo_toml && <Badge variant="gray">Cargo.toml</Badge>}
          {package_info.has_requirements_txt && <Badge variant="gray">requirements.txt</Badge>}
          {package_info.has_gemfile && <Badge variant="gray">Gemfile</Badge>}
          {package_info.has_go_mod && <Badge variant="gray">go.mod</Badge>}
        </div>
        {package_info.missing_common_files.length > 0 && (
          <div className="mt-2">
            <div className="text-xs text-gray-600 mb-1">Missing common files:</div>
            <div className="flex flex-wrap gap-2 text-xs">
              {package_info.missing_common_files.map((f) => (
                <Badge key={f} variant="red">{f}</Badge>
              ))}
            </div>
          </div>
        )}
      </Section>

      <Section title="Testing">
        <Row label="Has Framework" value={testing_info.has_testing_framework ? <Badge variant="green">Yes</Badge> : <Badge variant="red">No</Badge>} />
        <Row label="Test Files" value={testing_info.has_test_files ? testing_info.test_file_count : 0} />
        {typeof testing_info.source_to_test_ratio === 'number' && (
          <Row label="Source/Test Ratio" value={testing_info.source_to_test_ratio?.toFixed(2)} />
        )}
        {testing_info.testing_frameworks.length > 0 && (
          <div className="mt-1 flex flex-wrap gap-2 text-xs">
            {testing_info.testing_frameworks.map((f) => (
              <Badge key={f} variant="gray">{f}</Badge>
            ))}
          </div>
        )}
      </Section>
    </div>
  );
};

export default ProjectInsightsComponent;
```

## File: src/utils/format.ts
```typescript
export const basename = (p: string): string => {
  if (!p) return '';
  const parts = p.split(/[/\\]/);
  return parts[parts.length - 1] || p;
};

export const numberWithCommas = (n?: number | null): string => {
  if (n === undefined || n === null) return '0';
  return n.toLocaleString();
};
```

## File: src/utils/models.ts
```typescript
export const isThinkingModel = (modelId: string): boolean => {
  if (!modelId) return false;
  const id = modelId.toLowerCase();
  const thinking = ['o1', 'o1-mini', 'o1-preview', 'deepseek-r1'];
  return thinking.some(name => id.includes(name));
};
```

## File: src-tauri/src/ai.rs
```rust
use crate::analysis::RepoAnalysis;
use crate::storage::{ProjectSummary, Settings};
use regex::Regex;
use reqwest::header::{HeaderMap, AUTHORIZATION, CONTENT_TYPE};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ModelInfo {
    pub id: String,
    pub name: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ModelsResponse {
    pub data: Vec<ModelInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IdeaRequest {
    pub analysis: RepoAnalysis,
    pub settings: Settings,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SummaryRequest {
    pub analysis: RepoAnalysis,
    pub settings: Settings,
    pub project_path: String,
}

fn extract_thinking_and_response(content: &str) -> (Option<String>, String) {
    let re = Regex::new(r"(?s)<think>(.*?)</think>(.*)").unwrap();
    if let Some(captures) = re.captures(content) {
        let thinking = captures.get(1).map(|m| m.as_str().trim().to_string());
        let response = captures
            .get(2)
            .map(|m| m.as_str().trim().to_string())
            .unwrap_or_default();
        (thinking, response)
    } else {
        (None, content.to_string())
    }
}

fn parse_structured_response(content: &str) -> Vec<String> {
    let lines: Vec<&str> = content.lines().collect();
    let mut ideas = Vec::new();
    let mut current_idea = String::new();
    for line in lines {
        let line = line.trim();
        let is_new_idea = line
            .chars()
            .next()
            .map(|c| c.is_numeric())
            .unwrap_or(false) && line.contains('.')
            || line.starts_with("• ")
            || line.starts_with("- ")
            || line.starts_with("* ");
        if is_new_idea && !current_idea.trim().is_empty() {
            ideas.push(current_idea.trim().to_string());
            current_idea.clear();
        }
        if is_new_idea {
            let cleaned = line
                .trim_start_matches(char::is_numeric)
                .trim_start_matches('.')
                .trim_start_matches(')')
                .trim_start_matches("• ")
                .trim_start_matches("- ")
                .trim_start_matches("* ")
                .trim();
            current_idea.push_str(cleaned);
        } else if !current_idea.is_empty() {
            current_idea.push(' ');
            current_idea.push_str(line);
        } else {
            current_idea.push_str(line);
        }
    }
    if !current_idea.trim().is_empty() {
        ideas.push(current_idea.trim().to_string());
    }
    ideas.into_iter().filter(|idea| idea.len() > 20).collect()
}

fn extract_key_features(summary: &str) -> Vec<String> {
    let mut features = Vec::new();
    let lines: Vec<&str> = summary.lines().collect();
    for line in lines {
        let trimmed = line.trim();
        if trimmed.starts_with('-') || trimmed.starts_with('•') {
            let feature = trimmed
                .trim_start_matches('-')
                .trim_start_matches('•')
                .trim()
                .to_string();
            if !feature.is_empty() && feature.len() < 200 {
                features.push(feature);
            }
        }
    }
    features
}

#[tauri::command]
pub async fn load_models(api_url: String, api_key: String) -> Result<Vec<ModelInfo>, String> {
    let client = reqwest::Client::new();
    let model_endpoints = vec![
        format!("{}/models", api_url.replace("/chat/completions", "")),
        format!(
            "{}/v1/models",
            api_url
                .replace("/v1/chat/completions", "")
                .replace("/chat/completions", "")
        ),
    ];
    for endpoint in model_endpoints {
        let mut headers = HeaderMap::new();
        if !api_key.is_empty() {
            headers.insert(AUTHORIZATION, format!("Bearer {}", api_key).parse().unwrap());
        }
        match client.get(&endpoint).headers(headers).send().await {
            Ok(response) => {
                let status = response.status();
                let response_text = response.text().await.unwrap_or_default();
                if status.is_success() {
                    if let Ok(models_response) = serde_json::from_str::<ModelsResponse>(&response_text) {
                        return Ok(models_response.data);
                    }
                    if let Ok(models_json) = serde_json::from_str::<serde_json::Value>(&response_text) {
                        if let Some(models_array) = models_json["models"].as_array() {
                            let models: Vec<ModelInfo> = models_array
                                .iter()
                                .filter_map(|model| {
                                    if let Some(name) = model["name"].as_str() {
                                        Some(ModelInfo {
                                            id: name.to_string(),
                                            name: Some(name.to_string()),
                                            description: model["details"]["parameter_size"].as_str().map(|s| s.to_string()),
                                        })
                                    } else {
                                        None
                                    }
                                })
                                .collect();
                            if !models.is_empty() { return Ok(models); }
                        }
                    }
                }
            }
            Err(_) => continue,
        }
    }
    Err("Unable to load models from API. Please check your API URL and key.".to_string())
}

#[tauri::command]
pub async fn generate_ideas(request: IdeaRequest) -> Result<Vec<String>, String> {
    let client = reqwest::Client::new();
    let prompt = format!(
        "Analyze this code repository and generate 5-10 creative, actionable development ideas.

Repository Analysis:
- Technologies: {}
- Total Files: {}
- Total Lines: {}
- Directory Structure: {} directories analyzed

Key Files Preview:
{}

Please provide specific, actionable suggestions for:
1. Code improvements and refactoring opportunities
2. New features that would enhance the project
3. Architecture improvements
4. Developer experience enhancements
5. Performance optimizations
6. Testing strategies
7. Documentation improvements

IMPORTANT: Format your response as a numbered list with one idea per line.",
        request.analysis.technologies.join(", "),
        request.analysis.metrics.get("total_files").unwrap_or(&0),
        request.analysis.metrics.get("total_lines").unwrap_or(&0),
        request.analysis.structure.len(),
        request
            .analysis
            .files
            .iter()
            .take(10)
            .map(|f| format!(
                "{} ({}): {}",
                f.path,
                f.language,
                if f.content.len() > 200 {
                    format!("{}...", &f.content[..200])
                } else {
                    f.content.clone()
                }
            ))
            .collect::<Vec<_>>()
            .join("\n\n")
    );

    let mut headers = HeaderMap::new();
    if !request.settings.api_key.is_empty() {
        headers.insert(AUTHORIZATION, format!("Bearer {}", request.settings.api_key).parse().unwrap());
    }
    headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());

    let body = serde_json::json!({
        "model": request.settings.model,
        "messages": [
            {
                "role": "system",
                "content": "You are a senior software engineer and architect who provides creative, practical development ideas for code repositories. Always format your response as a clear numbered list with one idea per line."
            },
            { "role": "user", "content": prompt }
        ],
        "max_tokens": 2000,
        "temperature": 0.8
    });

    let response = client
        .post(&request.settings.api_url)
        .headers(headers)
        .json(&body)
        .send()
        .await
        .map_err(|e| format!("Request failed: {}", e))?;

    let response_json: serde_json::Value = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse response: {}", e))?;

    if let Some(choices) = response_json["choices"].as_array() {
        if let Some(choice) = choices.first() {
            if let Some(message) = choice["message"]["content"].as_str() {
                let (_thinking, content) = extract_thinking_and_response(message);
                let ideas = parse_structured_response(&content);
                return Ok(ideas);
            }
        }
    }
    Err("Failed to generate ideas".to_string())
}

#[tauri::command]
pub async fn generate_project_summary(request: SummaryRequest) -> Result<ProjectSummary, String> {
    let client = reqwest::Client::new();
    let file_previews: Vec<String> = request
        .analysis
        .files
        .iter()
        .take(15)
        .map(|f| {
            let preview = if f.content.len() > 300 {
                format!("{}...", &f.content[..300])
            } else {
                f.content.clone()
            };
            format!("File: {} ({})\nContent snippet:\n{}\n", f.path, f.language, preview)
        })
        .collect();

    let prompt = format!(
        "Analyze this code repository and create a clear, focused summary that explains what this application/project does.

Repository Analysis:
- Technologies: {}
- Total Files: {}
- Total Lines: {}
- Directory Structure: {} directories analyzed

File Previews:
{}

Please provide a summary that focuses on:
1. What this application/project does
2. Core features
3. Technical implementation
4. Project type

Write the summary in clear, accessible language.",
        request.analysis.technologies.join(", "),
        request.analysis.metrics.get("total_files").unwrap_or(&0),
        request.analysis.metrics.get("total_lines").unwrap_or(&0),
        request.analysis.structure.len(),
        file_previews.join("\n---\n")
    );

    let mut headers = HeaderMap::new();
    if !request.settings.api_key.is_empty() {
        headers.insert(AUTHORIZATION, format!("Bearer {}", request.settings.api_key).parse().unwrap());
    }
    headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());

    let body = serde_json::json!({
        "model": request.settings.model,
        "messages": [
            { "role": "system", "content": "You are a technical documentation specialist who creates clear, concise project summaries." },
            { "role": "user", "content": prompt }
        ],
        "max_tokens": 1000,
        "temperature": 0.7
    });

    let response = client
        .post(&request.settings.api_url)
        .headers(headers)
        .json(&body)
        .send()
        .await
        .map_err(|e| format!("Request failed: {}", e))?;

    let response_json: serde_json::Value = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse response: {}", e))?;

    if let Some(choices) = response_json["choices"].as_array() {
        if let Some(choice) = choices.first() {
            if let Some(message) = choice["message"]["content"].as_str() {
                let (_thinking, summary_text) = extract_thinking_and_response(message);
                let key_features = extract_key_features(&summary_text);
                let summary = ProjectSummary {
                    project_path: request.project_path,
                    summary: summary_text,
                    generated_at: chrono::Utc::now().to_rfc3339(),
                    technologies: request.analysis.technologies.clone(),
                    key_features,
                };
                return Ok(summary);
            }
        }
    }
    Err("Failed to generate summary".to_string())
}
```

## File: src-tauri/src/analysis.rs
```rust
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;
use std::time::{Duration, SystemTime};
use walkdir::WalkDir;
use chrono::{DateTime, Utc};

use crate::cache::{load_analysis_cache, save_analysis_cache, AnalysisCacheEntry};
use crate::fs_utils::{get_dir_modified_time, get_language_from_extension, should_analyze_file};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FileInfo {
  pub path: String,
  pub content: String,
  pub language: String,
  pub size: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RepoAnalysis {
  pub files: Vec<FileInfo>,
  pub structure: HashMap<String, Vec<String>>,
  pub technologies: Vec<String>,
  pub metrics: HashMap<String, i32>,
  pub generated_at: Option<String>,
  pub from_cache: Option<bool>,
}

async fn analyze_repository_impl(folder_path: String, force: bool) -> Result<RepoAnalysis, String> {
  let path = Path::new(&folder_path);
  if !path.exists() || !path.is_dir() {
    return Err("Invalid folder path".to_string());
  }

  // Cache for up to 1 hour
  let last_modified = get_dir_modified_time(path);
  let now = SystemTime::now()
    .duration_since(SystemTime::UNIX_EPOCH)
    .unwrap_or(Duration::from_secs(0))
    .as_secs();
  const TTL_SECS: u64 = 3600;

  let mut cache = load_analysis_cache();
  if !force {
    if let Some(entry) = cache.get(&folder_path).cloned() {
      if entry.last_modified >= last_modified && (now - entry.cached_at) < TTL_SECS {
        let mut a = entry.analysis.clone();
        a.from_cache = Some(true);
        let ts: DateTime<Utc> = DateTime::<Utc>::from_timestamp(entry.cached_at as i64, 0)
          .unwrap_or_else(|| Utc::now());
        a.generated_at = Some(ts.to_rfc3339());
        return Ok(a);
      }
    }
  }

  // Collect candidate files and process in parallel
  let valid_files: Vec<_> = WalkDir::new(&folder_path)
    .into_iter()
    .filter_map(|e| e.ok())
    .filter(|entry| entry.path().is_file())
    .filter(|entry| should_analyze_file(&entry.path().to_string_lossy()))
    .map(|entry| entry.path().to_owned())
    .collect();

  #[derive(Clone)]
  struct FileProcessResult {
    file_info: Option<FileInfo>,
    lines: usize,
    language: String,
    parent: Option<String>,
    path: String,
  }

  let results: Vec<FileProcessResult> = valid_files
    .par_iter()
    .filter_map(|path| {
      let path_str = path.to_string_lossy().to_string();
      match fs::read_to_string(path) {
        Ok(content) => {
          let lines = content.lines().count();
          let language = get_language_from_extension(&path_str);
          let include = content.len() < 100_000;
          let file_info = if include {
            Some(FileInfo {
              path: path_str.clone(),
              content: if content.len() > 5000 {
                format!("{}...(truncated)", &content[..5000])
              } else {
                content
              },
              language: language.clone(),
              size: path.metadata().map(|m| m.len()).unwrap_or(0),
            })
          } else { None };
          let parent = path.parent().map(|p| p.to_string_lossy().to_string());
          Some(FileProcessResult { file_info, lines, language, parent, path: path_str })
        }
        Err(_) => None,
      }
    })
    .collect();

  let mut files: Vec<FileInfo> = Vec::new();
  let mut structure: HashMap<String, Vec<String>> = HashMap::new();
  let mut technologies_set: HashSet<String> = HashSet::new();
  let mut total_files: i32 = 0;
  let mut total_lines: i32 = 0;

  for r in &results {
    total_files += 1;
    total_lines += r.lines as i32;
    if r.language != "Unknown" { technologies_set.insert(r.language.clone()); }
    if let Some(ref fi) = r.file_info {
      files.push(fi.clone());
      if let Some(parent) = &r.parent {
        let name = Path::new(&r.path).file_name().unwrap_or_default().to_string_lossy().to_string();
        structure.entry(parent.clone()).or_default().push(name);
      }
    }
  }

  let technologies: Vec<String> = technologies_set.into_iter().collect();
  let mut metrics = HashMap::new();
  metrics.insert("total_files".to_string(), total_files);
  metrics.insert("total_lines".to_string(), total_lines);
  metrics.insert("analyzed_files".to_string(), files.len() as i32);

  let mut analysis = RepoAnalysis { files, structure, technologies, metrics, generated_at: None, from_cache: Some(false) };
  analysis.generated_at = Some(Utc::now().to_rfc3339());

  cache.insert(
    folder_path.clone(),
    AnalysisCacheEntry { path: folder_path, last_modified, cached_at: now, analysis: analysis.clone() },
  );
  save_analysis_cache(&cache);

  Ok(analysis)
}

#[tauri::command]
pub async fn analyze_repository(folder_path: String) -> Result<RepoAnalysis, String> {
  analyze_repository_impl(folder_path, false).await
}

#[tauri::command]
pub async fn analyze_repository_fresh(folder_path: String) -> Result<RepoAnalysis, String> {
  analyze_repository_impl(folder_path, true).await
}
```

## File: src-tauri/src/cache.rs
```rust
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FileCountCache {
    pub path: String,
    pub count: usize,
    pub last_modified: u64,
    pub cached_at: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AnalysisCacheEntry {
    pub path: String,
    pub last_modified: u64,
    pub cached_at: u64,
    pub analysis: crate::analysis::RepoAnalysis,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProjectMetaCacheEntry {
    pub path: String,
    pub description: Option<String>,
    pub is_git_repo: bool,
    pub last_modified: u64,
    pub cached_at: u64,
}

fn app_data_dir() -> Option<std::path::PathBuf> {
    dirs::data_local_dir().map(|d| d.join("repomuse"))
}

pub fn load_file_count_cache() -> HashMap<String, FileCountCache> {
    let cache_path = match app_data_dir() { Some(d) => d.join("file_count_cache.json"), None => return HashMap::new() };
    if let Ok(s) = fs::read_to_string(cache_path) {
        if let Ok(map) = serde_json::from_str(&s) { return map; }
    }
    HashMap::new()
}

pub fn save_file_count_cache(cache: &HashMap<String, FileCountCache>) {
    if let Some(dir) = app_data_dir() {
        let _ = fs::create_dir_all(&dir);
        let path = dir.join("file_count_cache.json");
        if let Ok(json) = serde_json::to_string_pretty(cache) {
            let _ = fs::write(path, json);
        }
    }
}

pub fn clear_file_count_cache_file() -> Result<(), String> {
    let dir = app_data_dir().ok_or("Failed to get app data directory")?;
    let cache_path = dir.join("file_count_cache.json");
    if cache_path.exists() { fs::remove_file(cache_path).map_err(|e| e.to_string())?; }
    Ok(())
}

pub fn load_analysis_cache() -> HashMap<String, AnalysisCacheEntry> {
    let cache_path = match app_data_dir() { Some(d) => d.join("analysis_cache.json"), None => return HashMap::new() };
    if let Ok(s) = fs::read_to_string(cache_path) {
        if let Ok(map) = serde_json::from_str(&s) { return map; }
    }
    HashMap::new()
}

pub fn save_analysis_cache(cache: &HashMap<String, AnalysisCacheEntry>) {
    if let Some(dir) = app_data_dir() {
        let _ = fs::create_dir_all(&dir);
        let path = dir.join("analysis_cache.json");
        if let Ok(json) = serde_json::to_string_pretty(cache) {
            let _ = fs::write(path, json);
        }
    }
}

pub fn load_project_meta_cache() -> HashMap<String, ProjectMetaCacheEntry> {
    let cache_path = match app_data_dir() { Some(d) => d.join("project_meta_cache.json"), None => return HashMap::new() };
    if let Ok(s) = fs::read_to_string(cache_path) {
        if let Ok(map) = serde_json::from_str(&s) { return map; }
    }
    HashMap::new()
}

pub fn save_project_meta_cache(cache: &HashMap<String, ProjectMetaCacheEntry>) {
    if let Some(dir) = app_data_dir() {
        let _ = fs::create_dir_all(&dir);
        let path = dir.join("project_meta_cache.json");
        if let Ok(json) = serde_json::to_string_pretty(cache) {
            let _ = fs::write(path, json);
        }
    }
}
```

## File: src-tauri/src/fs_utils.rs
```rust
use std::fs;
use std::path::Path;
use std::time::{Duration, SystemTime};

// Determine language from file extension
pub fn get_language_from_extension(path: &str) -> String {
    match Path::new(path).extension().and_then(|ext| ext.to_str()) {
        Some("rs") => "Rust".to_string(),
        Some("js") | Some("jsx") => "JavaScript".to_string(),
        Some("ts") | Some("tsx") => "TypeScript".to_string(),
        Some("py") => "Python".to_string(),
        Some("java") => "Java".to_string(),
        Some("cpp") | Some("cc") | Some("cxx") => "C++".to_string(),
        Some("c") => "C".to_string(),
        Some("go") => "Go".to_string(),
        Some("php") => "PHP".to_string(),
        Some("rb") => "Ruby".to_string(),
        Some("cs") => "C#".to_string(),
        Some("swift") => "Swift".to_string(),
        Some("kt") => "Kotlin".to_string(),
        Some("html") => "HTML".to_string(),
        Some("css") => "CSS".to_string(),
        Some("scss") | Some("sass") => "SCSS".to_string(),
        Some("json") => "JSON".to_string(),
        Some("xml") => "XML".to_string(),
        Some("yml") | Some("yaml") => "YAML".to_string(),
        Some("toml") => "TOML".to_string(),
        Some("md") => "Markdown".to_string(),
        _ => "Unknown".to_string(),
    }
}

// Filter files we should analyze
pub fn should_analyze_file(path: &str) -> bool {
    let ignore_extensions = vec![
        "png", "jpg", "jpeg", "gif", "svg", "ico", "woff", "woff2", "ttf", "eot", "pdf", "zip", "tar", "gz",
    ];
    let ignore_dirs = vec![
        "node_modules", "target", "build", "dist", ".git", ".svn", "vendor", "__pycache__",
    ];

    for ignore_dir in ignore_dirs {
        if path.contains(&format!("/{}/", ignore_dir)) || path.contains(&format!("\\{}\\", ignore_dir)) {
            return false;
        }
    }

    if let Some(ext) = Path::new(path).extension().and_then(|ext| ext.to_str()) {
        return !ignore_extensions.contains(&ext);
    }

    true
}

// Directory last modified seconds since epoch
pub fn get_dir_modified_time(path: &Path) -> u64 {
    if let Ok(metadata) = fs::metadata(path) {
        if let Ok(modified) = metadata.modified() {
            return modified
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap_or(Duration::from_secs(0))
                .as_secs();
        }
    }
    0
}
```

## File: src-tauri/src/insights.rs
```rust
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

#[derive(Debug, Serialize, Deserialize)]
pub struct GitStatus {
  pub is_git_repo: bool,
  pub has_uncommitted_changes: bool,
  pub uncommitted_files: Vec<String>,
  pub current_branch: Option<String>,
  pub last_commit_date: Option<String>,
  pub commit_count: Option<usize>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReadmeInfo {
  pub exists: bool,
  pub is_default: bool,
  pub path: Option<String>,
  pub content_preview: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CIInfo {
  pub has_ci: bool,
  pub ci_platforms: Vec<String>,
  pub ci_files: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PackageInfo {
  pub has_package_json: bool,
  pub has_cargo_toml: bool,
  pub has_requirements_txt: bool,
  pub has_gemfile: bool,
  pub has_go_mod: bool,
  pub missing_common_files: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TestingInfo {
  pub has_testing_framework: bool,
  pub testing_frameworks: Vec<String>,
  pub has_test_files: bool,
  pub test_file_count: usize,
  pub test_file_patterns: Vec<String>,
  pub source_to_test_ratio: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectInsights {
  pub git_status: GitStatus,
  pub readme_info: ReadmeInfo,
  pub ci_info: CIInfo,
  pub package_info: PackageInfo,
  pub testing_info: TestingInfo,
}

fn get_git_status(path: &Path) -> GitStatus {
  let is_git_repo = path.join(".git").exists();
  GitStatus {
    is_git_repo,
    has_uncommitted_changes: false,
    uncommitted_files: vec![],
    current_branch: None,
    last_commit_date: None,
    commit_count: None,
  }
}

fn get_readme_info(path: &Path) -> ReadmeInfo {
  let candidates = ["README.md", "README.txt", "readme.md", "readme.txt"]; 
  for name in candidates.iter() {
    let p = path.join(name);
    if p.exists() {
      let preview = fs::read_to_string(&p).ok().map(|s| s.chars().take(200).collect());
      return ReadmeInfo { exists: true, is_default: false, path: Some(p.to_string_lossy().to_string()), content_preview: preview };
    }
  }
  ReadmeInfo { exists: false, is_default: false, path: None, content_preview: None }
}

fn get_ci_info(path: &Path) -> CIInfo {
  let mut ci_platforms = Vec::new();
  let mut ci_files = Vec::new();
  if path.join(".github").join("workflows").exists() {
    ci_platforms.push("GitHub Actions".to_string());
    if let Ok(entries) = fs::read_dir(path.join(".github/workflows")) {
      for e in entries.flatten() {
        ci_files.push(format!(".github/workflows/{}", e.file_name().to_string_lossy()));
      }
    }
  }
  if path.join(".gitlab-ci.yml").exists() { ci_platforms.push("GitLab CI".to_string()); ci_files.push(".gitlab-ci.yml".to_string()); }
  if path.join(".travis.yml").exists() { ci_platforms.push("Travis CI".to_string()); ci_files.push(".travis.yml".to_string()); }
  if path.join(".circleci").join("config.yml").exists() { ci_platforms.push("CircleCI".to_string()); ci_files.push(".circleci/config.yml".to_string()); }
  if path.join("Jenkinsfile").exists() { ci_platforms.push("Jenkins".to_string()); ci_files.push("Jenkinsfile".to_string()); }
  if path.join("azure-pipelines.yml").exists() { ci_platforms.push("Azure Pipelines".to_string()); ci_files.push("azure-pipelines.yml".to_string()); }
  if path.join(".buildkite").exists() { ci_platforms.push("Buildkite".to_string()); ci_files.push(".buildkite/".to_string()); }
  CIInfo { has_ci: !ci_platforms.is_empty(), ci_platforms, ci_files }
}

fn get_package_info(path: &Path) -> PackageInfo {
  let has_package_json = path.join("package.json").exists();
  let has_cargo_toml = path.join("Cargo.toml").exists();
  let has_requirements_txt = path.join("requirements.txt").exists();
  let has_gemfile = path.join("Gemfile").exists();
  let has_go_mod = path.join("go.mod").exists();
  let mut missing = Vec::new();
  for (file, exists) in [
    ("README.md", path.join("README.md").exists()),
    ("LICENSE", path.join("LICENSE").exists()),
    (".gitignore", path.join(".gitignore").exists()),
  ] {
    if !exists { missing.push(file.to_string()); }
  }
  PackageInfo { has_package_json, has_cargo_toml, has_requirements_txt, has_gemfile, has_go_mod, missing_common_files: missing }
}

fn get_testing_info(path: &Path) -> TestingInfo {
  let mut frameworks = Vec::new();
  if let Ok(package_json) = fs::read_to_string(path.join("package.json")) {
    if let Ok(json) = serde_json::from_str::<serde_json::Value>(&package_json) {
      let mut check = |deps: &serde_json::Value| {
        if let Some(obj) = deps.as_object() {
          for (key, _) in obj {
            match key.as_str() {
              "jest" => frameworks.push("Jest".to_string()),
              "vitest" => frameworks.push("Vitest".to_string()),
              "mocha" => frameworks.push("Mocha".to_string()),
              "jasmine" => frameworks.push("Jasmine".to_string()),
              "cypress" => frameworks.push("Cypress".to_string()),
              "playwright" => frameworks.push("Playwright".to_string()),
              _ => {}
            }
          }
        }
      };
      if let Some(deps) = json["dependencies"].as_object() { check(&serde_json::Value::Object(deps.clone())); }
      if let Some(dev) = json["devDependencies"].as_object() { check(&serde_json::Value::Object(dev.clone())); }
    }
  }

  let mut test_file_count = 0usize;
  let mut source_file_count = 0usize;
  let mut patterns: Vec<String> = Vec::new();
  if let Ok(entries) = WalkDir::new(path).max_depth(4).into_iter().collect::<Result<Vec<_>, _>>() {
    for entry in entries {
      if entry.path().is_file() {
        let path_str = entry.path().to_string_lossy();
        let name = entry.path().file_name().unwrap_or_default().to_string_lossy().to_string();
        if name.ends_with(".test.js") || name.ends_with(".test.ts") || name.ends_with(".test.jsx") || name.ends_with(".test.tsx")
          || name.ends_with(".spec.js") || name.ends_with(".spec.ts") || name.ends_with(".spec.jsx") || name.ends_with(".spec.tsx")
          || name.starts_with("test_") && name.ends_with(".py") || name.ends_with("_test.py")
          || name.ends_with("_test.go") || name.ends_with("_spec.rb") || name.ends_with("_test.rb")
          || path_str.contains("/test/") || path_str.contains("/tests/") || path_str.contains("/__tests__/") || path_str.contains("/spec/") {
          test_file_count += 1;
          if !patterns.contains(&name) { patterns.push(name.clone()); }
        } else {
          let is_source = name.ends_with(".js") || name.ends_with(".ts") || name.ends_with(".jsx") || name.ends_with(".tsx") || name.ends_with(".py") || name.ends_with(".rs") || name.ends_with(".go") || name.ends_with(".rb") || name.ends_with(".java") || name.ends_with(".cs") || name.ends_with(".php") || name.ends_with(".cpp") || name.ends_with(".c");
          if is_source { source_file_count += 1; }
        }
      }
    }
  }

  let ratio = if test_file_count > 0 { Some(source_file_count as f64 / test_file_count as f64) } else { None };
  frameworks.sort(); frameworks.dedup();
  patterns.sort(); patterns.dedup(); patterns.truncate(10);

  TestingInfo { has_testing_framework: !frameworks.is_empty(), testing_frameworks: frameworks, has_test_files: test_file_count>0, test_file_count, test_file_patterns: patterns, source_to_test_ratio: ratio }
}

#[tauri::command]
pub async fn get_project_insights(project_path: String) -> Result<ProjectInsights, String> {
  let path = Path::new(&project_path);
  if !path.exists() || !path.is_dir() { return Err("Invalid project path".to_string()); }
  let git_status = get_git_status(path);
  let readme_info = get_readme_info(path);
  let ci_info = get_ci_info(path);
  let package_info = get_package_info(path);
  let testing_info = get_testing_info(path);
  Ok(ProjectInsights { git_status, readme_info, ci_info, package_info, testing_info })
}
```

## File: src-tauri/src/projects.rs
```rust
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::time::{Duration, SystemTime};
use walkdir::WalkDir;

use crate::cache::{
    clear_file_count_cache_file, load_file_count_cache, load_project_meta_cache, save_file_count_cache,
    save_project_meta_cache, FileCountCache, ProjectMetaCacheEntry,
};
use crate::fs_utils::{get_dir_modified_time, should_analyze_file};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProjectDirectory {
    pub name: String,
    pub path: String,
    pub is_git_repo: bool,
    pub file_count: usize,
    pub description: Option<String>,
    pub is_counting: bool,
}

fn is_project_directory(path: &Path) -> bool {
    let project_indicators = vec![
        "package.json", "Cargo.toml", "pom.xml", "build.gradle", "requirements.txt", "Gemfile", "go.mod",
        "composer.json", "project.clj", "mix.exs", ".csproj", "pubspec.yaml", "CMakeLists.txt", "Makefile",
        "README.md", "README.txt",
    ];

    for indicator in project_indicators {
        if indicator.ends_with(".csproj") {
            if let Ok(entries) = fs::read_dir(path) {
                for entry in entries.flatten() {
                    if let Some(name) = entry.file_name().to_str() {
                        if name.ends_with(".csproj") {
                            return true;
                        }
                    }
                }
            }
        } else if path.join(indicator).exists() {
            return true;
        }
    }
    false
}

fn get_project_description(path: &Path) -> Option<String> {
    if let Ok(package_json) = fs::read_to_string(path.join("package.json")) {
        if let Ok(json) = serde_json::from_str::<serde_json::Value>(&package_json) {
            if let Some(description) = json["description"].as_str() {
                return Some(description.to_string());
            }
        }
    }
    if let Ok(cargo_toml) = fs::read_to_string(path.join("Cargo.toml")) {
        if let Some(desc_line) = cargo_toml.lines().find(|line| line.starts_with("description")) {
            if let Some(desc) = desc_line.split('=').nth(1) {
                return Some(desc.trim().trim_matches('"').to_string());
            }
        }
    }
    for readme_name in &["README.md", "README.txt", "readme.md", "readme.txt"] {
        if let Ok(readme) = fs::read_to_string(path.join(readme_name)) {
            let first_line = readme.lines().next().unwrap_or("").trim();
            if !first_line.is_empty() && first_line.len() < 200 {
                let cleaned = first_line.trim_start_matches('#').trim();
                if !cleaned.is_empty() {
                    return Some(cleaned.to_string());
                }
            }
        }
    }
    None
}

fn estimate_deep_files(path: &Path) -> usize {
    let mut count = 0;
    let mut checked = 0;
    const MAX_CHECK: usize = 50;
    for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()).take(MAX_CHECK) {
        checked += 1;
        if entry.path().is_file() {
            let path_str = entry.path().to_string_lossy();
            if should_analyze_file(&path_str) {
                count += 1;
            }
        }
    }
    if checked >= MAX_CHECK { count * 2 } else { count }
}

fn estimate_file_count(path: &Path) -> usize {
    let mut count = 0;
    let mut depth = 0;
    const MAX_DEPTH: usize = 3;
    const SAMPLE_FACTOR: usize = 10;
    for entry in WalkDir::new(path).max_depth(MAX_DEPTH).into_iter().filter_map(|e| e.ok()) {
        if entry.path().is_file() {
            let path_str = entry.path().to_string_lossy();
            if should_analyze_file(&path_str) { count += 1; }
        }
        if entry.path().is_dir() && entry.depth() == MAX_DEPTH {
            depth += 1;
            if depth % SAMPLE_FACTOR == 0 {
                count += estimate_deep_files(entry.path()) * SAMPLE_FACTOR;
            }
        }
    }
    count
}

fn count_project_files(path: &Path) -> usize {
    WalkDir::new(path)
        .into_iter()
        .par_bridge()
        .filter_map(|e| e.ok())
        .filter(|entry| entry.path().is_file() && should_analyze_file(&entry.path().to_string_lossy()))
        .count()
}

fn process_project_directory_fast(
    path: std::path::PathBuf,
    count_cache: &HashMap<String, FileCountCache>,
    meta_cache: &HashMap<String, ProjectMetaCacheEntry>,
) -> Option<(ProjectDirectory, Option<ProjectMetaCacheEntry>)> {
    let dir_name = path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("Unknown")
        .to_string();

    if dir_name.starts_with('.')
        || ["node_modules", "target", "build", "dist", "vendor", "__pycache__"].contains(&dir_name.as_str())
    {
        return None;
    }

    if is_project_directory(&path) {
        let path_str = path.to_string_lossy().to_string();
        let dir_modified = get_dir_modified_time(&path);
        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap_or(Duration::from_secs(0))
            .as_secs();
        const META_TTL: u64 = 3600;

        let mut is_git_repo = path.join(".git").exists();
        let mut description: Option<String> = None;
        let mut meta_update: Option<ProjectMetaCacheEntry> = None;

        if let Some(meta) = meta_cache.get(&path_str) {
            if meta.last_modified >= dir_modified && (now - meta.cached_at) < META_TTL {
                is_git_repo = meta.is_git_repo;
                description = meta.description.clone();
            }
        }
        if description.is_none() {
            let desc = get_project_description(&path);
            description = desc.clone();
            meta_update = Some(ProjectMetaCacheEntry {
                path: path_str.clone(),
                description: desc,
                is_git_repo,
                last_modified: dir_modified,
                cached_at: now,
            });
        }

        let (file_count, is_counting) = if let Some(cached) = count_cache.get(&path_str) {
            if cached.last_modified >= dir_modified && (now - cached.cached_at) < 86400 {
                (cached.count, false)
            } else {
                (estimate_file_count(&path), true)
            }
        } else {
            (estimate_file_count(&path), true)
        };

        let project = ProjectDirectory {
            name: dir_name,
            path: path_str,
            is_git_repo,
            file_count,
            description,
            is_counting,
        };

        Some((project, meta_update))
    } else {
        None
    }
}

#[tauri::command]
pub async fn list_project_directories(root_path: String) -> Result<Vec<ProjectDirectory>, String> {
    let root = Path::new(&root_path);
    if !root.exists() || !root.is_dir() {
        return Err("Invalid root directory".to_string());
    }

    let count_cache = load_file_count_cache();
    let mut meta_cache = load_project_meta_cache();

    let entries: Vec<std::path::PathBuf> = fs::read_dir(root)
        .map_err(|e| format!("Failed to read directory: {}", e))?
        .filter_map(|e| e.ok())
        .filter(|entry| entry.path().is_dir())
        .map(|entry| entry.path())
        .collect();

    let results: Vec<(ProjectDirectory, Option<ProjectMetaCacheEntry>)> = entries
        .par_iter()
        .filter_map(|p| process_project_directory_fast(p.clone(), &count_cache, &meta_cache))
        .collect();

    let mut projects: Vec<ProjectDirectory> = Vec::with_capacity(results.len());
    for (proj, update) in results {
        if let Some(entry) = update { meta_cache.insert(proj.path.clone(), entry); }
        projects.push(proj);
    }
    save_project_meta_cache(&meta_cache);

    projects.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
    Ok(projects)
}

#[tauri::command]
pub async fn update_project_file_count(project_path: String) -> Result<usize, String> {
    let path = Path::new(&project_path);
    if !path.exists() || !path.is_dir() { return Err("Invalid project path".to_string()); }
    let count = count_project_files(path);
    let last_modified = get_dir_modified_time(path);
    let cached_at = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap_or(Duration::from_secs(0))
        .as_secs();
    let mut cache = load_file_count_cache();
    cache.insert(project_path.clone(), FileCountCache { path: project_path, count, last_modified, cached_at });
    save_file_count_cache(&cache);
    Ok(count)
}

#[tauri::command]
pub async fn clear_file_count_cache() -> Result<(), String> {
    clear_file_count_cache_file()
}
```

## File: src-tauri/src/storage.rs
```rust
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Settings {
    pub api_url: String,
    pub model: String,
    pub api_key: String,
}

fn app_dir() -> Result<std::path::PathBuf, String> {
    dirs::data_local_dir()
        .ok_or("Failed to get app data directory".to_string())
        .map(|d| d.join("repomuse"))
}

#[tauri::command]
pub async fn save_settings(settings: Settings) -> Result<(), String> {
    let dir = app_dir()?;
    if !dir.exists() { fs::create_dir_all(&dir).map_err(|e| e.to_string())?; }
    let path = dir.join("settings.json");
    let json = serde_json::to_string_pretty(&settings).map_err(|e| e.to_string())?;
    fs::write(path, json).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn load_settings() -> Result<Settings, String> {
    let dir = app_dir()?;
    let path = dir.join("settings.json");
    if !path.exists() {
        return Ok(Settings {
            api_url: "http://localhost:11434/v1/chat/completions".to_string(),
            model: "llama2".to_string(),
            api_key: "".to_string(),
        });
    }
    let s = fs::read_to_string(path).map_err(|e| e.to_string())?;
    serde_json::from_str(&s).map_err(|e| e.to_string())
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProjectSummary {
    pub project_path: String,
    pub summary: String,
    pub generated_at: String,
    pub technologies: Vec<String>,
    pub key_features: Vec<String>,
}

#[tauri::command]
pub async fn save_project_summary(summary: ProjectSummary) -> Result<(), String> {
    let dir = app_dir()?.join("summaries");
    if !dir.exists() { fs::create_dir_all(&dir).map_err(|e| e.to_string())?; }
    let filename = summary
        .project_path
        .replace("/", "_")
        .replace("\\", "_")
        .replace(":", "");
    let path = dir.join(format!("{}.json", filename));
    let json = serde_json::to_string_pretty(&summary).map_err(|e| e.to_string())?;
    fs::write(path, json).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn load_project_summary(project_path: String) -> Result<Option<ProjectSummary>, String> {
    let dir = app_dir()?.join("summaries");
    let filename = project_path.replace("/", "_").replace("\\", "_").replace(":", "");
    let path = dir.join(format!("{}.json", filename));
    if !path.exists() { return Ok(None); }
    let s = fs::read_to_string(path).map_err(|e| e.to_string())?;
    let summary: ProjectSummary = serde_json::from_str(&s).map_err(|e| e.to_string())?;
    Ok(Some(summary))
}

#[tauri::command]
pub async fn save_root_folder(root_folder: String) -> Result<(), String> {
    let dir = app_dir()?;
    if !dir.exists() { fs::create_dir_all(&dir).map_err(|e| e.to_string())?; }
    let path = dir.join("root_folder.txt");
    fs::write(path, root_folder).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn load_root_folder() -> Result<Option<String>, String> {
    let dir = app_dir()?;
    let path = dir.join("root_folder.txt");
    if !path.exists() { return Ok(None); }
    let s = fs::read_to_string(path).map_err(|e| e.to_string())?;
    if std::path::Path::new(&s).exists() { Ok(Some(s)) } else { Ok(None) }
}
```

## File: src/components/FolderSelector.tsx
```typescript
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
        <div className="bg-white rounded-lg shadow-md p-8">
          <div className="mb-6">
            <Folder className="mx-auto h-16 w-16 text-gray-400" />
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
```

## File: src/components/ProjectAnalyzer.tsx
```typescript
import React, { useState, useEffect } from 'react';
import { ProjectDirectory, RepoAnalysis, Settings, ProjectSummary, ProjectInsights } from '../types';
import { analyzeRepository, analyzeRepositoryFresh, generateIdeaList, generateProjectSummary, saveProjectSummary, loadProjectSummary, getProjectInsights } from '../utils/api';
import Spinner from './ui/Spinner';
import Alert from './ui/Alert';
import Card from './ui/Card';
import Button from './ui/Button';
import Badge from './ui/Badge';
import StatTile from './ui/StatTile';
import EmptyState from './ui/EmptyState';
import Tabs from './ui/Tabs';
import { FileText, Lightbulb, TrendingUp } from 'lucide-react';
import MarkdownRenderer from './MarkdownRenderer';
import ProjectInsightsComponent from './ProjectInsights';
import ProjectHeader from './ProjectHeader';
import { useToast } from './ui/ToastProvider';

interface ProjectAnalyzerProps {
  selectedProject: ProjectDirectory | null;
  settings: Settings;
}

const ProjectAnalyzer: React.FC<ProjectAnalyzerProps> = ({ selectedProject, settings }) => {
  const [analysis, setAnalysis] = useState<RepoAnalysis | null>(null);
  const [ideas, setIdeas] = useState<string[]>([]);
  const [summary, setSummary] = useState<ProjectSummary | null>(null);
  const [insights, setInsights] = useState<ProjectInsights | null>(null);
  const [isAnalyzing, setIsAnalyzing] = useState(false);
  const [isGeneratingIdeas, setIsGeneratingIdeas] = useState(false);
  const [isGeneratingSummary, setIsGeneratingSummary] = useState(false);
  const [isLoadingInsights, setIsLoadingInsights] = useState(false);
  const [analysisError, setAnalysisError] = useState<string>('');
  const [ideasError, setIdeasError] = useState<string>('');
  const [summaryError, setSummaryError] = useState<string>('');
  const [insightsError, setInsightsError] = useState<string>('');
  const { toast } = useToast();

  useEffect(() => {
    if (selectedProject) {
      setAnalysis(null);
      setIdeas([]);
      setSummary(null);
      setInsights(null);
      setAnalysisError('');
      setIdeasError('');
      setSummaryError('');
      setInsightsError('');
      analyzeProject();
      loadSummary();
      loadInsights();
    }
  }, [selectedProject]);

  const analyzeProject = async () => {
    if (!selectedProject) return;

    setIsAnalyzing(true);
    setAnalysisError('');

    try {
      const result = await analyzeRepository(selectedProject.path);
      setAnalysis(result);
    } catch (err) {
      setAnalysisError(err as string);
      toast({ title: 'Failed to analyze project', description: String(err), variant: 'error' });
    } finally {
      setIsAnalyzing(false);
    }
  };

  const refreshAnalysis = async () => {
    if (!selectedProject) return;
    setIsAnalyzing(true);
    setAnalysisError('');
    try {
      const result = await analyzeRepositoryFresh(selectedProject.path);
      setAnalysis(result);
      const total = result?.metrics?.total_files ?? 0;
      toast({ title: 'Analysis refreshed', description: `${total} files analyzed`, variant: 'success' });
    } catch (err) {
      setAnalysisError(err as string);
      toast({ title: 'Failed to refresh analysis', description: String(err), variant: 'error' });
    } finally {
      setIsAnalyzing(false);
    }
  };

  const generateIdeas = async () => {
    if (!analysis || !settings.api_url || !settings.model) {
      setIdeasError('Please configure API settings first');
      return;
    }

    setIsGeneratingIdeas(true);
    setIdeasError('');

    try {
      const generatedIdeas = await generateIdeaList({
        analysis,
        settings,
      });
      setIdeas(generatedIdeas);
      toast({ title: 'Ideas generated', description: `${generatedIdeas.length} ideas created`, variant: 'success' });
    } catch (err) {
      setIdeasError(err as string);
      toast({ title: 'Failed to generate ideas', description: String(err), variant: 'error' });
    } finally {
      setIsGeneratingIdeas(false);
    }
  };

  const loadSummary = async () => {
    if (!selectedProject) return;
    
    try {
      const loadedSummary = await loadProjectSummary(selectedProject.path);
      if (loadedSummary) {
        setSummary(loadedSummary);
      }
    } catch (err) {
      console.error('Error loading summary:', err);
    }
  };

  const loadInsights = async () => {
    if (!selectedProject) return;
    
    setIsLoadingInsights(true);
    setInsightsError('');
    
    try {
      const projectInsights = await getProjectInsights(selectedProject.path);
      setInsights(projectInsights);
    } catch (err) {
      setInsightsError(err as string);
    } finally {
      setIsLoadingInsights(false);
    }
  };

  const generateSummary = async () => {
    if (!analysis || !settings.api_url || !settings.model || !selectedProject) {
      setSummaryError('Please configure API settings first');
      return;
    }

    setIsGeneratingSummary(true);
    setSummaryError('');

    try {
      const generatedSummary = await generateProjectSummary({ analysis, settings, project_path: selectedProject.path });
      setSummary(generatedSummary);
      // Save the summary for future use
      await saveProjectSummary(generatedSummary);
      toast({ title: 'Summary generated', variant: 'success' });
    } catch (err) {
      setSummaryError(err as string);
      toast({ title: 'Failed to generate summary', description: String(err), variant: 'error' });
    } finally {
      setIsGeneratingSummary(false);
    }
  };

  if (!selectedProject) {
    return (
      <div className="flex items-center justify-center h-full">
        <EmptyState
          icon={<FileText className="h-16 w-16 text-gray-300" />}
          title="Select a Project"
          subtitle="Choose a project from the left sidebar to analyze and generate ideas"
        />
      </div>
    );
  }

  return (
    <div className="h-full flex flex-col">
      {/* Project Header */}
      <ProjectHeader
        name={selectedProject.name}
        path={selectedProject.path}
        description={selectedProject.description || null}
        isGitRepo={selectedProject.is_git_repo}
        fileCount={selectedProject.file_count}
      />

      {/* Content Area */}
      <div className="flex-1 overflow-y-auto">
        {isAnalyzing && (
          <div className="flex items-center justify-center py-12 text-gray-600">
            <Spinner color="blue" size="md" />
            <p className="ml-4">Analyzing project...</p>
          </div>
        )}

        {analysisError && (
          <div className="p-6">
            <Alert variant="error" title="Analysis Error">{analysisError}</Alert>
            <div className="mt-4">
              <Button variant="primary" onClick={analyzeProject}>Retry Analysis</Button>
            </div>
          </div>
        )}

        {analysis && (
          <div className="p-6">
            {/* Analysis Summary */}
            <Card className="p-6 mb-6">
              <div className="flex justify-between items-center mb-2">
                <h2 className="text-lg font-semibold text-gray-900">Project Analysis</h2>
                <Button variant="secondary" onClick={refreshAnalysis}>Refresh Analysis</Button>
              </div>
              {analysis.generated_at && (
                <p className="text-xs text-gray-500 mb-4">
                  Last analyzed: {new Date(analysis.generated_at).toLocaleString()}
                  {analysis.from_cache ? ' (cached)' : ''}
                </p>
              )}
              <div className="grid grid-cols-1 md:grid-cols-3 gap-4 mb-6">
                <StatTile label="Total Files" value={analysis.metrics.total_files} color="blue" />
                <StatTile label="Total Lines" value={analysis.metrics.total_lines?.toLocaleString()} color="green" />
                <StatTile label="Analyzed Files" value={analysis.metrics.analyzed_files} color="purple" />
              </div>
              <div className="mb-6">
                <h3 className="text-md font-semibold text-gray-900 mb-3">Technologies Detected</h3>
                <div className="flex flex-wrap gap-2">
                  {analysis.technologies.map((tech) => (
                    <span key={tech} className="bg-gray-100 text-gray-800 px-3 py-1 rounded-full text-sm">
                      {tech}
                    </span>
                  ))}
                </div>
              </div>
            </Card>

            {/* Tabbed Interface */}
            <Tabs
              tabs={[
                {
                  id: 'summary',
                  label: 'Project Summary',
                  content: (
                    <div>
                      <div className="flex justify-between items-center mb-6">
                        <div />
                        {!isGeneratingSummary && (
                          <Button onClick={generateSummary}>
                            {summary ? 'Regenerate Summary' : 'Generate Summary'}
                          </Button>
                        )}
                      </div>

                      {isGeneratingSummary && (
                        <div className="text-center py-8 text-gray-600">
                          <Spinner color="blue" />
                          <p className="mt-4">Generating AI summary...</p>
                          <p className="text-sm text-gray-500">This may take a moment...</p>
                        </div>
                      )}

                      {summaryError && (
                        <>
                          <Alert variant="error" title="Summary Error">{summaryError}</Alert>
                          <div className="mt-4">
                            <Button onClick={generateSummary}>Retry Generation</Button>
                          </div>
                        </>
                      )}

                      {!summary && !isGeneratingSummary && !summaryError && (
                        <EmptyState
                          icon={<Lightbulb className="h-12 w-12 text-gray-300" />}
                          title="No summary generated yet"
                          subtitle='Click "Generate Summary" to get an AI-powered overview of this project'
                        />
                      )}

                      {summary && (
                        <div className="space-y-4">
                          <div className="prose prose-sm max-w-none prose-headings:text-gray-900 prose-p:text-gray-700 prose-strong:text-gray-800 prose-ul:text-gray-700 prose-li:text-gray-700">
                            <MarkdownRenderer markdown={summary.summary} />
                          </div>
                          
                          {summary.key_features.length > 0 && (
                            <div className="mt-4">
                              <h3 className="text-sm font-semibold text-gray-900 mb-2">Key Features:</h3>
                              <ul className="space-y-1">
                                {summary.key_features.map((feature, index) => (
                                  <li key={index} className="flex items-start">
                                    <span className="text-blue-500 mr-2">•</span>
                                    <span className="text-gray-700 text-sm">{feature}</span>
                                  </li>
                                ))}
                              </ul>
                            </div>
                          )}
                          
                          <div className="mt-4 pt-4 border-t border-gray-200">
                            <p className="text-xs text-gray-500">
                              Generated: {new Date(summary.generated_at).toLocaleString()}
                            </p>
                          </div>
                        </div>
                      )}
                    </div>
                  ),
                  badge: summary ? <Badge variant="green" className="ml-2">✓</Badge> : undefined
                },
                {
                  id: 'ideas',
                  label: 'Development Ideas',
                  content: (
                    <div>
                      <div className="flex justify-between items-center mb-6">
                        <div />
                        {!isGeneratingIdeas && (
                          <Button onClick={generateIdeas}>
                            {ideas.length > 0 ? 'Regenerate Ideas' : 'Generate Ideas'}
                          </Button>
                        )}
                      </div>

                      {isGeneratingIdeas && (
                        <div className="text-center py-8 text-gray-600">
                          <Spinner color="green" />
                          <p className="mt-4">Generating creative ideas...</p>
                          <p className="text-sm text-gray-500">This may take a moment...</p>
                        </div>
                      )}

                      {ideasError && (
                        <div className="mb-6">
                          <Alert variant="error" title="Generation Error">{ideasError}</Alert>
                          <div className="mt-4">
                            <Button onClick={generateIdeas}>Retry Generation</Button>
                          </div>
                        </div>
                      )}

                      {ideas.length === 0 && !isGeneratingIdeas && !ideasError && (
                        <EmptyState
                          icon={<Lightbulb className="h-12 w-12 text-gray-300" />}
                          title="No ideas generated yet"
                          subtitle='Click "Generate Ideas" to get AI-powered development suggestions'
                        />
                      )}

                      {ideas.length > 0 && (
                        <div className="space-y-4">
                          {ideas.map((idea, index) => (
                            <div
                              key={index}
                              className="border-l-4 border-green-500 bg-green-50 p-4 rounded-r-md"
                            >
                              <div className="flex items-start">
                                <div className="flex-shrink-0">
                                  <span className="bg-green-500 text-white rounded-full w-6 h-6 flex items-center justify-center text-sm font-medium">
                                    {index + 1}
                                  </span>
                                </div>
                                <div className="ml-3">
                                  <p className="text-gray-800 whitespace-pre-line">{idea}</p>
                                </div>
                              </div>
                            </div>
                          ))}
                        </div>
                      )}
                    </div>
                  ),
                  badge: ideas.length > 0 ? <Badge variant="green" className="ml-2">{ideas.length}</Badge> : undefined
                },
                {
                  id: 'insights',
                  label: 'Insights',
                  content: (
                    <div>
                      {isLoadingInsights && (
                        <div className="text-center py-8 text-gray-600">
                          <Spinner color="blue" />
                          <p className="mt-4">Analyzing project health...</p>
                        </div>
                      )}

                      {insightsError && (
                        <>
                          <Alert variant="error" title="Insights Error">{insightsError}</Alert>
                          <div className="mt-4">
                            <Button onClick={loadInsights}>Retry Analysis</Button>
                          </div>
                        </>
                      )}

                      {insights && (
                        <ProjectInsightsComponent insights={insights} />
                      )}

                      {!insights && !isLoadingInsights && !insightsError && (
                        <EmptyState
                          icon={<TrendingUp className="h-12 w-12 text-gray-300" />}
                          title="No insights available"
                          subtitle="Unable to analyze project health"
                        />
                      )}
                    </div>
                  ),
                  badge: insights && (!insights.git_status.is_git_repo || insights.git_status.has_uncommitted_changes || !insights.readme_info.exists || insights.readme_info.is_default || !insights.ci_info.has_ci || !insights.testing_info.has_testing_framework || !insights.testing_info.has_test_files || insights.package_info.missing_common_files.length > 0) 
                    ? <Badge variant="red" className="ml-2">!</Badge> 
                    : insights ? <Badge variant="green" className="ml-2">✓</Badge> : undefined
                }
              ]}
              defaultTab="summary"
            />
          </div>
        )}
      </div>
    </div>
  );
};

export default ProjectAnalyzer;
```

## File: src/components/ProjectList.tsx
```typescript
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
```

## File: src/utils/storage.ts
```typescript
import { invoke } from '@tauri-apps/api/core';
import { Settings } from '../types';

export const saveSettings = async (settings: Settings): Promise<void> => {
  await invoke('save_settings', { settings });
};

export const loadSettings = async (): Promise<Settings> => {
  return await invoke('load_settings');
};
```

## File: src/index.css
```css
@tailwind base;
@tailwind components;
@tailwind utilities;

* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}

body {
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', 'Roboto', 'Oxygen',
    'Ubuntu', 'Cantarell', 'Fira Sans', 'Droid Sans', 'Helvetica Neue',
    sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
}

code {
  font-family: source-code-pro, Menlo, Monaco, Consolas, 'Courier New',
    monospace;
}
```

## File: src/main.tsx
```typescript
import React from 'react';
import ReactDOM from 'react-dom/client';
import App from './App';

ReactDOM.createRoot(document.getElementById('root')!).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>
);
```

## File: src/vite-env.d.ts
```typescript
/// <reference types="vite/client" />
```

## File: src-tauri/src/lib.rs
```rust
// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

## File: src-tauri/.gitignore
```
# Generated by Cargo
# will have compiled files and executables
/target/

# Generated by Tauri
# will have schema files for capabilities auto-completion
/gen/schemas
```

## File: src-tauri/build.rs
```rust
fn main() {
    tauri_build::build()
}
```

## File: src-tauri/tauri.conf.json
```json
{
  "$schema": "https://schema.tauri.app/config/2",
  "productName": "repomuse",
  "version": "0.1.0",
  "identifier": "com.mike.repomuse",
  "build": {
    "beforeDevCommand": "npm run dev",
    "devUrl": "http://localhost:1420",
    "beforeBuildCommand": "npm run build",
    "frontendDist": "../dist"
  },
  "app": {
    "windows": [
      {
        "title": "repomuse",
        "width": 800,
        "height": 600
      }
    ],
    "security": {
      "csp": null
    }
  },
  "bundle": {
    "active": true,
    "targets": "all",
    "icon": [
      "icons/32x32.png",
      "icons/128x128.png",
      "icons/128x128@2x.png",
      "icons/icon.icns",
      "icons/icon.ico"
    ]
  }
}
```

## File: .gitignore
```
# Logs
logs
*.log
npm-debug.log*
yarn-debug.log*
yarn-error.log*
pnpm-debug.log*
lerna-debug.log*

node_modules
dist
dist-ssr
*.local

# Editor directories and files
.vscode/*
!.vscode/extensions.json
.idea
.DS_Store
*.suo
*.ntvs*
*.njsproj
*.sln
*.sw?
```

## File: AGENTS.md
```markdown
# Repository Guidelines

## Project Structure & Module Organization
- `src/`: React + TypeScript UI (components, utils, types, assets). Example: `src/components/IdeaGenerator.tsx`.
- `src-tauri/`: Tauri (Rust) backend. Entry: `src-tauri/src/main.rs`; config: `src-tauri/tauri.conf.json`.
- `public/` and `index.html`: Static assets and Vite entry.
- Tooling: Vite (`vite.config.ts`), Tailwind (`tailwind.config.js`, `postcss.config.js`), TS config (`tsconfig.json`).

## Build, Test, and Development Commands
- Install: `npm install`.
- Desktop dev (recommended): `npm run tauri dev` — launches the Tauri app with hot reload.
- Web-only dev: `npm run dev` — Vite dev server (some Tauri APIs may be unavailable in browser).
- Build frontend: `npm run build` — type-checks (`tsc`) and builds Vite assets.
- Preview build: `npm run preview` — serves `dist/` locally.
- Desktop build: `npm run tauri build` — builds the Tauri app (Rust + frontend bundle).

## Coding Style & Naming Conventions
- TypeScript: strict mode enabled; prefer functional React components and hooks.
- Indentation: 2 spaces; camelCase for variables/functions; PascalCase for React components (e.g., `FolderSelector.tsx`).
- File layout: colocate tests and styles with components when added.
- Tailwind: prefer utility classes; keep component markup readable.
- Rust: idiomatic `rustfmt` style; one command module per file where practical.

## Testing Guidelines
- JS/TS: not set up yet. Recommend Vitest + React Testing Library. Name tests `*.test.ts` / `*.test.tsx` next to source.
- Rust: add unit tests with `#[cfg(test)]` in modules; run with `cargo test` inside `src-tauri/`.
- Target coverage: ~80% lines/branches for new/changed code.

## Commit & Pull Request Guidelines
- Commits: use Conventional Commits where possible (e.g., `feat: add idea generator panel`). Keep changes atomic.
- PRs: include a clear description, linked issues, and screenshots/GIFs for UI changes.
- Verification: run `npm run build` and, for desktop, `npm run tauri build` before requesting review.

## Security & Configuration Tips
- Do not hardcode secrets. API URL/model/key are managed via the Settings UI and persisted by Tauri (see `save_settings`/`load_settings`).
- Keep large/binary files out of analysis; repository walk already ignores common directories (`node_modules`, `target`, `dist`).
```

## File: index.html
```html
<!doctype html>
<html lang="en">
  <head>
    <meta charset="UTF-8" />
    <link rel="icon" type="image/svg+xml" href="/vite.svg" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <title>Tauri + React + Typescript</title>
  </head>

  <body>
    <div id="root"></div>
    <script type="module" src="/src/main.tsx"></script>
  </body>
</html>
```

## File: package.json
```json
{
  "name": "repomuse",
  "private": true,
  "version": "0.1.0",
  "type": "module",
  "scripts": {
    "dev": "vite",
    "build": "tsc && vite build",
    "preview": "vite preview",
    "tauri": "tauri"
  },
  "dependencies": {
    "react": "^19.1.0",
    "react-dom": "^19.1.0",
    "@tauri-apps/api": "^2",
    "@tauri-apps/plugin-dialog": "^2.0.0",
    "@tauri-apps/plugin-opener": "^2",
    "lucide-react": "^0.471.0"
  },
  "devDependencies": {
    "@types/react": "^19.1.8",
    "@types/react-dom": "^19.1.6",
    "@vitejs/plugin-react": "^4.6.0",
    "typescript": "~5.8.3",
    "tailwindcss": "^3.3.0",
    "autoprefixer": "^10.4.14",
    "postcss": "^8.4.24",
    "vite": "^7.0.4",
    "@tauri-apps/cli": "^2"
  }
}
```

## File: postcss.config.js
```javascript
export default {
  plugins: {
    tailwindcss: {},
    autoprefixer: {},
  },
}
```

## File: README.md
```markdown
# RepoMuse

AI‑assisted repository explorer and idea generator. RepoMuse is a cross‑platform desktop app built with Tauri (Rust) and React + TypeScript that helps you:

- Point at a root folder of code projects
- Discover projects automatically (Node, Rust, Python, Go, etc.)
- Analyze a selected project (files, lines, languages, lightweight structure)
- Generate a clear project summary and actionable development ideas using an OpenAI‑compatible API (OpenAI, Azure OpenAI, DeepSeek, Ollama)
- Persist settings and summaries locally for quick revisits

## What It Does

- Project discovery: Scans the chosen root folder for likely projects using common indicators (e.g., `package.json`, `Cargo.toml`, `requirements.txt`, `.csproj`, `Makefile`). Shows name, Git presence and a fast file count that refines in the background.
- Repository analysis: Reads source files (skipping big/binary/ignored folders), detects languages by extension, and computes metrics like total files and lines plus a lightweight directory structure.
- AI summaries and ideas: Calls your configured OpenAI‑compatible endpoint to produce a human‑readable project summary and 5–10 practical, numbered development ideas. Thinking‑style models (e.g., `o1-mini`, `deepseek-r1`) are supported and hidden reasoning tags are filtered.
- Local persistence: Settings, selected root folder, file‑count cache and generated summaries are stored locally via Tauri for a snappy, privacy‑friendly experience.

## Quick Start

Prerequisites
- Node.js 18+ and npm
- Rust toolchain (stable) and platform build tools required by Tauri
  - macOS: Xcode Command Line Tools
  - Windows: Visual Studio Build Tools (C++), WebView2
  - Linux: common GTK/WebKit and build packages

Install
- `npm install`

Run (desktop, recommended)
- `npm run tauri dev` — launches the Tauri app with hot reload.

Run (web‑only)
- `npm run dev` — Vite dev server in the browser. Some Tauri‑only features (e.g., native dialogs, local persistence paths) may be limited.

Build
- Frontend: `npm run build`
- Desktop app: `npm run tauri build`

## Using RepoMuse

1) Choose a root folder
- On first launch, click “Choose Folder” and select a directory that contains many project subfolders.

2) Browse projects
- The left sidebar lists detected projects. It shows a quick file count (refined asynchronously) and a Git badge when applicable.

3) Analyze a project
- Select a project to view metrics (files, lines), detected technologies, and a lightweight directory view.

4) Configure AI
- Open Settings to set `API Server URL`, optional `API Key`, and `Model`.
- Click “Load Models” to fetch available models from compatible APIs.
- Examples:
  - Ollama (local): URL `http://localhost:11434/v1/chat/completions`, empty key, models like `llama2`, `codellama`, `deepseek-r1`.
  - OpenAI: URL `https://api.openai.com/v1/chat/completions`, API key required, models like `gpt-4`, `o1-mini`.
  - DeepSeek: URL `https://api.deepseek.com/v1/chat/completions`, API key required, models like `deepseek-chat`, `deepseek-r1`.

5) Generate output
- In the Project Summary card, click “Generate Summary” to create a concise explanation of what the project is and does.
- In the Development Ideas card, click “Generate Ideas” to get actionable suggestions. Regenerate anytime.
- Summaries are saved locally and loaded automatically next time.

## Tech Stack

- Frontend: React 19, TypeScript, Vite, Tailwind CSS
- Desktop: Tauri 2 (Rust)
- Native plugins: `@tauri-apps/plugin-dialog`, `@tauri-apps/plugin-opener`
- Icons: `lucide-react`
- AI backend: Any OpenAI‑compatible Chat Completions API

## Scripts

- `npm run dev`: Start Vite dev server (web‑only)
- `npm run build`: Type‑check and build frontend
- `npm run preview`: Serve built assets from `dist/`
- `npm run tauri dev`: Run desktop app with hot reload
- `npm run tauri build`: Build production desktop binaries

## Project Structure

- `src/`: React + TypeScript UI (components, utils, types, assets)
  - Example: `src/components/ProjectAnalyzer.tsx`, `src/components/Settings.tsx`
- `src-tauri/`: Tauri (Rust) backend
  - Entry: `src-tauri/src/main.rs`
  - Config: `src-tauri/tauri.conf.json`
- `public/` and `index.html`: Static assets and Vite entry
- Tooling: `vite.config.ts`, `tailwind.config.js`, `postcss.config.js`, `tsconfig*.json`

## Privacy & Security

- No secrets are hardcoded. Enter your API URL, model and key in Settings; they are stored locally by Tauri.
- Repo content is analyzed locally; only the prompts you request (for summaries/ideas) are sent to your configured API endpoint.
- Large/binary folders like `node_modules`, `target`, and `dist` are skipped during analysis.

## Testing

- JS/TS: Not set up yet (recommended: Vitest + React Testing Library). Name tests `*.test.ts(x)` next to sources.
- Rust: Add unit tests with `#[cfg(test)]` inside modules and run with `cargo test` in `src-tauri/`.

## Troubleshooting

- Models not loading: Verify the API URL points to a models‑listing endpoint compatible with OpenAI or Ollama variants, and that your API key (if required) is valid.
- Ideas/Summary not generating: Ensure Settings are saved and the model supports chat completions. Check your network and API logs.
- Desktop build issues: Confirm Tauri prerequisites for your OS are installed and Rust toolchain is up to date.

---

Made with Tauri + React to make sense of codebases and spark next steps.
```

## File: tailwind.config.js
```javascript
module.exports = {
  content: ["./src/**/*.{js,ts,jsx,tsx}"],
  theme: {
    extend: {},
  },
  plugins: [],
}
```

## File: tsconfig.json
```json
{
  "compilerOptions": {
    "target": "ES2020",
    "useDefineForClassFields": true,
    "lib": ["ES2020", "DOM", "DOM.Iterable"],
    "module": "ESNext",
    "skipLibCheck": true,

    /* Bundler mode */
    "moduleResolution": "bundler",
    "allowImportingTsExtensions": true,
    "resolveJsonModule": true,
    "isolatedModules": true,
    "noEmit": true,
    "jsx": "react-jsx",

    /* Linting */
    "strict": true,
    "noUnusedLocals": true,
    "noUnusedParameters": true,
    "noFallthroughCasesInSwitch": true
  },
  "include": ["src"],
  "references": [{ "path": "./tsconfig.node.json" }]
}
```

## File: tsconfig.node.json
```json
{
  "compilerOptions": {
    "composite": true,
    "skipLibCheck": true,
    "module": "ESNext",
    "moduleResolution": "bundler",
    "allowSyntheticDefaultImports": true
  },
  "include": ["vite.config.ts"]
}
```

## File: vite.config.ts
```typescript
import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";

// @ts-expect-error process is a nodejs global
const host = process.env.TAURI_DEV_HOST;

// https://vite.dev/config/
export default defineConfig(async () => ({
  plugins: [react()],

  // Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
  //
  // 1. prevent Vite from obscuring rust errors
  clearScreen: false,
  // 2. tauri expects a fixed port, fail if that port is not available
  server: {
    port: 1420,
    strictPort: true,
    host: host || false,
    hmr: host
      ? {
          protocol: "ws",
          host,
          port: 1421,
        }
      : undefined,
    watch: {
      // 3. tell Vite to ignore watching `src-tauri`
      ignored: ["**/src-tauri/**"],
    },
  },
}));
```

## File: src/components/Settings.tsx
```typescript
import React, { useState } from 'react';
import { Settings as SettingsType, ModelInfo } from '../types';
import { saveSettings } from '../utils/storage';
import { loadModels } from '../utils/api';
import Button from './ui/Button';
import Badge from './ui/Badge';
import TextField from './ui/TextField';
import Select from './ui/Select';
import Fieldset from './ui/Fieldset';
import FormRow from './ui/FormRow';
import { isThinkingModel } from '../utils/models';
import { useToast } from './ui/ToastProvider';

interface SettingsProps {
  settings: SettingsType;
  onSettingsUpdated: (settings: SettingsType) => void;
}

const Settings: React.FC<SettingsProps> = ({ settings, onSettingsUpdated }) => {
  const [formData, setFormData] = useState(settings);
  const [isSaving, setIsSaving] = useState(false);
  const [isLoadingModels, setIsLoadingModels] = useState(false);
  const [models, setModels] = useState<ModelInfo[]>([]);
  const [modelsLoaded, setModelsLoaded] = useState(false);
  const { toast } = useToast();

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setIsSaving(true);
    try {
      await saveSettings(formData);
      onSettingsUpdated(formData);
      toast({ title: 'Settings saved', variant: 'success' });
    } catch (error) {
      console.error('Error saving settings:', error);
      toast({ title: 'Failed to save settings', description: String(error), variant: 'error' });
    } finally {
      setIsSaving(false);
    }
  };

  const handleChange = (e: React.ChangeEvent<HTMLInputElement | HTMLSelectElement>) => {
    setFormData({
      ...formData,
      [e.target.name]: e.target.value,
    });
  };

  const handleLoadModels = async () => {
    if (!formData.api_url) {
      alert('Please enter an API URL first');
      return;
    }

    setIsLoadingModels(true);
    try {
      const availableModels = await loadModels(formData.api_url, formData.api_key);
      setModels(availableModels);
      setModelsLoaded(true);
      toast({ title: `Loaded ${availableModels.length} models`, variant: 'success' });
    } catch (error) {
      console.error('Error loading models:', error);
      toast({ title: 'Failed to load models', description: 'Check API URL and key.', variant: 'error' });
    } finally {
      setIsLoadingModels(false);
    }
  };

  // moved to utils/models

  return (
    <div className="max-w-2xl mx-auto">
      <div className="bg-white rounded-lg shadow-md p-8">
        <Fieldset title="API Settings">
          <form onSubmit={handleSubmit} className="space-y-6">
            <FormRow>
              <TextField
                label="API Server URL"
                type="url"
                id="api_url"
                name="api_url"
                value={formData.api_url}
                onChange={handleChange}
                placeholder="http://localhost:11434/v1/chat/completions"
                required
                helpText="OpenAI-compatible API endpoint (e.g., Ollama, OpenAI, Azure OpenAI)"
              />
            </FormRow>

            <FormRow>
              <TextField
                label="API Key"
                type="password"
                id="api_key"
                name="api_key"
                value={formData.api_key}
                onChange={handleChange}
                placeholder="Your API key (leave empty for local APIs like Ollama)"
                helpText="Leave empty if your API doesn't require authentication"
              />
            </FormRow>

            <FormRow>
              <div className="flex items-center justify-between mb-2">
                <label htmlFor="model" className="block text-sm font-medium text-gray-700">
                  Model
                </label>
                <Button
                  type="button"
                  variant="secondary"
                  size="sm"
                  onClick={handleLoadModels}
                  disabled={isLoadingModels || !formData.api_url}
                  loading={isLoadingModels}
                >
                  Load Models
                </Button>
              </div>

              {modelsLoaded && models.length > 0 ? (
                <Select
                  label="Model"
                  id="model"
                  name="model"
                  value={formData.model}
                  onChange={handleChange}
                  required
                >
                  <option value="">Select a model...</option>
                  {models.map((model) => (
                    <option key={model.id} value={model.id}>
                      {model.name || model.id}
                      {isThinkingModel(model.id) && ' (Thinking Model)'}
                      {model.description && ` - ${model.description}`}
                    </option>
                  ))}
                </Select>
              ) : (
                <TextField
                  label="Model"
                  type="text"
                  id="model"
                  name="model"
                  value={formData.model}
                  onChange={handleChange}
                  placeholder="llama2, gpt-3.5-turbo, gpt-4, o1-mini, etc."
                  required
                />
              )}
              <p className="mt-1 text-sm text-gray-500">
                {isThinkingModel(formData.model)
                  ? 'This is a thinking model - it will use <think></think> tags for reasoning'
                  : "The model to use for generating ideas. Click 'Load Models' to see available options"}
              </p>
            </FormRow>

            <div className="flex items-center justify-between">
              <Button type="submit" variant="primary" loading={isSaving}>Save Settings</Button>
            </div>
          </form>
        </Fieldset>

        {models.length > 0 && (
          <div className="mt-8 p-4 bg-blue-50 rounded-md">
            <h3 className="text-lg font-medium text-blue-900 mb-2">Available Models ({models.length})</h3>
            <div className="grid gap-2 text-sm max-h-48 overflow-y-auto">
              {models.map((model) => (
                <div key={model.id} className="flex justify-between items-center p-2 bg-white rounded border">
                  <div>
                    <span className="font-medium">{model.name || model.id}</span>
                    {isThinkingModel(model.id) && (
                      <Badge variant="purple">Thinking Model</Badge>
                    )}
                  </div>
                  {model.description && (
                    <span className="text-gray-500 text-xs">{model.description}</span>
                  )}
                </div>
              ))}
            </div>
          </div>
        )}

        <div className="mt-8 p-4 bg-gray-50 rounded-md">
          <h3 className="text-lg font-medium text-gray-900 mb-2">Popular API Configurations</h3>
          <div className="space-y-3 text-sm">
            <div>
              <strong>Ollama (Local):</strong><br />
              URL: http://localhost:11434/v1/chat/completions<br />
              Models: llama2, codellama, deepseek-r1, etc.<br />
              API Key: (leave empty)
            </div>
            <div>
              <strong>OpenAI:</strong><br />
              URL: https://api.openai.com/v1/chat/completions<br />
              Models: gpt-3.5-turbo, gpt-4, o1-mini, o1-preview<br />
              API Key: Your OpenAI API key
            </div>
            <div>
              <strong>DeepSeek:</strong><br />
              URL: https://api.deepseek.com/v1/chat/completions<br />
              Models: deepseek-chat, deepseek-r1 (thinking)<br />
              API Key: Your DeepSeek API key
            </div>
          </div>
        </div>

        <div className="mt-6 p-4 bg-purple-50 rounded-md">
          <h3 className="text-lg font-medium text-purple-900 mb-2">🧠 Thinking Models</h3>
          <p className="text-sm text-purple-800">
            Thinking models like OpenAI's o1, o1-mini, or DeepSeek's R1 use special reasoning patterns. 
            They enclose their reasoning in <span className="font-mono bg-purple-100 px-1 rounded">&lt;think&gt;...&lt;/think&gt;</span> tags, 
            which this app automatically filters out to show only the final response.
          </p>
        </div>
      </div>
    </div>
  );
};

export default Settings;
```

## File: src/types/index.ts
```typescript
export interface Settings {
  api_url: string;
  model: string;
  api_key: string;
}

export interface ModelInfo {
  id: string;
  name?: string;
  description?: string;
}

export interface FileInfo {
  path: string;
  content: string;
  language: string;
  size: number;
}

export interface RepoAnalysis {
  files: FileInfo[];
  structure: Record<string, string[]>;
  technologies: string[];
  metrics: Record<string, number>;
  generated_at?: string;
  from_cache?: boolean;
}

export interface IdeaRequest {
  analysis: RepoAnalysis;
  settings: Settings;
}

export interface ProjectDirectory {
  name: string;
  path: string;
  is_git_repo: boolean;
  file_count: number;
  description?: string;
  is_counting: boolean;
}

export interface ProjectSummary {
  project_path: string;
  summary: string;
  generated_at: string;
  technologies: string[];
  key_features: string[];
}

export interface SummaryRequest {
  analysis: RepoAnalysis;
  settings: Settings;
  project_path?: string;
}

export interface GitStatus {
  is_git_repo: boolean;
  has_uncommitted_changes: boolean;
  uncommitted_files: string[];
  current_branch?: string;
  last_commit_date?: string;
  commit_count?: number;
}

export interface ReadmeInfo {
  exists: boolean;
  is_default: boolean;
  path?: string;
  content_preview?: string;
}

export interface CIInfo {
  has_ci: boolean;
  ci_platforms: string[];
  ci_files: string[];
}

export interface PackageInfo {
  has_package_json: boolean;
  has_cargo_toml: boolean;
  has_requirements_txt: boolean;
  has_gemfile: boolean;
  has_go_mod: boolean;
  missing_common_files: string[];
}

export interface TestingInfo {
  has_testing_framework: boolean;
  testing_frameworks: string[];
  has_test_files: boolean;
  test_file_count: number;
  test_file_patterns: string[];
  source_to_test_ratio?: number;
}

export interface ProjectInsights {
  git_status: GitStatus;
  readme_info: ReadmeInfo;
  ci_info: CIInfo;
  package_info: PackageInfo;
  testing_info: TestingInfo;
}
```

## File: src/utils/api.ts
```typescript
import { invoke } from '@tauri-apps/api/core';
import { RepoAnalysis, IdeaRequest, ModelInfo, ProjectDirectory, ProjectSummary, SummaryRequest, ProjectInsights } from '../types';

export async function listProjectDirectories(rootPath: string): Promise<ProjectDirectory[]> {
  return await invoke('list_project_directories', { rootPath });
}

export async function analyzeRepository(folderPath: string): Promise<RepoAnalysis> {
  return await invoke('analyze_repository', { folderPath });
}

export async function analyzeRepositoryFresh(folderPath: string): Promise<RepoAnalysis> {
  return await invoke('analyze_repository_fresh', { folderPath });
}

export async function generateIdeaList(request: IdeaRequest): Promise<string[]> {
  return await invoke('generate_ideas', { request });
}

export async function loadModels(apiUrl: string, apiKey: string): Promise<ModelInfo[]> {
  return await invoke('load_models', { apiUrl, apiKey });
}

export async function generateProjectSummary(request: SummaryRequest): Promise<ProjectSummary> {
  return await invoke('generate_project_summary', { request });
}

export async function saveProjectSummary(summary: ProjectSummary): Promise<void> {
  return await invoke('save_project_summary', { summary });
}

export async function loadProjectSummary(projectPath: string): Promise<ProjectSummary | null> {
  return await invoke('load_project_summary', { projectPath });
}

export async function saveRootFolder(rootFolder: string): Promise<void> {
  return await invoke('save_root_folder', { rootFolder });
}

export async function loadRootFolder(): Promise<string | null> {
  return await invoke('load_root_folder');
}

export async function getProjectInsights(projectPath: string): Promise<ProjectInsights> {
  return await invoke('get_project_insights', { projectPath });
}
```

## File: src/App.tsx
```typescript
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
```

## File: src-tauri/capabilities/default.json
```json
{
  "$schema": "../gen/schemas/desktop-schema.json",
  "identifier": "default",
  "description": "Capability for the main window",
  "windows": ["main"],
  "permissions": [
    "core:default",
    "dialog:default",
    "opener:default"
  ]
}
```

## File: src-tauri/src/main.rs
```rust
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod fs_utils;
mod cache;
mod analysis;
mod projects;
mod storage;
mod ai;
mod insights;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            projects::list_project_directories,
            analysis::analyze_repository,
            analysis::analyze_repository_fresh,
            ai::generate_ideas,
            storage::save_settings,
            storage::load_settings,
            ai::load_models,
            ai::generate_project_summary,
            storage::save_project_summary,
            storage::load_project_summary,
            storage::save_root_folder,
            storage::load_root_folder,
            projects::update_project_file_count,
            projects::clear_file_count_cache,
            insights::get_project_insights
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

## File: src-tauri/Cargo.toml
```toml
[package]
name = "repomuse"
version = "0.1.0"
description = "A Tauri App"
authors = ["you"]
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[lib]
# The `_lib` suffix may seem redundant but it is necessary
# to make the lib name unique and wouldn't conflict with the bin name.
# This seems to be only an issue on Windows, see https://github.com/rust-lang/cargo/issues/8519
name = "repomuse_lib"
crate-type = ["staticlib", "cdylib", "rlib"]

[build-dependencies]
tauri-build = { version = "2", features = [] }

[dependencies]
tauri = { version = "2.0", features = [] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
tokio = { version = "1", features = ["full", "time"] }
walkdir = "2"
ignore = "0.4"
reqwest = { version = "0.11", features = ["json"] }
tauri-plugin-dialog = "2"
tauri-plugin-opener = "2"
regex = "1"
dirs = "5"
chrono = "0.4"
rayon = "1.10"

[features]
custom-protocol = ["tauri/custom-protocol"]
```
