# KloudLab

Production-oriented cloud laboratory platform MVP. The first vertical slice is runnable locally with a Rust/Axum API, Next.js frontend, mock payment, mock environment, mock validation, 45-minute server-owned sessions, scoring, and DynamoDB Local available through Docker Compose.

## Run locally

```bash
cargo run -p kloudlab-api
cd apps/web && npm install && npm run dev
```

The API listens on `http://localhost:8080`; the web app listens on `http://localhost:3000`. Copy `.env.example` to `.env` as needed. `docker compose up dynamodb-local` starts the local database service; the current MVP repository defaults to memory so the vertical slice does not require it.

## API flow

Register/login with an email and password of at least eight characters, list labs, create and verify a mock order, start a lab, then validate tasks through `/api/v1/lab-sessions/:id/tasks/:task_id/validate`. Errors use `{ error: { code, message, request_id } }`.

See [docs/architecture.md](docs/architecture.md) for the domain boundaries, access-pattern-driven DynamoDB design, and provider migration plan.
