import React from 'react';

export type Option = { value: string; label: string };

type SelectProps = React.SelectHTMLAttributes<HTMLSelectElement> & {
  label: string;
  helpText?: string;
  options?: Option[];
};

const Select: React.FC<SelectProps> = ({ label, id, helpText, className = '', options, children, ...rest }) => {
  const selectId = id || rest.name || Math.random().toString(36).slice(2);
  return (
    <div>
      <label htmlFor={selectId} className="block text-sm font-medium text-gray-700 mb-2">
        {label}
      </label>
      <select
        id={selectId}
        className={`w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent ${className}`}
        {...rest}
      >
        {options ? options.map((o) => (
          <option key={o.value} value={o.value}>{o.label}</option>
        )) : children}
      </select>
      {helpText && <p className="mt-1 text-sm text-gray-500">{helpText}</p>}
    </div>
  );
};

export default Select;

