use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User { pub id: Uuid, pub email: String, pub password_hash: String }

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Lab { pub id: String, pub slug: String, pub title: String, pub provider: ProviderKind, pub difficulty: String, pub duration_minutes: u16, pub description: String, pub price_minor: u32, pub currency: String, pub objectives: Vec<String>, pub tasks: Vec<Task> }

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Task { pub id: String, pub title: String, pub description: String, pub points: u16, pub validator: ValidatorKind, pub hints: Vec<String> }

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ProviderKind { Aws, Kubernetes }

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ValidatorKind { MockBucketExists, MockVersioningEnabled, MockObjectExists, MockLifecycleEnabled, MockRoleExists, MockFunctionExists }

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SessionStatus { Created, Provisioning, Ready, Running, Validating, Completed, Expired, Cleaning, Destroyed, Failed }

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Entitlement { pub id: Uuid, pub user_id: Uuid, pub lab_id: String, pub starts_at: DateTime<Utc>, pub expires_at: DateTime<Utc> }
impl Entitlement { pub fn active(&self, now: DateTime<Utc>) -> bool { self.starts_at <= now && now < self.expires_at } }

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LabSession { pub id: Uuid, pub user_id: Uuid, pub lab_id: String, pub provider: ProviderKind, pub environment_id: Option<String>, pub status: SessionStatus, pub started_at: DateTime<Utc>, pub expires_at: DateTime<Utc>, pub completed_at: Option<DateTime<Utc>>, pub score: u16 }
impl LabSession { pub fn expired(&self, now: DateTime<Utc>) -> bool { now >= self.expires_at || matches!(self.status, SessionStatus::Expired | SessionStatus::Destroyed) } pub fn expire(&mut self) { self.status = SessionStatus::Expired; } }

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TaskAttempt { pub session_id: Uuid, pub task_id: String, pub passed: bool, pub points_awarded: u16, pub message: String, pub created_at: DateTime<Utc> }

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Order { pub id: Uuid, pub user_id: Uuid, pub lab_id: String, pub amount_minor: u32, pub currency: String, pub status: String, pub provider_order_id: String }

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Inquiry { pub id: Uuid, pub name: String, pub email: String, pub message: String, pub created_at: DateTime<Utc> }

pub fn lab_definitions() -> Vec<Lab> {
    vec![
        lab("aws-s3-101", "s3-fundamentals", "AWS S3 Fundamentals", "Learn the fundamentals of object storage with a safe simulated AWS environment.", vec!["Create and configure an S3 bucket", "Understand versioning and lifecycle controls", "Practice validating cloud infrastructure"], vec![("create-bucket", "Create an S3 bucket", ValidatorKind::MockBucketExists), ("enable-versioning", "Enable bucket versioning", ValidatorKind::MockVersioningEnabled), ("upload-object", "Upload an object", ValidatorKind::MockObjectExists), ("lifecycle", "Configure lifecycle management", ValidatorKind::MockLifecycleEnabled)]),
        lab("aws-iam-101", "iam-fundamentals", "AWS IAM Fundamentals", "Build a clear mental model for identities, roles, and least-privilege access.", vec!["Identify IAM building blocks", "Create a role for a workload", "Validate a safe identity workflow"], vec![("create-role", "Create an IAM role", ValidatorKind::MockRoleExists), ("attach-policy", "Attach a least-privilege policy", ValidatorKind::MockVersioningEnabled), ("test-access", "Test the role access path", ValidatorKind::MockObjectExists)]),
        lab("aws-lambda-101", "lambda-fundamentals", "AWS Lambda Fundamentals", "Explore event-driven compute through a provider-independent mock environment.", vec!["Create a function", "Understand invocation inputs", "Validate a serverless deployment"], vec![("create-function", "Create a Lambda function", ValidatorKind::MockFunctionExists), ("configure-trigger", "Configure an event trigger", ValidatorKind::MockVersioningEnabled), ("invoke-function", "Invoke the function", ValidatorKind::MockObjectExists)]),
    ]
}

fn lab(id: &str, slug: &str, title: &str, description: &str, objectives: Vec<&str>, task_defs: Vec<(&str, &str, ValidatorKind)>) -> Lab {
    let points = 100 / task_defs.len() as u16;
    Lab { id: id.into(), slug: slug.into(), title: title.into(), provider: ProviderKind::Aws, difficulty: "beginner".into(), duration_minutes: 45, description: description.into(), price_minor: 9900, currency: "INR".into(), objectives: objectives.into_iter().map(str::to_owned).collect(), tasks: task_defs.into_iter().map(|(id, title, validator)| Task { id: id.into(), title: title.into(), description: format!("Complete the {} step in your temporary lab environment.", title.to_lowercase()), points, validator, hints: vec!["Use the task objective as your checklist.".into(), "Validate only after you have made the change in the environment.".into()] }).collect() }
}

pub fn session_expiry(start: DateTime<Utc>) -> DateTime<Utc> { start + Duration::minutes(45) }
