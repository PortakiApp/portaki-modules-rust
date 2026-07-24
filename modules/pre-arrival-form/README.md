# pre-arrival-form

Official Portaki pre-arrival form — ETA, occasion, allergies, and a message to the host.

Part of the [`portaki-modules`](https://github.com/PortakiApp/portaki-modules) monorepo.

## Module id

`pre-arrival-form`

OCI image: `ghcr.io/portakiapp/portaki-modules-pre-arrival-form:<semver>`

## Capabilities

| Capability | Required | Purpose |
|------------|----------|---------|
| `core.storage` | Yes | `PreArrivalResponse` entity (unique per stay) |

## Surfaces

| Shell | Surface id | Description |
|-------|------------|-------------|
| guest | `home.card` | Inline form (ETA, occasion, allergies, message) or thank-you |
| host | `main` | Informational page (no config keys) — property workspace tab |
| host | `stay` | Stay-detail embed — read-only responses (`input.stayId`) |

Host manifest (`portaki.module.json`): `property-workspace-tab` + `stay-detail` (`pathSegment`: `stay`).

## Queries and commands

- `getStatus` — `{ completed, arrivalTimeEstimated?, … }` for the current stay
- `submit` — upsert response; emits `pre-arrival.completed`

## Development

```bash
cargo test -p pre-arrival-form
cd modules/pre-arrival-form
portaki build --release
```

## License

Apache-2.0 — see [LICENSE](../../LICENSE).
