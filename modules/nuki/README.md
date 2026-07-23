# nuki

Official Portaki smart-lock **provider** for the [`access.smart_lock`](https://github.com/PortakiApp/portaki-sdk) contract. Guest unlock UX lives in **access-guide** — this module has no guest surfaces.

Part of the [`portaki-modules`](https://github.com/PortakiApp/portaki-modules) monorepo.

## Module id

`nuki`

## Binding access-guide

1. Install this module on the property.
2. Open **Accès & parking** (access-guide) → primary method **Smart lock**.
3. Set **Provider** to **nuki** (`smart_lock_provider_module_id = "nuki"`).
4. Configure keypad code here (Modules → Nuki).

When codes are revealed, access-guide sends guest commands `unlock` and `getGuestCredential` to this module with optional `{ "stayId": "…" }`.

## Capabilities

| Capability | Role |
|------------|------|
| `access.smart_lock` | **Provided** — peer discovery for access-guide |
| `core.storage` | **Required** — KV config |

No BYOK / integration capabilities in v0.1 (orchestrator credential providers are not wired for Nuki yet).

## KV config

```json
{
  "smartlock_id": "…",
  "keypad_code": "……",
  "device_name": "…"
}
```

## Guest commands (v0.1)

| Command | Behavior |
|---------|----------|
| `getGuestCredential` | `{ "type": "keypad", "code": "…", "smartlockId": "…" }` — errors if `keypad_code` empty |
| `unlock` | `{ "ok": true, "mode": "credential_fallback", "code": "…" }` when keypad configured — **no Nuki Cloud HTTP** |

Platform connector egress today only supports GET-style calls (e.g. OpenWeather). A `custom_connector` for Nuki is declared for a future POST + Bearer unlock path; enabling it requires a platform follow-up.

## Surfaces

| Shell | Surface id | Description |
|-------|------------|-------------|
| host | `main` | Smart lock ID, keypad code, device name |

## Development

```bash
cargo test -p nuki
```

## License

Apache-2.0 — see [LICENSE](../../LICENSE).
