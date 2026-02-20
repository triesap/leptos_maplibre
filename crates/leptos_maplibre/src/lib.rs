#![forbid(unsafe_code)]

mod component;
mod js;
mod types;

pub use component::MapView;
pub use types::{
    FeatureHit, MapClickEvent, MapControlAnchor, MapHandle, MapInitOptions, NativeControlOptions,
};

pub fn set_style(handle: MapHandle, style_url: &str) {
    js::set_style(handle, style_url);
}

pub fn fly_to(handle: MapHandle, lng: f64, lat: f64, zoom: Option<f64>, duration_ms: Option<u32>) {
    js::fly_to(handle, lng, lat, zoom, duration_ms);
}

pub fn jump_to(
    handle: MapHandle,
    lng: f64,
    lat: f64,
    zoom: Option<f64>,
    bearing: Option<f64>,
    pitch: Option<f64>,
) {
    js::jump_to(handle, lng, lat, zoom, bearing, pitch);
}

pub fn ease_to(
    handle: MapHandle,
    lng: f64,
    lat: f64,
    zoom: Option<f64>,
    bearing: Option<f64>,
    pitch: Option<f64>,
    duration_ms: Option<u32>,
) {
    js::ease_to(handle, lng, lat, zoom, bearing, pitch, duration_ms);
}

pub fn fit_bounds(
    handle: MapHandle,
    west: f64,
    south: f64,
    east: f64,
    north: f64,
    padding: Option<f64>,
    duration_ms: Option<u32>,
    max_zoom: Option<f64>,
) {
    js::fit_bounds(
        handle,
        west,
        south,
        east,
        north,
        padding,
        duration_ms,
        max_zoom,
    );
}

pub fn add_geojson_source(
    handle: MapHandle,
    source_id: &str,
    geojson: &serde_json::Value,
    promote_id: Option<&str>,
) {
    js::add_geojson_source(handle, source_id, geojson, promote_id);
}

pub fn add_source(handle: MapHandle, source_id: &str, source_spec: &serde_json::Value) {
    js::add_source(handle, source_id, source_spec);
}

fn with_source_type(
    source_type: &'static str,
    source_spec: &serde_json::Value,
) -> Option<serde_json::Value> {
    let mut source_object = source_spec.as_object()?.clone();
    source_object.insert(
        "type".to_string(),
        serde_json::Value::String(source_type.to_string()),
    );
    Some(serde_json::Value::Object(source_object))
}

pub fn add_vector_source(handle: MapHandle, source_id: &str, source_spec: &serde_json::Value) {
    if let Some(source_spec) = with_source_type("vector", source_spec) {
        js::add_source(handle, source_id, &source_spec);
    }
}

pub fn add_raster_source(handle: MapHandle, source_id: &str, source_spec: &serde_json::Value) {
    if let Some(source_spec) = with_source_type("raster", source_spec) {
        js::add_source(handle, source_id, &source_spec);
    }
}

pub fn add_image_source(handle: MapHandle, source_id: &str, source_spec: &serde_json::Value) {
    if let Some(source_spec) = with_source_type("image", source_spec) {
        js::add_source(handle, source_id, &source_spec);
    }
}

pub fn add_video_source(handle: MapHandle, source_id: &str, source_spec: &serde_json::Value) {
    if let Some(source_spec) = with_source_type("video", source_spec) {
        js::add_source(handle, source_id, &source_spec);
    }
}

pub fn add_canvas_source(handle: MapHandle, source_id: &str, source_spec: &serde_json::Value) {
    if let Some(source_spec) = with_source_type("canvas", source_spec) {
        js::add_source(handle, source_id, &source_spec);
    }
}

pub fn update_geojson_source(handle: MapHandle, source_id: &str, geojson: &serde_json::Value) {
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

pub fn set_layout_property(
    handle: MapHandle,
    layer_id: &str,
    property_name: &str,
    value: &serde_json::Value,
) {
    js::set_layout_property(handle, layer_id, property_name, value);
}

pub fn set_paint_property(
    handle: MapHandle,
    layer_id: &str,
    property_name: &str,
    value: &serde_json::Value,
) {
    js::set_paint_property(handle, layer_id, property_name, value);
}

pub fn set_filter(handle: MapHandle, layer_id: &str, filter: Option<&serde_json::Value>) {
    js::set_filter(handle, layer_id, filter);
}

pub fn set_layer_zoom_range(
    handle: MapHandle,
    layer_id: &str,
    min_zoom: Option<f64>,
    max_zoom: Option<f64>,
) {
    js::set_layer_zoom_range(handle, layer_id, min_zoom, max_zoom);
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

pub fn set_terrain(handle: MapHandle, terrain: Option<&serde_json::Value>) {
    js::set_terrain(handle, terrain);
}

pub fn set_fog(handle: MapHandle, fog: Option<&serde_json::Value>) {
    js::set_fog(handle, fog);
}

pub fn set_light(handle: MapHandle, light: Option<&serde_json::Value>) {
    js::set_light(handle, light);
}

#[cfg(target_arch = "wasm32")]
pub fn register_on_map_events_js(handle: MapHandle, callback: &js_sys::Function) {
    js::register_on_map_events(handle, callback);
}

#[cfg(target_arch = "wasm32")]
pub fn unregister_on_map_events_js(handle: MapHandle) {
    js::unregister_on_map_events(handle);
}

#[cfg(target_arch = "wasm32")]
pub fn register_on_layer_events_js(handle: MapHandle, layer_id: &str, callback: &js_sys::Function) {
    js::register_on_layer_events(handle, layer_id, callback);
}

#[cfg(target_arch = "wasm32")]
pub fn unregister_on_layer_events_js(handle: MapHandle, layer_id: &str) {
    js::unregister_on_layer_events(handle, layer_id);
}

#[cfg(target_arch = "wasm32")]
pub fn add_native_control_js(
    handle: MapHandle,
    control_kind: &str,
    anchor: Option<MapControlAnchor>,
    options: Option<&serde_json::Value>,
) -> Option<u32> {
    js::add_native_control(handle, control_kind, anchor, options)
}

#[cfg(target_arch = "wasm32")]
pub fn remove_native_control_js(control_handle: u32) {
    js::remove_native_control(control_handle);
}

#[cfg(target_arch = "wasm32")]
pub fn create_marker_js(
    handle: MapHandle,
    lng: f64,
    lat: f64,
    draggable: bool,
    anchor: Option<&str>,
    offset_x: Option<f64>,
    offset_y: Option<f64>,
    rotation: Option<f64>,
) -> Option<u32> {
    js::create_marker(
        handle, lng, lat, draggable, anchor, offset_x, offset_y, rotation,
    )
}

#[cfg(target_arch = "wasm32")]
pub fn update_marker_js(
    marker_handle: u32,
    lng: f64,
    lat: f64,
    draggable: bool,
    anchor: Option<&str>,
    offset_x: Option<f64>,
    offset_y: Option<f64>,
    rotation: Option<f64>,
) {
    js::update_marker(
        marker_handle,
        lng,
        lat,
        draggable,
        anchor,
        offset_x,
        offset_y,
        rotation,
    );
}

#[cfg(target_arch = "wasm32")]
pub fn remove_marker_js(marker_handle: u32) {
    js::remove_marker(marker_handle);
}

#[cfg(target_arch = "wasm32")]
pub fn register_on_marker_drag_events_js(marker_handle: u32, callback: &js_sys::Function) {
    js::register_on_marker_drag_events(marker_handle, callback);
}

#[cfg(target_arch = "wasm32")]
pub fn unregister_on_marker_drag_events_js(marker_handle: u32) {
    js::unregister_on_marker_drag_events(marker_handle);
}

#[cfg(target_arch = "wasm32")]
pub fn create_popup_js(
    handle: MapHandle,
    lng: f64,
    lat: f64,
    html: &str,
    close_button: bool,
    close_on_click: bool,
    anchor: Option<&str>,
    offset_x: Option<f64>,
    offset_y: Option<f64>,
    max_width: Option<f64>,
) -> Option<u32> {
    js::create_popup(
        handle,
        lng,
        lat,
        html,
        close_button,
        close_on_click,
        anchor,
        offset_x,
        offset_y,
        max_width,
    )
}

#[cfg(target_arch = "wasm32")]
pub fn update_popup_js(
    popup_handle: u32,
    lng: f64,
    lat: f64,
    html: &str,
    offset_x: Option<f64>,
    offset_y: Option<f64>,
    max_width: Option<f64>,
) {
    js::update_popup(popup_handle, lng, lat, html, offset_x, offset_y, max_width);
}

#[cfg(target_arch = "wasm32")]
pub fn remove_popup_js(popup_handle: u32) {
    js::remove_popup(popup_handle);
}

#[cfg(target_arch = "wasm32")]
pub fn register_on_popup_events_js(popup_handle: u32, callback: &js_sys::Function) {
    js::register_on_popup_events(popup_handle, callback);
}

#[cfg(target_arch = "wasm32")]
pub fn unregister_on_popup_events_js(popup_handle: u32) {
    js::unregister_on_popup_events(popup_handle);
}

#[cfg(test)]
mod tests {
    use super::{
        MapHandle, add_canvas_source, add_geojson_source, add_image_source, add_layer,
        add_raster_source, add_source, add_vector_source, add_video_source, ease_to, fit_bounds,
        fly_to, jump_to, remove_layer, remove_source, set_feature_state, set_filter, set_fog,
        set_layer_zoom_range, set_layout_property, set_light, set_paint_property, set_style,
        set_terrain, update_geojson_source,
    };
    use serde_json::json;

    #[test]
    fn imperative_api_noop_on_host_target() {
        let handle = MapHandle(404);
        set_style(handle, "https://demotiles.maplibre.org/style.json");
        fly_to(handle, 17.0, 59.0, Some(8.0), Some(500));
        jump_to(handle, 17.1, 59.1, Some(9.0), Some(8.0), Some(20.0));
        ease_to(
            handle,
            17.2,
            59.2,
            Some(9.5),
            Some(10.0),
            Some(25.0),
            Some(750),
        );
        fit_bounds(
            handle,
            16.0,
            58.0,
            18.0,
            60.0,
            Some(24.0),
            Some(600),
            Some(10.0),
        );
        add_source(
            handle,
            "custom-source",
            &json!({"type":"vector","tiles":["https://example.com/{z}/{x}/{y}.pbf"]}),
        );
        add_vector_source(
            handle,
            "lots-vector",
            &json!({"tiles":["https://example.com/vector/{z}/{x}/{y}.pbf"],"minzoom":0.0,"maxzoom":14.0}),
        );
        add_vector_source(handle, "lots-vector-invalid", &json!("not-an-object"));
        add_raster_source(
            handle,
            "lots-raster",
            &json!({"tiles":["https://example.com/raster/{z}/{x}/{y}.png"],"tileSize":256}),
        );
        add_image_source(
            handle,
            "overlay-image",
            &json!({"url":"https://example.com/overlay.png","coordinates":[[16.0,60.0],[18.0,60.0],[18.0,58.0],[16.0,58.0]]}),
        );
        add_video_source(
            handle,
            "overlay-video",
            &json!({"urls":["https://example.com/overlay.mp4"],"coordinates":[[16.0,60.0],[18.0,60.0],[18.0,58.0],[16.0,58.0]]}),
        );
        add_canvas_source(
            handle,
            "overlay-canvas",
            &json!({"canvas":"heatmap","coordinates":[[16.0,60.0],[18.0,60.0],[18.0,58.0],[16.0,58.0]],"animate":true}),
        );
        add_geojson_source(
            handle,
            "lots",
            &json!({"type":"FeatureCollection","features":[]}),
            None,
        );
        update_geojson_source(
            handle,
            "lots",
            &json!({"type":"FeatureCollection","features":[]}),
        );
        add_layer(
            handle,
            "lots-fill",
            &json!({"id":"lots-fill","type":"fill","source":"lots"}),
            None,
        );
        set_layout_property(handle, "lots-fill", "visibility", &json!("visible"));
        set_paint_property(handle, "lots-fill", "fill-opacity", &json!(0.5));
        set_filter(
            handle,
            "lots-fill",
            Some(&json!(["==", ["get", "lot"], "SE-1"])),
        );
        set_filter(handle, "lots-fill", None);
        set_layer_zoom_range(handle, "lots-fill", Some(2.0), Some(12.0));
        set_feature_state(
            handle,
            "lots",
            None,
            &json!("id-1"),
            &json!({"selected":true}),
        );
        set_terrain(handle, Some(&json!({"source":"dem","exaggeration":1.4})));
        set_terrain(handle, None);
        set_fog(handle, Some(&json!({"range":[0.6,8.0],"color":"#dbe7ff"})));
        set_fog(handle, None);
        set_light(
            handle,
            Some(&json!({"anchor":"viewport","color":"#ffffff"})),
        );
        set_light(handle, None);
        remove_layer(handle, "lots-fill");
        remove_source(handle, "lots");
    }
}
