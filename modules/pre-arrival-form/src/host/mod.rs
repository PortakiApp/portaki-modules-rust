//! Host dashboard surface — read-only guidance (guest submits the form).

use portaki_sdk::prelude::*;
use portaki_sdk::sdui::primitives::{Page, Text};
use portaki_sdk::sdui::surface::Surface;

/// Host main — explains the guest pre-arrival form (no config keys).
#[portaki_sdk::surface(host, id = "main")]
pub fn render_host_main(ctx: HostContext) -> Surface {
    let _ = ctx;
    Surface::new(
        Page::new()
            .title("i18n:surface.host.main.title")
            .child(
                Text::new()
                    .text("i18n:surface.host.main.subtitle")
                    .variant(TextVariant::Body),
            )
            .child(
                Text::new()
                    .text("i18n:host.main.help")
                    .variant(TextVariant::Caption),
            ),
    )
    .with_id(crate::ids::HOST_MAIN)
}
