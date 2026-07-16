-- rules @ schema v1 — idempotent (module_rules)

CREATE SCHEMA IF NOT EXISTS module_rules;

CREATE TABLE IF NOT EXISTS module_rules.rules_content (
    id UUID PRIMARY KEY,
    property_id UUID NOT NULL,
    content_fr TEXT NOT NULL DEFAULT '',
    content_en TEXT NOT NULL DEFAULT '',
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE UNIQUE INDEX IF NOT EXISTS rules_content_property_idx
    ON module_rules.rules_content (property_id);
