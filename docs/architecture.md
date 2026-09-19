# KloudLab architecture

## Scope

This phase implements the complete learner flow against a mock environment, mock validators, and mock payments. The local process uses an in-memory repository to keep startup deterministic; DynamoDB Local is included in Compose and the persistence boundary is intentionally isolated so the repository can be swapped without changing handlers or domain rules.

## Core model

`User`, `Lab`, `Task`, `Entitlement`, `Order`, `Payment`, `LabSession`, and `TaskAttempt` are provider-neutral. A session has `provider`, `environment_id`, authoritative `started_at` and `expires_at`, and a lifecycle status. No AWS account, role, or region fields are present in the core model.

## Access patterns and DynamoDB design

The production table is designed around access patterns rather than joins. A single-table option is:

| Access pattern | PK | SK |
|---|---|---|
| Get user | `USER#<id>` | `PROFILE` |
| Find user by email | `EMAIL#<normalized>` | `USER#<id>` (GSI1) |
| Get lab by slug | `LAB#<slug>` | `META` |
| List labs | `LABS` | `LAB#<slug>` |
| Get entitlement/session history | `USER#<id>` | `ENTITLEMENT#<expiry>#<lab>` / `SESSION#<started>#<id>` |
| Active entitlement for a lab | `USER#<id>` | `ENTITLEMENT#<lab>#<id>` (GSI2 with expiry) |
| Get attempts | `SESSION#<id>` | `ATTEMPT#<task>#<created>` |
| Payment lookup | `PAYMENT#<provider-id>` | `META` |

Repositories should expose `UserRepository`, `LabRepository`, `SessionRepository`, `EntitlementRepository`, `PaymentRepository`, and `ProgressRepository`. Handlers call application services, not DynamoDB clients.

## Provider boundaries

`EnvironmentProvider` owns provision/status/destroy. `Validator` owns task-specific checks. `PaymentProvider` owns order creation and verification. `MockEnvironmentProvider`, `MockValidator`, and `MockPaymentProvider` are the only implementations in this phase. `AwsEnvironmentProvider` and `KubernetesEnvironmentProvider` can be added without changing the session engine.

## Session lifecycle

Start verifies authentication and an unexpired entitlement, creates a `PROVISIONING` session, provisions an environment, then records the provider environment ID and starts the 45-minute server-side window. Every session read and validation checks expiry. Expiration transitions to `EXPIRED`; the worker contract will enqueue cleanup and eventually `DESTROYED`.

## Background jobs

The worker boundary is reserved for `ExpireLabSessions`, `CleanupLabEnvironment`, and `ProcessPaymentWebhook`. Local execution can use an in-process queue; production can use SQS.

## Future providers

The AWS implementation will provision a sandbox account/environment, issue a temporary lab role, and clean it up after expiry. Kubernetes will eventually implement the same environment contract using namespaces and scoped credentials. The KloudLab runtime remains ECS on EC2; Kubernetes is a lab provider, not the application runtime.
