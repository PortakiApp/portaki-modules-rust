# guest-reviews

Official Portaki guest reviews module — post-stay thank-you, Airbnb CTA + QR, or Portaki star form.

## Module id

`guest-reviews`

## Capabilities

| Capability | Required | Purpose |
|------------|----------|---------|
| `core.storage` | Yes | KV config + submitted Portaki reviews |

## Events

`submitReview` emits `guest-reviews.submitted` (`propertyId`, `rating`, `comment`, optional
`guestName`). Platform sends host transactional email `new-review`.

## Surfaces

| Shell | Surface id | Description |
|-------|------------|-------------|
| guest | `home.card` | Inline thank-you + review CTAs (no overlay) |
| host | `main` | Channel, Airbnb URL, QR toggle, thank-you message |

## Commands

- `updateConfig` — persist host settings
- `submitReview` — store Portaki rating + comment in KV

## Development

```bash
cargo test -p guest-reviews
```

## License

Apache-2.0 — see [LICENSE](../../LICENSE).
