import React, { useState, useMemo } from 'react';

type Tab = {
  id: string;
  label: string;
  content: React.ReactNode;
  badge?: React.ReactNode;
};

type TabsProps = {
  tabs: Tab[];
  defaultTab?: string;
  className?: string;
};

const Tabs: React.FC<TabsProps> = ({ tabs, defaultTab, className = '' }) => {
  const initial = useMemo(() => defaultTab || (tabs[0] && tabs[0].id) || '', [defaultTab, tabs]);
  const [active, setActive] = useState<string>(initial);

  const activeTab = tabs.find(t => t.id === active) || tabs[0];

  return (
    <div className={className}>
      <div className="border-b border-gray-200">
        <nav className="-mb-px flex space-x-4" aria-label="Tabs">
          {tabs.map((t) => {
            const isActive = t.id === activeTab.id;
            return (
              <button
                key={t.id}
                onClick={() => setActive(t.id)}
                className={`px-3 py-2 border-b-2 text-sm font-medium flex items-center gap-2 ${
                  isActive
                    ? 'border-blue-600 text-blue-600'
                    : 'border-transparent text-gray-500 hover:text-gray-700 hover:border-gray-300'
                }`}
              >
                <span>{t.label}</span>
                {t.badge}
              </button>
            );
          })}
        </nav>
      </div>
      <div className="mt-4">{activeTab?.content}</div>
    </div>
  );
};

export default Tabs;

