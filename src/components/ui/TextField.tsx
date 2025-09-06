import React from 'react';

type TextFieldProps = React.InputHTMLAttributes<HTMLInputElement> & {
  label: string;
  helpText?: string;
};

const TextField: React.FC<TextFieldProps> = ({ label, id, helpText, className = '', ...rest }) => {
  const inputId = id || rest.name || Math.random().toString(36).slice(2);
  return (
    <div>
      <label htmlFor={inputId} className="block text-sm font-medium text-gray-700 mb-2">
        {label}
      </label>
      <input
        id={inputId}
        className={`w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent ${className}`}
        {...rest}
      />
      {helpText && <p className="mt-1 text-sm text-gray-500">{helpText}</p>}
    </div>
  );
};

export default TextField;

