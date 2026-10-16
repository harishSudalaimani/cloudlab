# KloudLab.in

KloudLab is a cloud engineering and consulting site with a public, unsaved architecture playground. Visitors can add and arrange AWS, GCP, Azure, Oracle Cloud, Kubernetes, IaC, CI/CD and observability components; canvas changes are temporary and are not saved. Terraform and Rust remain service offerings, not deployment requirements for this website.

## Local development

Run the Rust API:

```bash
cargo run -p kloudlab-api
```

In another terminal, run the frontend:

```bash
cd apps/web
npm ci
npm run dev
```

The website runs at `http://localhost:3000` and the API at `http://localhost:8080`. The contact form posts to `POST /api/v1/inquiries`; `GET /health` is available for health checks. Set `VITE_API_URL` when building the frontend for a non-local API URL. `WEB_ORIGIN` configures API CORS.

The API currently keeps inquiries in process memory. They are lost when the API restarts, so configure durable storage before relying on this endpoint in production.

## Docker

Build and start the API with:

```bash
docker compose up --build api
```

Build the web image separately and point it at the deployed API:

```bash
docker build --build-arg VITE_API_URL=https://api.example.com -f apps/web/Dockerfile .
```

The API image uses `apps/api/Dockerfile`. No cloud resources are created by this repository.

## Project layout

- `apps/web` — React, TypeScript and Vite frontend
- `apps/api` — Rust and Axum health/contact API
- `docker-compose.yml` — local API container

The interactive canvas is a frontend-only tool. It does not require accounts, a database, or infrastructure provisioning to run.
