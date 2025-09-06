import React from 'react';

type CardProps = {
  children: React.ReactNode;
  className?: string;
};

const Card: React.FC<CardProps> = ({ children, className = '' }) => {
  return (
    <div className={`bg-background-secondary rounded-lg border border-border ${className}`}>
      {children}
    </div>
  );
};

export default Card;