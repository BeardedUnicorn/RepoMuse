import React from 'react';

type Color = 'blue' | 'green' | 'purple' | 'gray';

type StatTileProps = {
  label: string;
  value: React.ReactNode;
  color?: Color;
};

const bgMap: Record<Color, string> = {
  blue: 'bg-blue-50 text-blue-900',
  green: 'bg-green-50 text-green-900',
  purple: 'bg-purple-50 text-purple-900',
  gray: 'bg-gray-50 text-gray-900',
};

const valueMap: Record<Color, string> = {
  blue: 'text-blue-600',
  green: 'text-green-600',
  purple: 'text-purple-600',
  gray: 'text-gray-700',
};

const StatTile: React.FC<StatTileProps> = ({ label, value, color = 'gray' }) => {
  return (
    <div className={`${bgMap[color]} rounded-lg p-4`}>
      <h3 className="font-semibold mb-1">{label}</h3>
      <p className={`text-2xl font-bold ${valueMap[color]}`}>{value}</p>
    </div>
  );
};

export default StatTile;

