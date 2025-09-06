import React from 'react';

type TextFieldProps = React.InputHTMLAttributes<HTMLInputElement> & {
  label: string;
  helpText?: string;
};

const TextField: React.FC<TextFieldProps> = ({ label, id, helpText, className = '', ...rest }) => {
  const inputId = id || rest.name || Math.random().toString(36).slice(2);
  return (
    <div>
      <label htmlFor={inputId} className="block text-sm font-medium text-foreground mb-2">
        {label}
      </label>
      <input
        id={inputId}
        className={`w-full px-3 py-2 border border-border rounded-md focus:outline-none focus:ring-2 focus:ring-primary focus:border-transparent bg-background-secondary text-foreground placeholder-foreground-tertiary ${className}`}
        {...rest}
      />
      {helpText && <p className="mt-1 text-sm text-foreground-secondary">{helpText}</p>}
    </div>
  );
};

export default TextField;