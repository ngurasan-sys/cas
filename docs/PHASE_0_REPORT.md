# Phase 0 Report: Repository Audit and Baseline

## Repository Structure
- `backend/`: Rust backend using Axum and Tokio.
- `frontend/`: React/Vite/Tailwind frontend.
- `docs/`: Markdown documentation.
- `requirements/`: Initial spec.

## Current Architecture
The current application successfully serves a React frontend and connects it via WebSockets to a Rust backend. The backend utilizes a `SyntheticDataGenerator` to push random pricing data onto an `EventBus`, which is then streamed to the UI. The core domain directories (`engines/`, `strategies/`, `portfolio/`, `execution/`, `brokers/`, `instruments/`, `risk/`) have been scaffolded with basic structs and traits.

## Build & Test Results
- `cargo check`: Passes (with some unused code warnings related to stubs).
- `cargo test`: Passes (1 test for `RiskEngine` validation).
- `npm run build`: Passes successfully.

## Missing Functionality
The platform is currently an unconnected skeleton beyond the basic WebSocket streaming. The majority of the logic inside the engines, strategies, and portfolio tracking is stubbed. Critical foundations such as structured logging, correlation IDs, market clock, and robust error handling are entirely missing. Broker integrations and historical data storage (Redis/Clickhouse/Parquet) are not yet implemented.

## Known Problems
- The frontend `Terminal.tsx` uses `any` casting to bypass a TypeScript error with `klinecharts`.
- Several backend modules trigger "dead code" warnings due to incomplete implementations.
- Synthetic data is not labeled "SIMULATED" on the UI.
- No tests exist beyond a trivial risk limit check.