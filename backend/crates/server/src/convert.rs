//! Conversions from database models to API response types.

use api_types::{
    artists::ArtistResponse,
    playlists::{PlaylistKind, PlaylistResponse},
    tags::TagResponse,
};
use db::models::{Artist, Tag, playlist::Playlist};

use crate::error::ApiError;

/// Converts a [`Playlist`] model to a [`PlaylistResponse`].
///
/// Returns an error if the stored kind string is not a known variant.
pub(crate) fn playlist_response(playlist: Playlist) -> Result<PlaylistResponse, ApiError> {
    let kind = match playlist.kind.as_str() {
        "user" => PlaylistKind::User,
        "official" => PlaylistKind::Official,
        "favorites" => PlaylistKind::Favorites,
        other => {
            return Err(ApiError::Internal(format!(
                "unknown playlist kind in database: {other}"
            )));
        }
    };
    Ok(PlaylistResponse {
        id: playlist.id,
        title: playlist.title,
        description: playlist.description,
        kind,
        is_public: playlist.is_public,
        created_by: playlist.created_by,
    })
}

/// Converts an [`Artist`] model to an [`ArtistResponse`].
pub(crate) fn artist_response(artist: Artist) -> ArtistResponse {
    ArtistResponse {
        id: artist.id,
        name: artist.name,
        description: artist.description,
    }
}

/// Converts a [`Tag`] model to a [`TagResponse`].
pub(crate) fn tag_response(tag: Tag) -> TagResponse {
    TagResponse {
        id: tag.id,
        name: tag.name,
    }
}
