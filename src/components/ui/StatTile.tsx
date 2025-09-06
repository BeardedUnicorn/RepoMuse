import React from 'react';

type Color = 'blue' | 'green' | 'purple' | 'gray';

type StatTileProps = {
  label: string;
  value: React.ReactNode;
  color?: Color;
};

const bgMap: Record<Color, string> = {
  blue: 'bg-info/10 text-foreground dark:bg-info/20',
  green: 'bg-success/10 text-foreground dark:bg-success/20',
  purple: 'bg-purple-100 text-foreground dark:bg-purple-900/30',
  gray: 'bg-background-tertiary text-foreground',
};

const valueMap: Record<Color, string> = {
  blue: 'text-info',
  green: 'text-success',
  purple: 'text-purple-600 dark:text-purple-400',
  gray: 'text-foreground',
};

const StatTile: React.FC<StatTileProps> = ({ label, value, color = 'gray' }) => {
  return (
    <div className={`${bgMap[color]} rounded-lg p-4 transition-colors`}>
      <h3 className="font-semibold mb-1">{label}</h3>
      <p className={`text-2xl font-bold ${valueMap[color]}`}>{value}</p>
    </div>
  );
};

export default StatTile;