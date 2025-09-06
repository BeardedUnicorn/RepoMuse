import React from 'react';

type Variant = 'primary' | 'secondary' | 'ghost';
type Size = 'sm' | 'md';

type ButtonProps = React.ButtonHTMLAttributes<HTMLButtonElement> & {
  variant?: Variant;
  size?: Size;
  loading?: boolean;
};

const variantClasses: Record<Variant, string> = {
  primary: 'bg-primary text-primary-foreground hover:bg-primary-hover disabled:opacity-50 disabled:cursor-not-allowed',
  secondary: 'bg-background-tertiary hover:bg-border/20 text-foreground disabled:opacity-50 disabled:cursor-not-allowed',
  ghost: 'text-foreground-secondary hover:text-foreground hover:bg-background-tertiary disabled:opacity-50 disabled:cursor-not-allowed',
};

const sizeClasses: Record<Size, string> = {
  sm: 'px-3 py-1 text-sm',
  md: 'px-4 py-2 text-sm',
};

const Button: React.FC<ButtonProps> = ({
  variant = 'primary',
  size = 'md',
  loading = false,
  className = '',
  children,
  disabled,
  ...rest
}) => {
  return (
    <button
      className={`${variantClasses[variant]} ${sizeClasses[size]} rounded-md font-medium transition-colors ${className}`}
      disabled={disabled || loading}
      {...rest}
    >
      {loading ? 'Loading...' : children}
    </button>
  );
};

export default Button;