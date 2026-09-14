# Architecture Documentation

## Core Flow
1. Broker adapters stream quotes into the **Market Data Gateway**.
2. The Gateway normalizes the data against the **Instrument Master**.
3. It emits `MarketEvent`s to the asynchronous **Event Bus**.
4. Various Engines (e.g., Price, Order Flow, Option) process the events.
5. The **Strategy Router** evaluates eligibility and triggers strategies.
6. A proposed `OrderIntent` is verified by the **Risk Engine**.
7. If passed, the intent flows to the **Paper Execution Layer**.

## Technology Choices
- **Backend:** Rust, Axum, Tokio. Chosen for performance, type safety, and efficient concurrency.
- **Frontend:** React, TypeScript, TailwindCSS, KLineChart. Selected for building dynamic, unopinionated financial UI dashboards.

## Cross-Cutting Components
- Portfolio State
- Clock Service
- Reconciliation & Audit Logging
