import React from 'react';

type FormRowProps = {
  children: React.ReactNode;
  inline?: boolean;
  className?: string;
};

const FormRow: React.FC<FormRowProps> = ({ children, inline = false, className = '' }) => {
  if (inline) {
    return <div className={`flex items-end gap-3 ${className}`}>{children}</div>;
  }
  return <div className={`space-y-2 ${className}`}>{children}</div>;
};

export default FormRow;

