# lost-found

Official Portaki lost & found — host-declared found items (guest email) and guest self-reports (host email).

Part of the [`portaki-modules`](https://github.com/PortakiApp/portaki-modules) monorepo.

## Module id

`lost-found`

OCI image: `ghcr.io/portakiapp/portaki-modules-lost-found:<semver>`

## Capabilities

| Capability | Required | Purpose |
|------------|----------|---------|
| `core.storage` | Yes | `LostFoundReport` entity (many per stay) + KV config (`host_note`) |

## Data model

`LostFoundReport` (schema v2):

| Field | Notes |
|-------|--------|
| `kind` | `lost` \| `found` |
| `item_description` | Plain text (guest) or TipTap JSON (host-found) |
| `status` | `to_collect` (default, « À récupérer ») \| `sent` (« Envoyé ») \| `returned` (« Récupéré ») |
| `contact_hint` / `details` | Guest optional fields |

## Surfaces

| Shell | Surface id | Description |
|-------|------------|-------------|
| guest | `home.card` | Kind + description form; optional host tip banner; stay report list after submit |
| host | `main` | Banner, TipTap host note, recent reports (create UI is React in the dashboard) |

Host stay chrome (button + modal) lives in the dashboard — no SDK `stay-action` surface type.

## Queries and commands

- `listForStay` — guest stay reports; host may pass `stayId`
- `listRecent` — newest reports for the property (host)
- `submit` — guest create report; emits `lost-found.submitted` → host email
- `submitFound` — host create found report(s) for one or more `stayIds` (shared description/status); emits `lost-found.host-found` per stay → guest `lost-found` email
- `updateStatus` — host change report status (`to_collect` \| `sent` \| `returned`) after create
- `updateConfig` — persists optional `host_note` in KV (TipTap JSON ok)
- `emailContext` — for Portaki `lost-found` guest email: `checkoutTips` (host note), `lostItemDescription` + `hasDeclaration` from stay reports

## Development

```bash
cargo test -p lost-found
cd modules/lost-found
portaki build --release
```

## License

Apache-2.0 — see [LICENSE](../../LICENSE).
