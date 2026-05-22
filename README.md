# portaki-modules-rust

Official [Portaki](https://github.com/PortakiApp) module monorepo (Pattern A). Each crate under `modules/` is an independently versioned Wasm module published to Scaleway Container Registry.

## Structure

```
portaki-modules-rust/
├── Cargo.toml              # workspace + shared SDK git deps
├── modules/
│   └── weather/            # weather module (migrated from portaki-module-weather)
└── .github/workflows/
    ├── ci.yml              # fmt, clippy, test, portaki lint/build on PR
    └── release.yml         # publish on tag <module>-v<semver>
```

## Modules

| Module | OCI image | Description |
|--------|-----------|-------------|
| `weather` | `rg.fr-par.scw.cloud/portaki-modules/weather:<semver>` | Current weather + 5-day forecast |

## Development

```bash
rustup target add wasm32-unknown-unknown

# Install CLI (uses SDK branch with macro fix until PR #2 merges)
cargo install --git https://github.com/PortakiApp/portaki-sdk-rust --branch fix/macro-expand-emissions --locked portaki-cli

cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace

cd modules/weather
portaki build --release
portaki lint
```

## Release tagging

Official modules use **module-prefixed tags**, not bare semver:

```
weather-v0.2.0
local-guide-v1.0.0
```

The release workflow parses the tag, builds `modules/<name>/`, and runs `portaki publish --registry rg.fr-par.scw.cloud/portaki-modules`.

## Migration note

The `weather` module was migrated from standalone [portaki-module-weather](https://github.com/PortakiApp/portaki-module-weather) (C1 pilot). Standalone repo v0.1.0 was never published; first monorepo release is `weather-v0.2.0`.

See [CONTRIBUTING.md](./CONTRIBUTING.md) for adding new modules.

## License

Apache-2.0 — see [LICENSE](./LICENSE).
