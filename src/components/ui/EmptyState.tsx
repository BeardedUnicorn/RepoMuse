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

