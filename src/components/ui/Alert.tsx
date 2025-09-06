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