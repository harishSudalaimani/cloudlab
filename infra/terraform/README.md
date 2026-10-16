# KloudLab infrastructure

This is a deliberately small baseline: private S3 origin behind CloudFront, DynamoDB on-demand, and an EC2 role for the Rust API. ACM/Route 53 can be added once the production domain is confirmed. `terraform plan` is safe and does not create resources; do not run `apply` without approval.

Secrets belong in SSM `SecureString` with a customer KMS key. GitHub Actions should assume a deploy role using OIDC rather than storing AWS keys.
