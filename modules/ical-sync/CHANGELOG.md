# Changelog

## [Unreleased]

### Changed

- Config is `calendars[]` only — no more mirrored `ical_url_primary`. Legacy primary/secondary/`feeds_json` still migrate on load.

### Added

- Initial `ical-sync` module: host sheet config, ICS VEVENT parser, `listSources` / `applyFeeds` for platformFetch scheduled sync.
