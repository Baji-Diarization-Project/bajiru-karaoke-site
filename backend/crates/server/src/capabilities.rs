//! Capability title constants and authorization mappings.
//!
//! Each constant matches a row in the `capabilities` table. Handlers compare
//! against [`AuthUser::capabilities`] which is fetched fresh on every request.

use api_types::playlists::PlaylistKind;

pub const PLAYLISTS_VIEW_PRIVATE: &str = "playlists:view_private";
pub const PLAYLISTS_CREATE_OFFICIAL: &str = "playlists:create_official";
pub const PLAYLISTS_CREATE_FAVORITES: &str = "playlists:create_favorites";
pub const PLAYLISTS_MANAGE_ANY: &str = "playlists:manage_any";

/// Returns the capability required to create a playlist of the given kind, or
/// `None` if any authenticated user may create it.
pub fn required_playlist_create_capability(kind: &PlaylistKind) -> Option<&'static str> {
    match kind {
        PlaylistKind::User => None,
        PlaylistKind::Official => Some(PLAYLISTS_CREATE_OFFICIAL),
        PlaylistKind::Favorites => Some(PLAYLISTS_CREATE_FAVORITES),
    }
}
