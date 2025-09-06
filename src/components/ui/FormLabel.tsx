import React from 'react';

const FormLabel: React.FC<React.LabelHTMLAttributes<HTMLLabelElement>> = ({ children, className = '', ...rest }) => (
  <label className={`block text-sm font-medium text-gray-700 ${className}`} {...rest}>
    {children}
  </label>
);

export default FormLabel;

