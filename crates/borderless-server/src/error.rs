use axum::{http::StatusCode, response::{IntoResponse, Response}, Json};

#[derive(Debug)]
pub enum ApiError { NotFound, Unauthorized(String), Forbidden, Conflict(String), BadRequest(String), Internal(anyhow::Error) }

impl ApiError {
    pub fn conflict(msg: &str) -> Self { Self::Conflict(msg.into()) }
    pub fn bad_req(msg: &str)  -> Self { Self::BadRequest(msg.into()) }
    pub fn unauth(msg: &str)   -> Self { Self::Unauthorized(msg.into()) }
    pub fn internal(msg: &str) -> Self { Self::Internal(anyhow::anyhow!("{}", msg)) }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, msg) = match self {
            Self::NotFound        => (StatusCode::NOT_FOUND, "Not found".into()),
            Self::Unauthorized(m) => (StatusCode::UNAUTHORIZED, m),
            Self::Forbidden       => (StatusCode::FORBIDDEN, "Forbidden".into()),
            Self::Conflict(m)     => (StatusCode::CONFLICT, m),
            Self::BadRequest(m)   => (StatusCode::BAD_REQUEST, m),
            Self::Internal(e)     => { tracing::error!("Internal: {:#}", e); (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error".into()) }
        };
        (status, Json(serde_json::json!({ "error": msg }))).into_response()
    }
}

impl From<sqlx::Error>   for ApiError { fn from(e: sqlx::Error)   -> Self { Self::Internal(e.into()) } }
impl From<anyhow::Error> for ApiError { fn from(e: anyhow::Error) -> Self { Self::Internal(e) } }
