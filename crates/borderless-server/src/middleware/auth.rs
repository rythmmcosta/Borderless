use axum::{body::Body, extract::State, http::Request, middleware::Next, response::Response};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::{error::ApiError, state::AppState};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims { pub sub: Uuid, pub email: String, pub role: String, pub exp: usize, pub iat: usize }

#[derive(Debug, Clone)]
pub struct AuthUser { pub id: Uuid, pub email: String, pub role: String }

pub async fn require_auth(State(state): State<AppState>, mut req: Request<Body>, next: Next) -> Result<Response, ApiError> {
    let token = extract_bearer(&req).ok_or_else(|| ApiError::unauth("Missing Authorization: Bearer token"))?;
    let claims = verify_jwt(&token, &state.config.jwt_secret).map_err(|_| ApiError::unauth("Invalid or expired token"))?;
    req.extensions_mut().insert(AuthUser { id: claims.sub, email: claims.email, role: claims.role });
    Ok(next.run(req).await)
}

pub async fn require_admin(req: Request<Body>, next: Next) -> Result<Response, (axum::http::StatusCode, &'static str)> {
    match req.extensions().get::<AuthUser>() {
        Some(u) if u.role == "admin" || u.role == "super_admin" => Ok(next.run(req).await),
        _ => Err((axum::http::StatusCode::FORBIDDEN, "Admin only")),
    }
}

fn extract_bearer<B>(req: &Request<B>) -> Option<String> {
    req.headers().get("Authorization")?.to_str().ok()?.strip_prefix("Bearer ").map(String::from)
}

fn verify_jwt(token: &str, secret: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let mut v = Validation::new(Algorithm::HS256); v.validate_exp = true;
    Ok(decode::<Claims>(token, &DecodingKey::from_secret(secret.as_bytes()), &v)?.claims)
}

pub fn issue_jwt(uid: Uuid, email: &str, role: &str, secret: &str, exp_secs: u64) -> Result<String, jsonwebtoken::errors::Error> {
    let now = chrono::Utc::now().timestamp() as usize;
    encode(&Header::default(), &Claims { sub: uid, email: email.into(), role: role.into(), iat: now, exp: now + exp_secs as usize }, &EncodingKey::from_secret(secret.as_bytes()))
}
