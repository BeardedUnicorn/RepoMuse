import React, { createContext, useCallback, useContext, useMemo, useState } from 'react';

type ToastVariant = 'success' | 'error' | 'info';

type Toast = {
  id: string;
  title: string;
  description?: string;
  variant?: ToastVariant;
  duration?: number; // ms
};

type ToastContextValue = {
  toast: (t: Omit<Toast, 'id'>) => void;
};

const ToastContext = createContext<ToastContextValue | undefined>(undefined);

export const useToast = () => {
  const ctx = useContext(ToastContext);
  if (!ctx) throw new Error('useToast must be used within ToastProvider');
  return ctx;
};

const variantMap: Record<ToastVariant, string> = {
  success: 'bg-green-600',
  error: 'bg-red-600',
  info: 'bg-blue-600',
};

const ToastProvider: React.FC<{ children: React.ReactNode } & { position?: 'bottom-right' | 'top-right' | 'bottom-left' | 'top-left' }> = ({ children, position = 'bottom-right' }) => {
  const [toasts, setToasts] = useState<Toast[]>([]);

  const toast = useCallback((t: Omit<Toast, 'id'>) => {
    const id = Math.random().toString(36).slice(2);
    const item: Toast = { id, duration: 3500, variant: 'info', ...t };
    setToasts((prev) => [...prev, item]);
    window.setTimeout(() => {
      setToasts((prev) => prev.filter((x) => x.id !== id));
    }, item.duration);
  }, []);

  const value = useMemo(() => ({ toast }), [toast]);

  const posClass = useMemo(() => {
    const base = 'fixed z-50 space-y-2 p-4';
    switch (position) {
      case 'top-right':
        return `${base} top-0 right-0`;
      case 'top-left':
        return `${base} top-0 left-0`;
      case 'bottom-left':
        return `${base} bottom-0 left-0`;
      default:
        return `${base} bottom-0 right-0`;
    }
  }, [position]);

  return (
    <ToastContext.Provider value={value}>
      {children}
      <div className={posClass}>
        {toasts.map((t) => (
          <div key={t.id} className="shadow-lg rounded-md overflow-hidden min-w-[260px] max-w-sm">
            <div className={`${variantMap[t.variant || 'info']} text-white px-3 py-2 text-sm font-medium`}>{t.title}</div>
            {t.description && (
              <div className="bg-white px-3 py-2 text-sm text-gray-700 border border-t-0 border-gray-200">{t.description}</div>
            )}
          </div>
        ))}
      </div>
    </ToastContext.Provider>
  );
};

export default ToastProvider;

