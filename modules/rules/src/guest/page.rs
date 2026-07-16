//! Guest explore detail — full rules list (page body).

use portaki_sdk::sdui::primitives::Stack;
use portaki_sdk::sdui::surface::Surface;
use serde_json::json;

use super::home::rules_stack;
use crate::content::RulesPayload;

pub fn build_detail_page(payload: &RulesPayload) -> Surface {
    Surface::new(
        Stack::new()
            .gap(json!(0))
            .child(rules_stack(&payload.items)),
    )
    .with_id("explore.detail")
}
