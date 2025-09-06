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
