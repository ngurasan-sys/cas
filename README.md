# Indian Algo Trading Platform

## Overview
This repository contains the Indian-market algorithmic trading research and paper-trading platform. It features a modular architecture designed for high maintainability, with independent market intelligence engines and multiple trading strategies communicating over an event bus.

## Directory Structure
- `backend/`: Rust backend implementing core event loops, market data gateways, instrument masters, and algorithmic execution logic.
- `frontend/`: React/Vite/Tailwind frontend built as a single-page professional trading terminal.
- `research/`: Python notebooks and logic for offline quantitative research and model training.
- `docs/`: Comprehensive architecture and module documentation.
- `requirements/`: Initial specification and requirements checklists.
- `tests/`: End-to-end and integration tests.
- `docker/`: Container configurations for platform services like Redis, ClickHouse, etc.

## Setup Instructions
Please see the `docs/` folder for more detailed instructions on building, running, and testing both frontend and backend modules locally.

## License
[Add correct license here]
