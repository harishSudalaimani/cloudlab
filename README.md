# KloudLab.in

AWS-focused cloud architecture and consulting platform MVP. The frontend is a Vite/React/TypeScript app with the existing KloudLab dark/lime theme; the API is Rust/Axum; Terraform describes the deliberately small AWS baseline.

## Run locally

```bash
cargo run -p kloudlab-api
cd apps/web && npm install && npm run dev
```

The API listens on `http://localhost:8080`; the web app listens on `http://localhost:3000`. Copy `.env.example` to `.env` as needed. `docker compose up --build` starts the API and DynamoDB Local. Inquiry persistence is currently in-memory in the MVP; the Terraform baseline provisions the DynamoDB table and the next adapter can switch the repository to the AWS SDK without changing the HTTP contract.

## Product routes

`/` home, `/services`, `/work`, `/workspace`, `/about`, and `/contact`.

## API flow

The contact form posts JSON to `POST /api/v1/inquiries` and returns a receipt id. `GET /health` is available for container health checks. Errors use `{ error: { code, message, request_id } }`.

See [infra/terraform/README.md](infra/terraform/README.md) for the AWS baseline and [docs/architecture.md](docs/architecture.md) for the domain boundaries.
