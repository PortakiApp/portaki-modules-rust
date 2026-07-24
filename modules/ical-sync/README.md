# ical-sync

Official Portaki **host-only** module: import stays from iCal / Airbnb (and other `.ics`) calendar export URLs.

Part of the [`portaki-modules`](https://github.com/PortakiApp/portaki-modules) monorepo.

## Module id

`ical-sync`

## Audience

Host dashboard only — no guest booklet surfaces.

## Host surfaces

| Type | pathSegment | Role |
|------|-------------|------|
| `property-module-sheet` | `ical-sync` | Config cards in the module configure sheet |
| `property-stats-card` | `calendar-sync` | Compact card on the property stats page |

## Scheduled / manual sync

Manifest `hostScheduledSync` uses the platform-fetch path:

1. Query `listSources` → feed URLs  
2. Platform HTTPS-fetches each `.ics` body  
3. Query `applyFeeds` → parses VEVENT rows + updates `last_sync_at` / `sync_summary`  
4. Platform imports stays (`guestName`, `checkInAt`, `checkOutAt`, `icalUid`, …)

Manual trigger: `POST /api/v1/properties/{id}/modules/ical-sync/sync`.

## Capabilities

| Capability | Role |
|------------|------|
| `core.storage` | **Required** — KV config |
| `core.ical.import` | **Required** — plan allowance for calendar import |

## KV config

```json
{
  "calendars": [
    { "id": "cal-1", "url": "https://…/calendar.ics", "label": "Airbnb" },
    { "id": "cal-2", "url": "https://…/other.ics" }
  ],
  "ical_url_primary": "https://…/calendar.ics",
  "last_sync_at": "2026-07-23T08:12:00Z",
  "sync_summary": "3 stay(s) · 1 feed(s) ok · 0 feed(s) failed"
}
```

`calendars` is the source of truth (dynamic list). `ical_url_primary` is mirrored from the first connected URL for platform `property.icalUrl` sync. Legacy `ical_url_secondary` / `feeds_json` migrate on load.

Soft UI cap: 20 calendar rows (`CALENDAR_SLOTS`).

## Queries / commands

| Op | Kind | Role |
|----|------|------|
| `getConfig` | query | Read config |
| `updateConfig` | command | Save calendar list |
| `listSources` | query | Sources for platform fetch |
| `applyFeeds` | query | Parse ICS bodies → stay rows |

## Development

```bash
cargo test -p ical-sync
```

## License

Apache-2.0 — see [LICENSE](../../LICENSE).
