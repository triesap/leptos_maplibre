use crate::types::MapHandle;

pub(crate) fn set_style(handle: MapHandle, style_url: &str) {
    let _ = handle;
    let _ = style_url;
}

pub(crate) fn fly_to(
    handle: MapHandle,
    lng: f64,
    lat: f64,
    zoom: Option<f64>,
    duration_ms: Option<u32>,
) {
    let _ = handle;
    let _ = lng;
    let _ = lat;
    let _ = zoom;
    let _ = duration_ms;
}

pub(crate) fn add_geojson_source(
    handle: MapHandle,
    source_id: &str,
    geojson: &serde_json::Value,
    promote_id: Option<&str>,
) {
    let _ = handle;
    let _ = source_id;
    let _ = geojson;
    let _ = promote_id;
}

pub(crate) fn update_geojson_source(
    handle: MapHandle,
    source_id: &str,
    geojson: &serde_json::Value,
) {
    let _ = handle;
    let _ = source_id;
    let _ = geojson;
}

pub(crate) fn remove_source(handle: MapHandle, source_id: &str) {
    let _ = handle;
    let _ = source_id;
}

pub(crate) fn add_layer(
    handle: MapHandle,
    layer_id: &str,
    layer_spec: &serde_json::Value,
    before_id: Option<&str>,
) {
    let _ = handle;
    let _ = layer_id;
    let _ = layer_spec;
    let _ = before_id;
}

pub(crate) fn remove_layer(handle: MapHandle, layer_id: &str) {
    let _ = handle;
    let _ = layer_id;
}

pub(crate) fn set_feature_state(
    handle: MapHandle,
    source_id: &str,
    source_layer: Option<&str>,
    feature_id: &serde_json::Value,
    state: &serde_json::Value,
) {
    let _ = handle;
    let _ = source_id;
    let _ = source_layer;
    let _ = feature_id;
    let _ = state;
}
