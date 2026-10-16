use std::sync::Arc;

use tokio::sync::RwLock;

use crate::domain::Inquiry;

#[derive(Clone, Default)]
pub struct AppState {
    pub inquiries: Arc<RwLock<Vec<Inquiry>>>,
}
