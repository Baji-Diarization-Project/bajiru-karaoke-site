//! OpenAPI spec assembly and Swagger UI endpoint.

use axum::Router;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::{
    auth::AuthApi,
    routes::{
        performances::PerformancesApi, playlists::PlaylistsApi, songs::SongsApi, tags::TagsApi,
        users::UsersApi,
    },
    state::AppState,
};

/// Builds the `/docs` router, serving the merged OpenAPI spec and Swagger UI.
pub fn router() -> Router<AppState> {
    let mut spec = SongsApi::openapi();
    spec.merge(PerformancesApi::openapi());
    spec.merge(PlaylistsApi::openapi());
    spec.merge(TagsApi::openapi());
    spec.merge(UsersApi::openapi());
    spec.merge(AuthApi::openapi());

    Router::new().merge(SwaggerUi::new("/docs").url("/docs/openapi.json", spec))
}
