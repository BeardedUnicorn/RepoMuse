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
  blue: 'border-primary',
  green: 'border-success',
  gray: 'border-foreground-secondary',
};

const Spinner: React.FC<SpinnerProps> = ({ size = 'md', color = 'gray', className = '' }) => {
  return (
    <div
      className={`inline-block animate-spin rounded-full ${sizeMap[size]} ${colorMap[color]} ${className}`}
    />
  );
};

export default Spinner;