-- appliances @ schema v1 — idempotent (module_appliances)

CREATE SCHEMA IF NOT EXISTS module_appliances;

CREATE TABLE IF NOT EXISTS module_appliances.appliances_content (
    id UUID PRIMARY KEY,
    property_id UUID NOT NULL,
    content_fr TEXT NOT NULL DEFAULT '',
    content_en TEXT NOT NULL DEFAULT '',
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE UNIQUE INDEX IF NOT EXISTS appliances_content_property_idx
    ON module_appliances.appliances_content (property_id);
