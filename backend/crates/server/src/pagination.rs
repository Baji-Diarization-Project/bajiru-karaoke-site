//! Server side pagination policy and SQL parameter derivation.

use api_types::pagination::PaginationParams;

/// Maximum items a client may request in a single page.
pub(crate) const MAX_PER_PAGE: u32 = 200;

/// Returns the `(limit, offset)` pair for a SQL query derived from the given params.
///
/// `per_page` is clamped to [`MAX_PER_PAGE`]. `page` is 1-indexed; values below 1
/// are treated as 1 via saturating arithmetic.
pub(crate) fn limit_offset(params: &PaginationParams) -> (u32, u32) {
    let per_page = params.per_page.min(MAX_PER_PAGE);
    let offset = params.page.saturating_sub(1).saturating_mul(per_page);
    (per_page, offset)
}
