-- sections @ schema v1 — idempotent (module_sections)

CREATE SCHEMA IF NOT EXISTS module_sections;

CREATE TABLE IF NOT EXISTS module_sections.section_item (
    id UUID PRIMARY KEY,
    property_id UUID NOT NULL,
    sort_order INT NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX IF NOT EXISTS section_item_property_sort_idx
    ON module_sections.section_item (property_id, sort_order);

CREATE TABLE IF NOT EXISTS module_sections.section_item_locale (
    id UUID PRIMARY KEY,
    property_id UUID NOT NULL,
    section_id UUID NOT NULL,
    lang TEXT NOT NULL,
    title TEXT NOT NULL DEFAULT '',
    body_markdown TEXT NOT NULL DEFAULT '',
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE UNIQUE INDEX IF NOT EXISTS section_item_locale_section_lang_idx
    ON module_sections.section_item_locale (section_id, lang);

CREATE INDEX IF NOT EXISTS section_item_locale_property_idx
    ON module_sections.section_item_locale (property_id);
