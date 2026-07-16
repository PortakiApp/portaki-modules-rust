# checklist

Official Portaki checkout checklist — stay-scoped toggles for guests, JSON item editor for hosts.

Part of the [`portaki-modules`](https://github.com/PortakiApp/portaki-modules) monorepo.

## Module id

`checklist`

OCI image: `ghcr.io/portakiapp/portaki-modules-checklist:<semver>`

Host workspace tab: `pathSegment = checklist` (surface `main`).

## Capabilities

| Capability | Required | Purpose |
|------------|----------|---------|
| `core.storage` | Yes | `ChecklistItem` + `ChecklistCompletion` entities |

## Surfaces

| Shell | Surface id | Description |
|-------|------------|-------------|
| guest | `home.card` | Progress caption + inline toggles (no overlay) |
| host | `main` | JSON form → `replaceItems` |

## Queries and commands

- `listItems` — property checklist items
- `listCompletions` — completed item ids for the current stay
- `replaceItems` — host replace-all (`items` or `itemsJson`)
- `completeItem` / `uncompleteItem` — guest toggles (`itemId`)

## Development

```bash
cargo test -p checklist
cd modules/checklist
portaki build --release
```

## License

Apache-2.0 — see [LICENSE](../../LICENSE).
