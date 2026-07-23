-- lost-found @ schema v1 — idempotent (module_lost_found)

CREATE SCHEMA IF NOT EXISTS module_lost_found;

CREATE TABLE IF NOT EXISTS module_lost_found.lost_found_report (
    id UUID PRIMARY KEY,
    stay_id UUID NOT NULL,
    kind TEXT NOT NULL,
    item_description TEXT NOT NULL,
    contact_hint TEXT,
    details TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX IF NOT EXISTS lost_found_report_stay_idx
    ON module_lost_found.lost_found_report (stay_id);
