import React from 'react';

export const TopStatusBar: React.FC = () => {
  return (
    <header className="h-12 border-b border-divider bg-surface flex items-center px-4 justify-between">
      <div className="font-bold text-bullish tracking-wider">INDIAN ALGO TRADING</div>
      <div className="flex gap-4 text-sm text-muted">
        <div>STATUS: <span className="text-bullish">ONLINE</span></div>
        <div>SESSION: <span className="text-content">NORMAL</span></div>
      </div>
    </header>
  );
};
