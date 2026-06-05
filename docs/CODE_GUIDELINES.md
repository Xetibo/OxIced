# Code Guidelines

- Keep widget helpers minimal and composable.
- Avoid adding runtime dependencies unless needed by multiple helpers.
- Keep public APIs typed and stable enough for downstream crates.
- Add unit tests for pure style/helper behavior when practical.
- Use `cargo fmt` and `cargo test` for verification.
