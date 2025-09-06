import React from 'react';

// Very lightweight markdown renderer; safe for short, trusted content.
function toHtml(text: string): string {
  let html = text;

  // Headers
  html = html.replace(/^### (.*$)/gim, '<h3 class="text-foreground">$1</h3>');
  html = html.replace(/^## (.*$)/gim, '<h2 class="text-foreground">$1</h2>');
  html = html.replace(/^# (.*$)/gim, '<h1 class="text-foreground">$1</h1>');

  // Bold / Italic
  html = html.replace(/\*\*(.*?)\*\*/g, '<strong class="text-foreground">$1</strong>');
  html = html.replace(/__(.*?)__/g, '<strong class="text-foreground">$1</strong>');
  html = html.replace(/\*(.*?)\*/g, '<em>$1</em>');
  html = html.replace(/_(.*?)_/g, '<em>$1</em>');

  // Inline code
  html = html.replace(/`(.*?)`/g, '<code class="bg-background-tertiary px-1 py-0.5 rounded text-sm font-mono text-foreground">$1</code>');

  // Lists
  html = html.replace(/^\s*\* (.*$)/gim, '<li>$1</li>');
  html = html.replace(/^\s*- (.*$)/gim, '<li>$1</li>');
  html = html.replace(/^\s*\+ (.*$)/gim, '<li>$1</li>');
  html = html.replace(/^\s*\d+\.\s+(.*$)/gim, '<li>$1</li>');

  // Wrap consecutive list items
  html = html.replace(/(<li>.*<\/li>)/gs, (match) => {
    return `<ul class="list-disc list-inside space-y-1 my-2 text-foreground-secondary">${match}</ul>`;
  });
  html = html.replace(/<\/ul>\s*<ul[^>]*>/g, '');

  // Paragraphs
  html = html.replace(/\n\n/g, '</p><p class="text-foreground-secondary">');
  html = `<p class="text-foreground-secondary">${html}</p>`;
  html = html.replace(/<p[^>]*><\/p>/g, '');
  html = html.replace(/<p[^>]*>\s*<\/p>/g, '');
  html = html.replace(/<p[^>]*>(<h[1-6][^>]*>.*?<\/h[1-6]>)<\/p>/g, '$1');
  html = html.replace(/<p[^>]*>(<ul.*?<\/ul>)<\/p>/gs, '$1');

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