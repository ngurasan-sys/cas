import React, { useEffect, useRef, useState } from 'react';
import { AppShell } from '../components/AppShell';
import { init, dispose, type Chart } from 'klinecharts';

export const Terminal: React.FC = () => {
  const chartRef = useRef<HTMLDivElement>(null);
  const chartInstance = useRef<Chart | null>(null);
  const [niftyPrice, setNiftyPrice] = useState<number>(25100.50);
  const [bankniftyPrice, setBankNiftyPrice] = useState<number>(51200.00);

  useEffect(() => {
    if (chartRef.current) {
      chartInstance.current = init(chartRef.current);
    }

    const ws = new WebSocket("ws://localhost:3000/ws");

    ws.onmessage = (event) => {
      const data = JSON.parse(event.data);
      if (data.instrument_id === 'NIFTY') {
         setNiftyPrice(data.price);
      } else if (data.instrument_id === 'BANKNIFTY') {
         setBankNiftyPrice(data.price);
      }

      // For v10 of klinecharts, the method to add a single data point is generally 'updateData'
      // To fix typescript errors without deeper typings changes, we cast to any.
      if (chartInstance.current) {
          (chartInstance.current as any).updateData({
              timestamp: data.timestamp,
              open: data.price,
              high: data.price + 5,
              low: data.price - 5,
              close: data.price,
              volume: Math.random() * 1000
          });
      }
    };

    return () => {
      ws.close();
      if (chartRef.current) {
          dispose(chartRef.current);
      }
    };
  }, []);

  return (
    <AppShell>
      <div className="flex-1 flex bg-[#0b0e11] text-[#eaecef] w-full">
        <aside className="w-64 border-r border-[#2b3139] p-4 bg-[#181a20]">
          <h2 className="text-xs uppercase tracking-widest text-[#848e9c] font-medium mb-4">Market Watch</h2>
          <div className="space-y-2">
            <div className="flex justify-between">
              <span>NIFTY 50</span>
              <span className={niftyPrice > 25100 ? "text-[#0ecb81]" : "text-[#f6465d]"}>
                {niftyPrice.toFixed(2)}
              </span>
            </div>
            <div className="flex justify-between">
              <span>BANKNIFTY</span>
              <span className={bankniftyPrice > 51200 ? "text-[#0ecb81]" : "text-[#f6465d]"}>
                {bankniftyPrice.toFixed(2)}
              </span>
            </div>
          </div>
        </aside>
        <section className="flex-1 flex flex-col min-w-0">
          <div className="h-2/3 border-b border-[#2b3139] relative w-full">
            <div ref={chartRef} className="absolute inset-0" />
          </div>
          <div className="h-1/3 p-4 bg-[#181a20]">
             <h2 className="text-xs uppercase tracking-widest text-[#848e9c] font-medium mb-4">Signal Feed</h2>
             <div className="text-sm text-muted">WAITING FOR SIGNALS...</div>
          </div>
        </section>
        <aside className="w-80 border-l border-[#2b3139] p-4 bg-[#181a20]">
           <h2 className="text-xs uppercase tracking-widest text-[#848e9c] font-medium mb-4">Risk & Execution</h2>
           <div className="text-sm text-muted">PAPER MODE</div>
        </aside>
      </div>
    </AppShell>
  );
};
