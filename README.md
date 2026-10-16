# KloudLab.in

Cloud engineering and consulting platform MVP covering AWS, Terraform, Kubernetes, and Rust. The frontend is a Vite/React/TypeScript app with the existing KloudLab dark/lime theme; the API is Rust/Axum; Terraform describes the deliberately small AWS baseline.

## Run locally

```bash
cargo run -p kloudlab-api
cd apps/web && npm install && npm run dev
```

The API listens on `http://localhost:8080`; the web app listens on `http://localhost:3000`. Copy `.env.example` to `.env` as needed. `WEB_ORIGIN` must match the browser origin so credentialed account requests work. `docker compose up --build` starts the API and DynamoDB Local. Account, saved-design, template, and inquiry records are currently held in memory and reset when the API restarts; DynamoDB persistence remains a follow-up before production use.

## Product routes

`/` home, `/services`, `/work`, `/playground`, `/about`, `/contact`, and `/login`. The Playground opens with a connected sample website architecture and provides AWS, GCP, Azure, Oracle Cloud Infrastructure, Terraform, AWS CDK, Kubernetes, CI/CD, and observability components; cross-cloud designs can be mixed freely. Drag a node to move it, drag or tap between circular ports to create directional connections, and remove connections from the canvas or the live connection list. Rust is offered as a separate engineering service and is not a Playground component. Visitors can experiment with temporary local edits; accounts unlock saved designs and reusable templates. The former `/workspace` URL remains available as a legacy alias.

## API flow

The contact form posts JSON to `POST /api/v1/inquiries` and returns a receipt id. Accounts use `POST /api/v1/auth/register` and `/api/v1/auth/login`; authenticated design management uses `GET` and `POST /api/v1/designs`. `GET /health` is available for container health checks. Errors use `{ error: { code, message, request_id } }`.

See [infra/terraform/README.md](infra/terraform/README.md) for the AWS baseline and engineering capabilities and [docs/architecture.md](docs/architecture.md) for the domain boundaries.
