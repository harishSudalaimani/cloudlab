use std::sync::Arc;

use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{app::AppState, domain::Inquiry};

pub fn router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/api/v1/inquiries", post(create_inquiry))
        .with_state(state)
}

async fn health() -> impl IntoResponse {
    Json(serde_json::json!({ "status": "ok" }))
}

#[derive(Deserialize)]
struct InquiryInput {
    name: String,
    email: String,
    message: String,
}

async fn create_inquiry(
    State(state): State<Arc<AppState>>,
    Json(input): Json<InquiryInput>,
) -> Result<impl IntoResponse, ApiError> {
    let inquiry = Inquiry::new(&input.name, &input.email, &input.message)
        .map_err(ApiError::invalid_inquiry)?;
    let id = inquiry.id;
    state.inquiries.write().await.push(inquiry);
    Ok((StatusCode::CREATED, Json(serde_json::json!({ "id": id, "status": "received" }))))
}

#[derive(Serialize)]
struct ErrorBody {
    error: ErrorDetail,
}

#[derive(Serialize)]
struct ErrorDetail {
    code: &'static str,
    message: &'static str,
    request_id: String,
}

struct ApiError {
    status: StatusCode,
    code: &'static str,
    message: &'static str,
}

impl ApiError {
    fn invalid_inquiry(message: &'static str) -> Self {
        Self {
            status: StatusCode::BAD_REQUEST,
            code: "INVALID_INQUIRY",
            message,
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        (
            self.status,
            Json(ErrorBody {
                error: ErrorDetail {
                    code: self.code,
                    message: self.message,
                    request_id: format!("req_{}", Uuid::new_v4()),
                },
            }),
        )
            .into_response()
    }
}
