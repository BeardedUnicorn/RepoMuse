import React from 'react';

type FieldsetProps = {
  title: string;
  description?: React.ReactNode;
  actions?: React.ReactNode;
  children: React.ReactNode;
  className?: string;
};

const Fieldset: React.FC<FieldsetProps> = ({ title, description, actions, children, className = '' }) => {
  return (
    <section className={`mb-6 ${className}`}>
      <div className="flex items-start justify-between mb-3">
        <div>
          <h2 className="text-lg font-semibold text-foreground">{title}</h2>
          {description && <p className="text-sm text-foreground-secondary mt-1">{description}</p>}
        </div>
        {actions && <div className="ml-4">{actions}</div>}
      </div>
      <div className="space-y-4">{children}</div>
    </section>
  );
};

export default Fieldset;