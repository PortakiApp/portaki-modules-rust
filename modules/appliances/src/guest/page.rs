//! Guest explore detail — full appliance list (page body).

use portaki_sdk::sdui::primitives::Stack;
use portaki_sdk::sdui::surface::Surface;
use serde_json::json;

use super::home::devices_list;
use crate::content::AppliancesPayload;

pub fn build_detail_page(payload: &AppliancesPayload) -> Surface {
    Surface::new(Stack::new().gap(json!(8)).children(devices_list(payload)))
        .with_id("explore.detail")
}
