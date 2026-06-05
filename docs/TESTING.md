# Testing

## Strategy

- Unit tests cover pure helper/style behavior.
- UI rendering remains manually verified in downstream applications.
- Downstream crates should run their own tests after adopting new helpers.

## Commands

- `cargo fmt --all`
- `nix develop -c cargo test`
- `nix develop -c cargo clippy --all-targets`
