# Changelog

## [Unreleased]

### Added

- Host `submitFound` command (multi-stay) + `lost-found.host-found` event for guest email.
- Report `status` (`to_collect` | `sent` | `returned`, default `to_collect`).
- Host `updateStatus` command + status Select on recent list rows.
- TipTap-ready descriptions / host note; `listForStay` accepts host `stayId`.
- `emailContext` returns `lostItemDescription` / `hasDeclaration` when stay reports exist (J+2 gate).

## [0.1.0]

### Added

- Initial `lost-found` module: guest form, host tip + recent list, `lost-found.submitted` event, `emailContext` (`checkoutTips`).
