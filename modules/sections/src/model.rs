//! Shared view / DTO types for sections.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// One locale block for create/update.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct SectionLocaleInput {
    pub lang: String,
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub body_markdown: String,
}

/// Guest/host view of a section for one resolved locale.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SectionView {
    pub id: Uuid,
    pub sort_order: i32,
    pub title: String,
    pub body_markdown: String,
    pub locales: Vec<SectionLocaleInput>,
}

impl SectionView {
    pub fn is_blank(&self) -> bool {
        self.title.trim().is_empty() && self.body_markdown.trim().is_empty()
    }
}

pub fn pick_locale_fields(
    locales: &[SectionLocaleInput],
    locale: &str,
) -> (String, String) {
    let prefer_en = locale.to_ascii_lowercase().starts_with("en");
    let want = if prefer_en { "en" } else { "fr" };
    let fallback = if prefer_en { "fr" } else { "en" };

    let primary = locales.iter().find(|l| lang_matches(&l.lang, want));
    let secondary = locales.iter().find(|l| lang_matches(&l.lang, fallback));

    if let Some(primary) = primary {
        if !primary.title.trim().is_empty() || !primary.body_markdown.trim().is_empty() {
            return (primary.title.clone(), primary.body_markdown.clone());
        }
    }
    if let Some(secondary) = secondary {
        return (secondary.title.clone(), secondary.body_markdown.clone());
    }
    if let Some(any) = locales.first() {
        return (any.title.clone(), any.body_markdown.clone());
    }
    (String::new(), String::new())
}

fn lang_matches(lang: &str, want: &str) -> bool {
    lang.to_ascii_lowercase().starts_with(want)
}
