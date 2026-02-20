# Contributing

Thanks for your interest in contributing to leptos_maplibre.

## Ways to help

- Report bugs and regressions
- Improve documentation and examples

## Development setup

This repository is a Rust workspace. Typical tasks:

- `cargo fmt`
- `cargo test`

Example validation tasks:

- `cargo check --manifest-path examples/leptos-csr/Cargo.toml --target wasm32-unknown-unknown`
- `cd examples/leptos-csr && trunk build`

## Pull request checklist

- Keep changes focused and well-scoped
- Add or update tests when behavior changes
- Keep public APIs documented
- Avoid introducing new unsafe code

## Code style

- Use idiomatic Rust
- Prefer small, composable helpers
- Favor clear, explicit APIs over cleverness

## License

By contributing, you agree that your contributions are released under the
project license (MIT OR Apache-2.0).
