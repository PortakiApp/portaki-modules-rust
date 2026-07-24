-- lost-found @ schema v2 — status workflow (to_collect | sent | returned)

ALTER TABLE module_lost_found.lost_found_report
    ADD COLUMN IF NOT EXISTS status TEXT NOT NULL DEFAULT 'to_collect';

UPDATE module_lost_found.lost_found_report
SET status = 'to_collect'
WHERE status IS NULL OR trim(status) = '';
