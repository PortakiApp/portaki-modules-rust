//! TipTap JSON ↔ plain text helpers for descriptions / host notes.

use serde_json::Value;

/// Rough plain-text extract from TipTap JSON (or passthrough for plain strings).
pub fn to_plain_text(raw: &str) -> String {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return String::new();
    }
    let Ok(value) = serde_json::from_str::<Value>(trimmed) else {
        return trimmed.to_string();
    };
    if value.get("type").and_then(|t| t.as_str()) != Some("doc") {
        return trimmed.to_string();
    }
    let mut out = Vec::new();
    collect_text(&value, &mut out);
    out.join(" ")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

fn collect_text(node: &Value, out: &mut Vec<String>) {
    if let Some(text) = node.get("text").and_then(|t| t.as_str()) {
        if !text.is_empty() {
            out.push(text.to_string());
        }
    }
    if let Some(children) = node.get("content").and_then(|c| c.as_array()) {
        for child in children {
            collect_text(child, out);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plain_passthrough() {
        assert_eq!(to_plain_text("  Chargeur  "), "Chargeur");
    }

    #[test]
    fn tiptap_extract() {
        let json = r#"{"type":"doc","content":[{"type":"paragraph","content":[{"type":"text","text":"Doudou"}]}]}"#;
        assert_eq!(to_plain_text(json), "Doudou");
    }
}
