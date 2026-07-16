//! Host dashboard surface — read-only guidance (guest submits the form).

use portaki_sdk::prelude::*;
use portaki_sdk::sdui::primitives::{Page, Text};
use portaki_sdk::sdui::surface::Surface;
use serde_json::json;

/// Host main — explains the guest pre-arrival form (no config keys).
#[portaki_sdk::surface(host, id = "main")]
pub fn render_host_main(ctx: HostContext) -> Surface {
    let _ = ctx;
    Surface::new(
        Page::new()
            .title(json!("i18n:surface.host.main.title"))
            .child(
                Text::new()
                    .text(json!("i18n:surface.host.main.subtitle"))
                    .variant(json!("body")),
            )
            .child(
                Text::new()
                    .text(json!("i18n:host.main.help"))
                    .variant(json!("caption")),
            ),
    )
    .with_id("main")
}
