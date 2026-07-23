# Module source layout

Canonical Wasm crate layout for crates under `modules/`:

- `ids.rs` — `define_surface_ids!` / `define_operation_names!` / `define_event_types!`
- `guest/` — `#[surface(guest, …)]` only
- `host/` — `#[surface(host, …)]` only (omit for guest-only modules)
- `connectors` / `connectors.rs` — when the module talks to external pools

Typed boundary ids (SDK **2.1.0+**): declare once, use typed consts at every
`Action::*` / `Surface::with_id` / `events::emit` call site.

Full rules live in the sibling SDK docs:

- [module-layout.md](../../portaki-sdk/docs/module-layout.md)
- [typed-ids.md](../../portaki-sdk/docs/typed-ids.md)
