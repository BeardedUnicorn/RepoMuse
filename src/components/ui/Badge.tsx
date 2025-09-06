import React from 'react';

type Variant = 'green' | 'purple' | 'gray' | 'blue' | 'red';

type BadgeProps = {
  variant?: Variant;
  children: React.ReactNode;
  className?: string;
};

const variants: Record<Variant, string> = {
  green: 'bg-success/10 text-success dark:bg-success/20',
  purple: 'bg-purple-100 text-purple-800 dark:bg-purple-900/30 dark:text-purple-400',
  gray: 'bg-background-tertiary text-foreground-secondary',
  blue: 'bg-info/10 text-info dark:bg-info/20',
  red: 'bg-error/10 text-error dark:bg-error/20',
};

const Badge: React.FC<BadgeProps> = ({ variant = 'gray', children, className = '' }) => (
  <span className={`inline-flex items-center px-2 py-0.5 rounded text-xs font-medium ${variants[variant]} ${className}`}>
    {children}
  </span>
);

export default Badge;