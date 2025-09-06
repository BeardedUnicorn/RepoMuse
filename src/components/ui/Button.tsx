import React from 'react';

type Variant = 'primary' | 'secondary' | 'ghost';
type Size = 'sm' | 'md';

type ButtonProps = React.ButtonHTMLAttributes<HTMLButtonElement> & {
  variant?: Variant;
  size?: Size;
  loading?: boolean;
};

const variantClasses: Record<Variant, string> = {
  primary: 'bg-blue-600 text-white hover:bg-blue-700 disabled:opacity-50 disabled:cursor-not-allowed',
  secondary: 'bg-gray-100 hover:bg-gray-200 text-gray-900 disabled:opacity-50 disabled:cursor-not-allowed',
  ghost: 'text-gray-500 hover:text-gray-700 disabled:opacity-50 disabled:cursor-not-allowed',
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
      className={`${variantClasses[variant]} ${sizeClasses[size]} rounded-md font-medium ${className}`}
      disabled={disabled || loading}
      {...rest}
    >
      {loading ? 'Loading...' : children}
    </button>
  );
};

export default Button;

