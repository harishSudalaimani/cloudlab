use async_trait::async_trait;
use thiserror::Error;
use crate::domain::{ProviderKind, Task, ValidatorKind};

#[derive(Debug, Error)] pub enum ProviderError { #[error("provider operation failed: {0}")] Failed(String) }
pub struct ProvisionRequest { pub session_id: String, pub lab_id: String, pub provider: ProviderKind }
pub struct Environment { pub id: String }

#[async_trait]
pub trait EnvironmentProvider: Send + Sync { async fn provision(&self, request: ProvisionRequest) -> Result<Environment, ProviderError>; async fn destroy(&self, environment_id: &str) -> Result<(), ProviderError>; }
pub struct MockEnvironmentProvider;
#[async_trait] impl EnvironmentProvider for MockEnvironmentProvider { async fn provision(&self, request: ProvisionRequest) -> Result<Environment, ProviderError> { Ok(Environment { id: format!("mock-env-{}-{}", request.lab_id, request.session_id) }) } async fn destroy(&self, _environment_id: &str) -> Result<(), ProviderError> { Ok(()) } }

pub struct ValidationInput<'a> { pub environment_id: &'a str, pub task: &'a Task }
pub struct ValidationResult { pub passed: bool, pub message: String }
#[async_trait]
pub trait Validator: Send + Sync { async fn validate(&self, input: ValidationInput<'_>) -> Result<ValidationResult, ProviderError>; }
pub struct MockValidator;
#[async_trait] impl Validator for MockValidator { async fn validate(&self, input: ValidationInput<'_>) -> Result<ValidationResult, ProviderError> { let passed = !input.environment_id.is_empty() && !matches!(input.task.validator, ValidatorKind::MockLifecycleEnabled); let message = if passed { "Task completed successfully." } else { "The required configuration has not been detected." }; Ok(ValidationResult { passed, message: message.into() }) } }

pub struct MockPaymentProvider;
#[async_trait] pub trait PaymentProvider: Send + Sync { async fn create_order(&self, order_id: &str) -> Result<String, ProviderError>; async fn verify_payment(&self, provider_order_id: &str) -> Result<bool, ProviderError>; }
#[async_trait] impl PaymentProvider for MockPaymentProvider { async fn create_order(&self, order_id: &str) -> Result<String, ProviderError> { Ok(format!("mock-order-{order_id}")) } async fn verify_payment(&self, provider_order_id: &str) -> Result<bool, ProviderError> { Ok(!provider_order_id.contains("fail")) } }
