-- checklist @ schema v1 — idempotent (module_checklist)

CREATE SCHEMA IF NOT EXISTS module_checklist;

CREATE TABLE IF NOT EXISTS module_checklist.checklist_item (
    id UUID PRIMARY KEY,
    property_id UUID NOT NULL,
    label_fr TEXT NOT NULL DEFAULT '',
    label_en TEXT NOT NULL DEFAULT '',
    sort_order INTEGER NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX IF NOT EXISTS checklist_item_property_sort_idx
    ON module_checklist.checklist_item (property_id, sort_order);

CREATE TABLE IF NOT EXISTS module_checklist.checklist_completion (
    id UUID PRIMARY KEY,
    stay_id UUID NOT NULL,
    item_id UUID NOT NULL,
    completed_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT checklist_completion_stay_item_uq UNIQUE (stay_id, item_id)
);

CREATE INDEX IF NOT EXISTS checklist_completion_stay_idx
    ON module_checklist.checklist_completion (stay_id);
