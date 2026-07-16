# sections

Official Portaki editorial sections — title + markdown body blocks for the guest booklet.

Part of the [`portaki-modules`](https://github.com/PortakiApp/portaki-modules) monorepo.

## Module id

`sections`

OCI image: `ghcr.io/portakiapp/portaki-modules-sections:<semver>`

## Capabilities

| Capability | Required | Purpose |
|------------|----------|---------|
| `core.storage` | Yes | `SectionItem` + `SectionItemLocale` |

## Data model

- `section_item` — id, sort_order
- `section_item_locale` — section_id, lang, title, body_markdown

No TipTap — markdown strings only.

## Surfaces

| Shell | Surface id | Description |
|-------|------------|-------------|
| guest | `home.card` | Teaser + first sections |
| guest | `explore.sheet` | Full bodies (bottom sheet) |
| host | `main` | Form editor for primary section |

Host workspace tab: `pathSegment = "sections"`.

## Queries and commands

- `listSections`
- `saveSection`
- `deleteSection`
- `reorder`

## Development

```bash
cargo test -p sections
```

## License

Apache-2.0 — see [LICENSE](../../LICENSE).
