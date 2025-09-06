import React from 'react';

const FormHelp: React.FC<{ children: React.ReactNode; className?: string }> = ({ children, className = '' }) => (
  <p className={`mt-1 text-sm text-foreground-secondary ${className}`}>{children}</p>
);

export default FormHelp;