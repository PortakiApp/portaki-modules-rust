# lost-found

Official Portaki lost & found — guests report lost or found items during or after the stay.

Part of the [`portaki-modules`](https://github.com/PortakiApp/portaki-modules) monorepo.

## Module id

`lost-found`

OCI image: `ghcr.io/portakiapp/portaki-modules-lost-found:<semver>`

## Capabilities

| Capability | Required | Purpose |
|------------|----------|---------|
| `core.storage` | Yes | `LostFoundReport` entity (many per stay) + KV config (`host_note`) |

## Surfaces

| Shell | Surface id | Description |
|-------|------------|-------------|
| guest | `home.card` | Kind + description form; optional host tip banner; stay report list after submit |
| host | `main` | Optional guest tip, save config, recent reports (up to 20) |

## Queries and commands

- `listForStay` — reports for the current guest stay
- `listRecent` — newest reports for the property (host)
- `submit` — create report; emits `lost-found.submitted`
- `updateConfig` — persists optional `host_note` in KV
- `emailContext` — `checkoutTips` for Portaki `lost-found` guest email when `host_note` is set

## Development

```bash
cargo test -p lost-found
cd modules/lost-found
portaki build --release
```

## License

Apache-2.0 — see [LICENSE](../../LICENSE).
