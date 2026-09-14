# Phase 0 Report: Repository Audit and Baseline

## Repository Structure
- `backend/`: Rust backend using Axum and Tokio.
- `frontend/`: React/Vite/Tailwind frontend. Unused Vite scaffolding (`App.css`, `App.tsx`) has been removed.
- `docs/`: Markdown documentation.
- `requirements/`: Initial spec.

## Current Architecture
The current application successfully serves a React frontend and connects it via WebSockets to a Rust backend. The backend utilizes a `SyntheticDataGenerator` to push random pricing data onto an `EventBus`, which is then streamed to the UI. The core domain directories (`engines/`, `strategies/`, `portfolio/`, `execution/`, `brokers/`, `instruments/`, `risk/`) have been scaffolded with basic structs and traits.

## Build & Test Results
- `cargo check`: Passes cleanly.
- `cargo test`: Passes.
- `npm run build`: Passes successfully.
- `npm run lint`: Passes successfully with zero warnings.

## Missing Functionality
The platform is currently a functional skeleton beyond the basic WebSocket streaming. The majority of the logic inside the engines, strategies, and portfolio tracking is stubbed. Broker integrations and historical data storage (Redis/Clickhouse/Parquet) are not yet implemented.

## Known Problems
- The frontend `Terminal.tsx` uses `any` casting to bypass a TypeScript error with `klinecharts`.
- Several backend modules trigger "dead code" warnings due to incomplete implementations, which are suppressed globally for test targets.
- Synthetic data generator does not yet populate realistic option chain or futures attributes.