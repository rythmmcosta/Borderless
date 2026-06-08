use axum::{Router, Json, extract::State, http::StatusCode, routing::{get, post}};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use argon2::password_hash::{rand_core::OsRng, SaltString};
use crate::{db, state::AppState, middleware::auth::issue_jwt, error::ApiError};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/register", post(register))
        .route("/login",    post(login))
        .route("/refresh",  post(refresh))
        .route("/logout",   post(logout))
        .route("/me",       get(me))
}

#[derive(Deserialize)] struct RegisterReq { email: String, password: String, display_name: Option<String> }
#[derive(Deserialize)] struct LoginReq    { email: String, password: String }
#[derive(Deserialize)] struct RefreshReq  { refresh_token: String }
#[derive(Serialize)]   struct AuthTokens  { access_token: String, refresh_token: String, expires_in: u64 }
#[derive(Serialize)]   struct MeResponse  { id: Uuid, email: String, display_name: Option<String>, role: String }

async fn register(
    State(state): State<AppState>,
    Json(req): Json<RegisterReq>,
) -> Result<(StatusCode, Json<MeResponse>), ApiError> {
    if req.email.is_empty() || req.password.len() < 8 {
        return Err(ApiError::bad_req("Email required; password must be \u{2265}8 chars"));
    }
    if db::users::get_by_email(&state.db, &req.email).await?.is_some() {
        return Err(ApiError::conflict("Email already registered"));
    }
    let salt = SaltString::generate(&mut OsRng);
    let hash = Argon2::default().hash_password(req.password.as_bytes(), &salt)
        .map_err(|_| ApiError::internal("Password hashing failed"))?.to_string();
    let role = if req.email == state.config.admin_email { "admin" } else { "user" };
    let user = db::users::create(&state.db, &req.email, req.display_name.as_deref(), &hash, role).await?;
    Ok((StatusCode::CREATED, Json(MeResponse { id: user.id, email: user.email, display_name: user.display_name, role: user.role })))
}

async fn login(
    State(state): State<AppState>,
    Json(req): Json<LoginReq>,
) -> Result<Json<AuthTokens>, ApiError> {
    let user = db::users::get_by_email(&state.db, &req.email).await?
        .ok_or_else(|| ApiError::unauth("Invalid credentials"))?;
    let hash_str = user.password_hash.as_deref().unwrap_or("");
    PasswordHash::new(hash_str)
        .and_then(|h| Argon2::default().verify_password(req.password.as_bytes(), &h))
        .map_err(|_| ApiError::unauth("Invalid credentials"))?;
    let access_token = issue_jwt(user.id, &user.email, &user.role, &state.config.jwt_secret, state.config.jwt_expiry_secs)
        .map_err(|_| ApiError::internal("Token generation failed"))?;
    let refresh_token = Uuid::new_v4().to_string();
    let exp = chrono::Utc::now() + chrono::Duration::seconds(state.config.refresh_expiry_secs as i64);
    db::users::store_refresh_token(&state.db, user.id, &refresh_token, exp).await?;
    db::users::set_last_seen(&state.db, user.id).await;
    Ok(Json(AuthTokens { access_token, refresh_token, expires_in: state.config.jwt_expiry_secs }))
}

async fn refresh(
    State(state): State<AppState>,
    Json(req): Json<RefreshReq>,
) -> Result<Json<AuthTokens>, ApiError> {
    let uid = db::users::verify_refresh_token(&state.db, &req.refresh_token).await?
        .ok_or_else(|| ApiError::unauth("Invalid or expired refresh token"))?;
    let user = db::users::get_by_id(&state.db, uid).await?.ok_or(ApiError::NotFound)?;
    let access_token = issue_jwt(user.id, &user.email, &user.role, &state.config.jwt_secret, state.config.jwt_expiry_secs)
        .map_err(|_| ApiError::internal("Token generation failed"))?;
    let new_refresh = Uuid::new_v4().to_string();
    let exp = chrono::Utc::now() + chrono::Duration::seconds(state.config.refresh_expiry_secs as i64);
    db::users::store_refresh_token(&state.db, user.id, &new_refresh, exp).await?;
    Ok(Json(AuthTokens { access_token, refresh_token: new_refresh, expires_in: state.config.jwt_expiry_secs }))
}

async fn logout(
    State(state): State<AppState>,
    Json(req): Json<RefreshReq>,
) -> StatusCode {
    db::users::revoke_refresh_token(&state.db, &req.refresh_token).await;
    StatusCode::NO_CONTENT
}

async fn me(
    State(state): State<AppState>,
    headers: axum::http::HeaderMap,
) -> Result<Json<MeResponse>, ApiError> {
    let claims = crate::middleware::auth::bearer_claims(&headers, &state.config.jwt_secret)?;
    let user = db::users::get_by_id(&state.db, claims.sub).await?.ok_or(ApiError::NotFound)?;
    Ok(Json(MeResponse { id: user.id, email: user.email, display_name: user.display_name, role: user.role }))
}
