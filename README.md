# QuantAura

QuantAura is a full-stack application for AI-assisted trading, quantitative strategy experiments, and runtime trading operations. It includes a Rust/Axum backend and a Vue 3 frontend, with support for authentication, model and exchange configuration, strategy management, backtesting, runtime trading monitoring, debate-style decision workflows, and alerts.

## Tech Stack

- Backend: Axum, SeaORM, SQLite
- Frontend: Vue 3, Vite, Pinia, Vue Router, Tailwind CSS
- Package management: pnpm workspace
- Database: local SQLite with automatic migrations on startup

## Local Setup

Prerequisites:

- Rust 1.92+
- Node.js and pnpm
- OpenSSL

Install frontend dependencies:

```bash
pnpm install
```

Prepare local environment variables:

```bash
cp .env.example .env
```

Generate development JWT keys:

```bash
openssl genrsa -out core/private.pem 2048
openssl rsa -in core/private.pem -pubout -out core/public.pem
```

The default `.env.example` uses:

```env
JWT_PRIVATE_KEY_PATH=./private.pem
JWT_PUBLIC_KEY_PATH=./public.pem
```

Backend development scripts run from the `core/` directory, so these relative paths are resolved from `core/`. If you store the keys elsewhere, update the paths in `.env` accordingly.

## Start Development

Start the backend and frontend together:

```bash
pnpm dev
```

Or start them separately:

```bash
pnpm dev:backend
pnpm dev:frontend
```

Default URLs:

- Frontend: `http://localhost:5173`
- Backend health check: `http://localhost:8000/api/health`

The Vite development server proxies `/api` requests to `http://localhost:8000`.

## Database

Default database configuration:

```env
DB_URL=sqlite://data/quantaura.db
```

The backend starts from the `core/` directory, so the default database file is `core/data/quantaura.db`. The backend creates the database file and runs migrations automatically on startup.

## Testing

Backend tests:

```bash
cd core
cargo test
cargo test --lib
cargo test --test integration_test
```

Frontend tests:

```bash
cd web
pnpm test
pnpm test:ui
pnpm test:coverage
```

## Common Development Commands

Backend checks and formatting:

```bash
cd core
cargo fmt
cargo check
```

Frontend build check:

```bash
cd web
pnpm run build
```

## Docker Deployment

Before starting the containers, prepare the root `.env` file and generate backend JWT keys:

```bash
cp .env.example .env
openssl genrsa -out core/private.pem 2048
openssl rsa -in core/private.pem -pubout -out core/public.pem
```

The default key paths in `.env`, `./private.pem` and `./public.pem`, are relative to `core/` during local development. In Docker, they are mounted into the backend container at `/app`, so no separate Docker-specific path changes are required.

Start Docker:

```bash
docker compose up --build
```

The SQLite database is persisted in the Docker volume `quantaura-data`.

## Environment Variables

Development environment variables live in the repository root `.env` file:

- The backend reads environment variables first; during local runs it automatically tries to load `../.env` or `.env`.
- The frontend reads the root `.env` through `envDir` in `web/vite.config.ts`.
- Variables prefixed with `VITE_` are exposed to the frontend.

See `.env.example` for the common variables.
