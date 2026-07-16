//! Shared guest SDUI building blocks.

use portaki_sdk::prelude::*;
use portaki_sdk::sdui::common::{Emphasis, Tone};
use portaki_sdk::sdui::primitives::{Icon, Stack, Text};
use serde_json::json;

pub fn metric_label(icon: &str, label: &str) -> Component {
    Component::Stack(
        Stack::new()
            .direction(json!("horizontal"))
            .gap(json!(6))
            .child(Icon::new().name(json!(icon)).size(json!(14)))
            .child(
                Text::new()
                    .text(json!(label))
                    .variant(json!("caption"))
                    .emphasis(Emphasis::Strong),
            ),
    )
}

/// One metric tile: icon + label on top, value below — two tiles per grid row.
pub fn metric_tile(icon: &str, label: &str, value: &str, value_tone: Option<Tone>) -> Component {
    let mut value_text = Text::new().text(json!(value)).variant(json!("caption"));
    if let Some(tone) = value_tone {
        value_text = value_text.tone(tone);
    }
    Component::Stack(
        Stack::new()
            .gap(json!(2))
            .child(metric_label(icon, label))
            .child(Component::Text(value_text)),
    )
}

pub fn table_header_cell(label: &str) -> Component {
    Component::Text(
        Text::new()
            .text(json!(label))
            .variant(json!("caption"))
            .emphasis(Emphasis::Strong),
    )
}

pub fn table_value_cell(value: &str) -> Component {
    Component::Text(Text::new().text(json!(value)).variant(json!("caption")))
}

pub fn optional_pct(value: Option<u8>) -> String {
    value
        .map(|pct| format!("{pct}%"))
        .unwrap_or_else(|| "—".to_string())
}
