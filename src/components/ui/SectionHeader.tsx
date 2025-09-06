import React from 'react';

type SectionHeaderProps = {
  title: string;
  actions?: React.ReactNode;
  className?: string;
};

const SectionHeader: React.FC<SectionHeaderProps> = ({ title, actions, className = '' }) => {
  return (
    <div className={`flex justify-between items-center mb-4 ${className}`}>
      <h2 className="text-lg font-semibold text-gray-900">{title}</h2>
      {actions}
    </div>
  );
};

export default SectionHeader;

