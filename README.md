# leptos_maplibre

MapLibre GL JS bindings for Leptos in Rust/WASM.

## Goals

- Render and control a MapLibre map in Leptos apps.
- Provide an ergonomic Rust API for style, camera, sources, layers, and feature state.
- Keep core non-opinionated and leave control UI to application code.
- Initialize on client mount only for SSR and hydration safety.

## JS bridge source

The wasm bridge consumes `crates/leptos_maplibre/bindings/js/src/map.js`.

`map.js` is the source of truth and is edited directly.

## Contributing

See `CONTRIBUTING.md`.

## License

MIT OR Apache-2.0. See `LICENSE-MIT` and `LICENSE-APACHE`.
