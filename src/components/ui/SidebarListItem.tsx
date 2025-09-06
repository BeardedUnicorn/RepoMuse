import React from 'react';

type SidebarListItemProps = {
  selected?: boolean;
  onClick?: () => void;
  title: string;
  subtitle?: React.ReactNode;
  meta?: React.ReactNode;
  left?: React.ReactNode;
};

const SidebarListItem: React.FC<SidebarListItemProps> = ({ selected, onClick, title, subtitle, meta, left }) => {
  return (
    <div
      onClick={onClick}
      className={`${
        selected ? 'border-blue-500 bg-blue-50' : 'border-transparent'
      } p-4 cursor-pointer hover:bg-blue-50 border-l-4`}
    >
      <div className="flex items-start justify-between">
        <div className="flex-1 min-w-0">
          <div className="flex items-center space-x-2">
            <h4 className="font-medium text-gray-900 truncate text-sm">{title}</h4>
            {left}
          </div>
          {subtitle && (
            <p className="text-xs text-gray-600 mt-1 line-clamp-2">{subtitle}</p>
          )}
          {meta && <div className="flex items-center mt-2 text-xs text-gray-500 space-x-3">{meta}</div>}
        </div>
      </div>
    </div>
  );
};

export default SidebarListItem;

