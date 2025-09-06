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
    <div className={`text-center py-8 text-foreground-secondary ${className}`}>
      {icon && <div className="mb-3 flex justify-center text-foreground-tertiary">{icon}</div>}
      <p className="font-medium text-foreground">{title}</p>
      {subtitle && <p className="text-sm text-foreground-tertiary mt-1">{subtitle}</p>}
      {action && <div className="mt-4">{action}</div>}
    </div>
  );
};

export default EmptyState;