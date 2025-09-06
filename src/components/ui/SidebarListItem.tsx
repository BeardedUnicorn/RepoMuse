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
        selected ? 'border-primary bg-primary/10' : 'border-transparent hover:bg-background-tertiary'
      } p-4 cursor-pointer border-l-4 transition-colors`}
    >
      <div className="flex items-start justify-between">
        <div className="flex-1 min-w-0">
          <div className="flex items-center space-x-2">
            <h4 className="font-medium text-foreground truncate text-sm">{title}</h4>
            {left}
          </div>
          {subtitle && (
            <p className="text-xs text-foreground-secondary mt-1 line-clamp-2">{subtitle}</p>
          )}
          {meta && <div className="flex items-center mt-2 text-xs text-foreground-tertiary space-x-3">{meta}</div>}
        </div>
      </div>
    </div>
  );
};

export default SidebarListItem;