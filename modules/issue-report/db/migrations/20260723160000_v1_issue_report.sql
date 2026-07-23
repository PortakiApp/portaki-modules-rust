-- issue-report @ schema v1 — idempotent (module_issue_report)

CREATE SCHEMA IF NOT EXISTS module_issue_report;

CREATE TABLE IF NOT EXISTS module_issue_report.issue_report (
    id UUID PRIMARY KEY,
    stay_id UUID NOT NULL,
    category TEXT NOT NULL,
    summary TEXT NOT NULL,
    details TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX IF NOT EXISTS issue_report_stay_idx
    ON module_issue_report.issue_report (stay_id);
