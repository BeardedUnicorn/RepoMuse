import React from 'react';

type HeaderNavProps = {
  title: string;
  subtitle?: React.ReactNode;
  actions?: React.ReactNode;
  className?: string;
};

const HeaderNav: React.FC<HeaderNavProps> = ({ title, subtitle, actions, className = '' }) => {
  return (
    <nav className={`bg-white shadow-sm border-b flex-shrink-0 ${className}`}>
      <div className="max-w-full mx-auto px-4 sm:px-6 lg:px-8">
        <div className="flex justify-between h-14">
          <div className="flex items-center">
            <h1 className="text-lg font-bold text-gray-900">{title}</h1>
            {subtitle && (
              <div className="ml-4 text-sm text-gray-600">{subtitle}</div>
            )}
          </div>
          <div className="flex items-center space-x-2">{actions}</div>
        </div>
      </div>
    </nav>
  );
};

export default HeaderNav;

