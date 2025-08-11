use axum::{
    extract::{Request, State},
    http::{header::AUTHORIZATION, StatusCode},
    middleware::Next,
    response::Response,
};
use std::sync::Arc;
use crate::application::traits::user_repo::UserRepository;
use crate::infrastructure::auth::jwt::{JwtService};

#[derive(Clone)]
pub struct JwtMiddlewareState {
    pub jwt_service: JwtService,
    pub user_repo: Arc<dyn UserRepository + Send + Sync>,
}

pub async fn jwt_middleware(
    State(jwt_state): State<Arc<JwtMiddlewareState>>,
    mut req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let header_value = req.headers().get(AUTHORIZATION).ok_or(StatusCode::UNAUTHORIZED)?;
    let auth_str = header_value.to_str().map_err(|_| StatusCode::UNAUTHORIZED)?;
    let token = auth_str.strip_prefix("Bearer ").ok_or(StatusCode::UNAUTHORIZED)?;

    let claims = jwt_state.jwt_service.decode_token(token)
        .map_err(|_| StatusCode::UNAUTHORIZED)?;
    
    let user = jwt_state.user_repo
        .find_by_id(claims.sub)
        .await
        .map_err(|_| StatusCode::UNAUTHORIZED)?
        .ok_or(StatusCode::UNAUTHORIZED)?;
    
    req.extensions_mut().insert(user);
    Ok(next.run(req).await)
}