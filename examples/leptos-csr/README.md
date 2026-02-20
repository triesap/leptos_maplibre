# leptos_maplibre csr example

This is a Leptos CSR-only example app for `leptos_maplibre` and `leptos_maplibre_ui`.

## What it shows

- `MapView` initialization with MapLibre style and camera options.
- Hardcoded coastal GeoJSON source and layer setup.
- Layer click handling with `MapEvents` and feature-state updates.
- `Marker` and `Popup` behavior driven by map and layer interactions.

## Run

From the repository root:

- `cargo check --manifest-path examples/leptos-csr/Cargo.toml --target wasm32-unknown-unknown`
- `cd examples/leptos-csr`
- `trunk serve`

Open `http://127.0.0.1:3010`.

## UI text

- Title: `leptos_maplibre csr demo`
- Status example: `map click: lng -126.3516, lat 49.2369, features 0`
