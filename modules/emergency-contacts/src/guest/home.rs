//! Guest home booklet card.

use portaki_sdk::sdui::primitives::Card;
use portaki_sdk::sdui::surface::Surface;
use serde_json::json;

use super::body::build_contacts_body;
use super::load::GuestData;

pub fn build_home_card(data: &GuestData) -> Surface {
    Surface::new(
        Card::new()
            .icon(json!("phone"))
            .title(json!("i18n:home.card.title"))
            .action(json!({
                "type": "openOverlay",
                "presentation": "bottomSheet",
                "surfaceRender": "explore.detail",
                "args": {
                    "icon": "phone",
                    "title": "i18n:home.card.title"
                }
            }))
            .children(build_contacts_body(data, false)),
    )
    .with_id("home.card")
}
