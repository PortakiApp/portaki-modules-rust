//! Guest home booklet card.

use portaki_sdk::sdui::primitives::Card;
use portaki_sdk::sdui::surface::Surface;
use serde_json::json;

use super::body::build_hours_body;
use super::load::GuestData;

pub fn build_home_card(data: &GuestData) -> Surface {
    Surface::new(
        Card::new()
            .icon(json!("clock"))
            .title(json!("i18n:home.card.title"))
            .action(json!({
                "type": "openOverlay",
                "presentation": "page",
                "surfaceRender": "explore.detail",
                "args": {
                    "icon": "clock",
                    "title": "i18n:home.card.title"
                }
            }))
            .children(build_hours_body(data, false)),
    )
    .with_id("home.card")
}
