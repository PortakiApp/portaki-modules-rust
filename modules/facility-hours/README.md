# facility-hours

Official Portaki facility hours module — pool, spa, and shared amenity schedules.

## Module id

`facility-hours`

## Capabilities

| Capability | Required | Purpose |
|------------|----------|---------|
| `core.storage` | Yes | KV config (`facilities_json`, `general_note`) |

## Surfaces

| Shell | Surface id | Description |
|-------|------------|-------------|
| guest | `home.card` | KeyValue hour rows |
| guest | `explore.detail` | Enriched list (page overlay) |
| host | `main` | Facilities JSON + note form |

## Development

```bash
cargo test -p facility-hours
```

## License

Apache-2.0 — see [LICENSE](../../LICENSE).
