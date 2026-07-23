# events

Official Portaki events module — host-curated local happenings for the guest booklet.

## Module id

`events`

## Capabilities

| Capability | Required | Purpose |
|------------|----------|---------|
| `core.storage` | Yes | KV config (`events`, `disclaimer`) |

## Surfaces

| Shell | Surface id | Description |
|-------|------------|-------------|
| guest | `home.card` | Upcoming events with day badges |
| guest | `explore.detail` | Full list, map when coordinates exist |
| host | `main` | Six event slots + disclaimer |

## Development

```bash
cargo test -p events
```

## License

Apache-2.0 — see [LICENSE](../../LICENSE).
