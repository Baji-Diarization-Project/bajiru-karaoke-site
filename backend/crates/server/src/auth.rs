//! Authentication routes: password login/register and OAuth2 via Twitch and Discord.
//!
//! All auth methods converge on a single `users` table and issue the same
//! session cookie on success. See [`session`] for token issuance and storage,
//! and [`middleware`] for the `AuthUser` extractor used by protected handlers.

pub(crate) mod discord;
pub(crate) mod middleware;
pub(crate) mod password;
pub(crate) mod session;
pub(crate) mod twitch;

use axum::{
    Json, Router,
    extract::State,
    http::StatusCode,
    routing::{get, post},
};
use axum_extra::extract::CookieJar;
use cookie::Cookie;

use api_types::{
    auth::{LoginRequest, MeResponse, RegisterRequest},
    common::ErrorResponse,
};
use db::queries;

use crate::{error::ApiError, state::AppState};
use middleware::AuthUser;

#[derive(utoipa::OpenApi)]
#[openapi(
    paths(
        password::register,
        password::login,
        me,
        logout,
        twitch::initiate,
        twitch::callback,
        discord::initiate,
        discord::callback,
    ),
    components(schemas(RegisterRequest, LoginRequest, MeResponse, ErrorResponse,))
)]
pub(crate) struct AuthApi;

/// Builds the `/auth` subrouter.
pub fn router() -> Router<AppState> {
    Router::new()
        .route("/register", post(password::register))
        .route("/login", post(password::login))
        .route("/twitch", get(twitch::initiate))
        .route("/twitch/callback", get(twitch::callback))
        .route("/discord", get(discord::initiate))
        .route("/discord/callback", get(discord::callback))
        .route("/me", get(me))
        .route("/logout", post(logout))
}

#[utoipa::path(
    get,
    path = "/auth/me",
    responses(
        (status = 200, description = "Current user", body = MeResponse),
        (status = 401, description = "Not authenticated", body = ErrorResponse),
    ),
    tag = "auth"
)]
pub(crate) async fn me(
    State(state): State<AppState>,
    auth: AuthUser,
) -> Result<Json<MeResponse>, ApiError> {
    let user = queries::users::get_by_id(&state.pool, auth.user_id)
        .await?
        .ok_or(ApiError::NotFound)?;
    Ok(Json(MeResponse {
        id: user.id,
        username: user.username,
        capabilities: auth.capabilities.into_iter().collect(),
    }))
}

#[utoipa::path(
    post,
    path = "/auth/logout",
    responses(
        (status = 204, description = "Logged out, session cookie cleared"),
    ),
    tag = "auth"
)]
pub(crate) async fn logout(
    State(state): State<AppState>,
    jar: CookieJar,
) -> Result<(CookieJar, StatusCode), ApiError> {
    if let Some(token) = jar.get("session").map(|c| c.value().to_owned()) {
        session::revoke(&state.pool, &token).await?;
    }
    let mut removal = Cookie::new("session", "");
    removal.set_path("/");
    Ok((jar.remove(removal), StatusCode::NO_CONTENT))
}
