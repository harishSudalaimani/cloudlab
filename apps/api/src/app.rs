use std::{collections::HashMap, sync::Arc};
use chrono::Utc;
use tokio::sync::RwLock;
use uuid::Uuid;
use crate::{domain::{self, Entitlement, Lab, LabSession, Order, TaskAttempt, User}, providers::{EnvironmentProvider, MockEnvironmentProvider, MockPaymentProvider, MockValidator, PaymentProvider, Validator}};

#[derive(Clone)] pub struct AppState { pub store: Arc<RwLock<Store>>, pub environment: Arc<dyn EnvironmentProvider>, pub validator: Arc<dyn Validator>, pub payments: Arc<dyn PaymentProvider> }
impl AppState { pub fn new() -> Self { Self { store: Arc::new(RwLock::new(Store::seeded())), environment: Arc::new(MockEnvironmentProvider), validator: Arc::new(MockValidator), payments: Arc::new(MockPaymentProvider) } } }
pub struct Store { pub users: HashMap<Uuid, User>, pub sessions: HashMap<Uuid, LabSession>, pub entitlements: Vec<Entitlement>, pub attempts: Vec<TaskAttempt>, pub orders: HashMap<Uuid, Order>, pub labs: Vec<Lab> }
impl Store { fn seeded() -> Self { Self { users: HashMap::new(), sessions: HashMap::new(), entitlements: vec![], attempts: vec![], orders: HashMap::new(), labs: domain::lab_definitions() } } }

pub fn active_entitlement(store: &Store, user_id: Uuid, lab_id: &str) -> bool { let now = Utc::now(); store.entitlements.iter().any(|e| e.user_id == user_id && e.lab_id == lab_id && e.active(now)) }
