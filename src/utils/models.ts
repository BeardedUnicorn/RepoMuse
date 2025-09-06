export const isThinkingModel = (modelId: string): boolean => {
  if (!modelId) return false;
  const id = modelId.toLowerCase();
  const thinking = ['o1', 'o1-mini', 'o1-preview', 'deepseek-r1'];
  return thinking.some(name => id.includes(name));
};

