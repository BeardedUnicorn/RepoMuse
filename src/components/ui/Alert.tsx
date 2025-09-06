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
    container: 'bg-error-background border border-error/20',
    title: 'text-error',
    text: 'text-error/80',
  },
  info: {
    container: 'bg-info-background border border-info/20',
    title: 'text-info',
    text: 'text-info/80',
  },
  success: {
    container: 'bg-success-background border border-success/20',
    title: 'text-success',
    text: 'text-success/80',
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