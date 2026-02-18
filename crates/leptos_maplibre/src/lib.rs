#![forbid(unsafe_code)]

mod component;
mod js;
mod types;

pub use component::MapView;
pub use types::{
    FeatureHit,
    MapClickEvent,
    MapControlAnchor,
    MapHandle,
    MapInitOptions,
    NativeControlOptions,
};

pub fn set_style(handle: MapHandle, style_url: &str) {
    js::set_style(handle, style_url);
}

pub fn fly_to(
    handle: MapHandle,
    lng: f64,
    lat: f64,
    zoom: Option<f64>,
    duration_ms: Option<u32>,
) {
    js::fly_to(handle, lng, lat, zoom, duration_ms);
}

pub fn add_geojson_source(
    handle: MapHandle,
    source_id: &str,
    geojson: &serde_json::Value,
    promote_id: Option<&str>,
) {
    js::add_geojson_source(handle, source_id, geojson, promote_id);
}

pub fn update_geojson_source(
    handle: MapHandle,
    source_id: &str,
    geojson: &serde_json::Value,
) {
    js::update_geojson_source(handle, source_id, geojson);
}

pub fn remove_source(handle: MapHandle, source_id: &str) {
    js::remove_source(handle, source_id);
}

pub fn add_layer(
    handle: MapHandle,
    layer_id: &str,
    layer_spec: &serde_json::Value,
    before_id: Option<&str>,
) {
    js::add_layer(handle, layer_id, layer_spec, before_id);
}

pub fn remove_layer(handle: MapHandle, layer_id: &str) {
    js::remove_layer(handle, layer_id);
}

pub fn set_feature_state(
    handle: MapHandle,
    source_id: &str,
    source_layer: Option<&str>,
    feature_id: &serde_json::Value,
    state: &serde_json::Value,
) {
    js::set_feature_state(handle, source_id, source_layer, feature_id, state);
}

#[cfg(target_arch = "wasm32")]
pub fn register_on_map_events_js(handle: MapHandle, callback: &js_sys::Function) {
    js::register_on_map_events(handle, callback);
}

#[cfg(target_arch = "wasm32")]
pub fn unregister_on_map_events_js(handle: MapHandle) {
    js::unregister_on_map_events(handle);
}

#[cfg(test)]
mod tests {
    use super::{
        add_geojson_source,
        add_layer,
        fly_to,
        remove_layer,
        remove_source,
        set_feature_state,
        set_style,
        update_geojson_source,
        MapHandle,
    };
    use serde_json::json;

    #[test]
    fn imperative_api_noop_on_host_target() {
        let handle = MapHandle(404);
        set_style(handle, "https://demotiles.maplibre.org/style.json");
        fly_to(handle, 17.0, 59.0, Some(8.0), Some(500));
        add_geojson_source(handle, "lots", &json!({"type":"FeatureCollection","features":[]}), None);
        update_geojson_source(handle, "lots", &json!({"type":"FeatureCollection","features":[]}));
        add_layer(
            handle,
            "lots-fill",
            &json!({"id":"lots-fill","type":"fill","source":"lots"}),
            None,
        );
        set_feature_state(handle, "lots", None, &json!("id-1"), &json!({"selected":true}));
        remove_layer(handle, "lots-fill");
        remove_source(handle, "lots");
    }
}
