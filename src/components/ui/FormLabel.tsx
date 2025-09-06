import React from 'react';

const FormLabel: React.FC<React.LabelHTMLAttributes<HTMLLabelElement>> = ({ children, className = '', ...rest }) => (
  <label className={`block text-sm font-medium text-foreground ${className}`} {...rest}>
    {children}
  </label>
);

export default FormLabel;