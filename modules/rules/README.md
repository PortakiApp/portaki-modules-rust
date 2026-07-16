# rules

Official Portaki house rules module — structured bilingual items for guest booklets and the host dashboard.

Part of the [`portaki-modules`](https://github.com/PortakiApp/portaki-modules) monorepo.

## Module id

`rules`

OCI image: `ghcr.io/portakiapp/portaki-modules-rules:<semver>`

## Capabilities

| Capability | Required | Purpose |
|------------|----------|---------|
| `core.storage` | Yes | `RulesContent` entity |

## Content model

`content_fr` / `content_en` store structured JSON (not TipTap):

```json
{
  "items": [
    { "icon": "clock-circle", "title": "Quiet after 10 pm", "subtitle": "Please respect neighbours" }
  ]
}
```

## Surfaces

| Shell | Surface id | Description |
|-------|------------|-------------|
| guest | `home.card` | Glance of up to 4 rules |
| guest | `explore.detail` | Full rules list (page body) |
| host | `main` | Bilingual JSON editor form |

Host workspace tab: `pathSegment = "rules"` (see `portaki.module.json`).

## Queries and commands

- `getContent` — locale-aware items + raw FR/EN JSON
- `saveContent` — upsert `RulesContent`

## Development

```bash
cargo test -p rules
cd modules/rules
portaki build --release
```

## License

Apache-2.0 — see [LICENSE](../../LICENSE).
