//! Guest explore / page detail surface.

use portaki_sdk::sdui::primitives::Stack;
use portaki_sdk::sdui::surface::Surface;
use serde_json::json;

use super::body::build_access_detail;
use super::load::GuestData;

pub fn build_detail_surface(data: &GuestData) -> Surface {
    Surface::new(
        Stack::new()
            .gap(json!(12))
            .children(build_access_detail(data)),
    )
    .with_id("explore.detail")
}
