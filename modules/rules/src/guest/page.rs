//! Guest explore detail — full rules list (page body).

use portaki_sdk::sdui::primitives::Stack;
use portaki_sdk::sdui::surface::Surface;

use super::home::rules_stack;
use crate::content::RulesPayload;

pub fn build_detail_page(payload: &RulesPayload) -> Surface {
    Surface::new(
        Stack::new()
            .gap(0.0)
            .child(rules_stack(&payload.items)),
    )
    .with_id(crate::ids::EXPLORE_DETAIL)
}
