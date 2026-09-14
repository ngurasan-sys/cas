import React from 'react';
import { TopStatusBar } from './TopStatusBar';

export const AppShell: React.FC<{ children: React.ReactNode }> = ({ children }) => {
  return (
    <div className="flex flex-col min-h-screen bg-app-base text-content">
      <TopStatusBar />
      <main className="flex-1 flex overflow-hidden">
        {children}
      </main>
    </div>
  );
};
