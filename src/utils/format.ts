export const basename = (p: string): string => {
  if (!p) return '';
  const parts = p.split(/[/\\]/);
  return parts[parts.length - 1] || p;
};

export const numberWithCommas = (n?: number | null): string => {
  if (n === undefined || n === null) return '0';
  return n.toLocaleString();
};

