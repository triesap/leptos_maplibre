use crate::types::MapHandle;

#[cfg(target_arch = "wasm32")]
use crate::types::{MapClickEvent, MapInitOptions};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsCast;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(module = "/bindings/js/src/map.js")]
extern "C" {
    #[wasm_bindgen(catch, js_name = init_map)]
    fn js_init_map(container: &web_sys::HtmlElement, options: &JsValue) -> Result<u32, JsValue>;

    #[wasm_bindgen(catch, js_name = destroy_map)]
    fn js_destroy_map(handle: u32) -> Result<(), JsValue>;

    #[wasm_bindgen(catch, js_name = set_style)]
    fn js_set_style(handle: u32, style_url: &str) -> Result<(), JsValue>;

    #[wasm_bindgen(catch, js_name = add_geojson_source)]
    fn js_add_geojson_source(
        handle: u32,
        source_id: &str,
        geojson: &JsValue,
        promote_id: Option<&str>,
    ) -> Result<(), JsValue>;

    #[wasm_bindgen(catch, js_name = add_source)]
    fn js_add_source(handle: u32, source_id: &str, source_spec: &JsValue) -> Result<(), JsValue>;

    #[wasm_bindgen(catch, js_name = update_geojson_source)]
    fn js_update_geojson_source(
        handle: u32,
        source_id: &str,
        geojson: &JsValue,
    ) -> Result<(), JsValue>;

    #[wasm_bindgen(catch, js_name = remove_source)]
    fn js_remove_source(handle: u32, source_id: &str) -> Result<(), JsValue>;

    #[wasm_bindgen(catch, js_name = add_layer)]
    fn js_add_layer(
        handle: u32,
        layer_id: &str,
        layer_spec: &JsValue,
        before_id: Option<&str>,
    ) -> Result<(), JsValue>;

    #[wasm_bindgen(catch, js_name = remove_layer)]
    fn js_remove_layer(handle: u32, layer_id: &str) -> Result<(), JsValue>;

    #[wasm_bindgen(catch, js_name = set_layout_property)]
    fn js_set_layout_property(
        handle: u32,
        layer_id: &str,
        property_name: &str,
        value: &JsValue,
    ) -> Result<(), JsValue>;

    #[wasm_bindgen(catch, js_name = set_paint_property)]
    fn js_set_paint_property(
        handle: u32,
        layer_id: &str,
        property_name: &str,
        value: &JsValue,
    ) -> Result<(), JsValue>;

    #[wasm_bindgen(catch, js_name = set_filter)]
    fn js_set_filter(handle: u32, layer_id: &str, filter: &JsValue) -> Result<(), JsValue>;

    #[wasm_bindgen(catch, js_name = set_layer_zoom_range)]
    fn js_set_layer_zoom_range(
        handle: u32,
        layer_id: &str,
        min_zoom: Option<f64>,
        max_zoom: Option<f64>,
    ) -> Result<(), JsValue>;

    #[wasm_bindgen(catch, js_name = set_feature_state)]
    fn js_set_feature_state(
        handle: u32,
        source_id: &str,
        source_layer: Option<&str>,
        feature_id: &JsValue,
        state: &JsValue,
    ) -> Result<(), JsValue>;

    #[wasm_bindgen(catch, js_name = fly_to)]
    fn js_fly_to(
        handle: u32,
        lng: f64,
        lat: f64,
        zoom: Option<f64>,
        duration_ms: Option<u32>,
    ) -> Result<(), JsValue>;

    #[wasm_bindgen(catch, js_name = jump_to)]
    fn js_jump_to(
        handle: u32,
        lng: f64,
        lat: f64,
        zoom: Option<f64>,
        bearing: Option<f64>,
        pitch: Option<f64>,
    ) -> Result<(), JsValue>;

    #[wasm_bindgen(catch, js_name = ease_to)]
    fn js_ease_to(
        handle: u32,
        lng: f64,
        lat: f64,
        zoom: Option<f64>,
        bearing: Option<f64>,
        pitch: Option<f64>,
        duration_ms: Option<u32>,
    ) -> Result<(), JsValue>;

    #[wasm_bindgen(catch, js_name = fit_bounds)]
    fn js_fit_bounds(
        handle: u32,
        west: f64,
        south: f64,
        east: f64,
        north: f64,
        padding: Option<f64>,
        duration_ms: Option<u32>,
        max_zoom: Option<f64>,
    ) -> Result<(), JsValue>;

    #[wasm_bindgen(catch, js_name = create_marker)]
    fn js_create_marker(handle: u32, lng: f64, lat: f64, draggable: bool) -> Result<u32, JsValue>;

    #[wasm_bindgen(catch, js_name = update_marker)]
    fn js_update_marker(
        marker_handle: u32,
        lng: f64,
        lat: f64,
        draggable: bool,
    ) -> Result<(), JsValue>;

    #[wasm_bindgen(catch, js_name = remove_marker)]
    fn js_remove_marker(marker_handle: u32) -> Result<(), JsValue>;

    #[wasm_bindgen(catch, js_name = create_popup)]
    fn js_create_popup(
        handle: u32,
        lng: f64,
        lat: f64,
        html: &str,
        close_button: bool,
        close_on_click: bool,
    ) -> Result<u32, JsValue>;

    #[wasm_bindgen(catch, js_name = update_popup)]
    fn js_update_popup(popup_handle: u32, lng: f64, lat: f64, html: &str) -> Result<(), JsValue>;

    #[wasm_bindgen(catch, js_name = remove_popup)]
    fn js_remove_popup(popup_handle: u32) -> Result<(), JsValue>;

    #[wasm_bindgen(catch, js_name = register_on_click)]
    fn js_register_on_click(handle: u32, cb: &js_sys::Function) -> Result<(), JsValue>;

    #[wasm_bindgen(catch, js_name = unregister_on_click)]
    fn js_unregister_on_click(handle: u32) -> Result<(), JsValue>;

    #[wasm_bindgen(catch, js_name = register_on_load)]
    fn js_register_on_load(handle: u32, cb: &js_sys::Function) -> Result<(), JsValue>;

    #[wasm_bindgen(catch, js_name = unregister_on_load)]
    fn js_unregister_on_load(handle: u32) -> Result<(), JsValue>;

    #[wasm_bindgen(catch, js_name = register_on_map_events)]
    fn js_register_on_map_events(handle: u32, cb: &js_sys::Function) -> Result<(), JsValue>;

    #[wasm_bindgen(catch, js_name = unregister_on_map_events)]
    fn js_unregister_on_map_events(handle: u32) -> Result<(), JsValue>;

    #[wasm_bindgen(catch, js_name = register_on_layer_events)]
    fn js_register_on_layer_events(
        handle: u32,
        layer_id: &str,
        cb: &js_sys::Function,
    ) -> Result<(), JsValue>;

    #[wasm_bindgen(catch, js_name = unregister_on_layer_events)]
    fn js_unregister_on_layer_events(handle: u32, layer_id: &str) -> Result<(), JsValue>;
}

#[cfg(target_arch = "wasm32")]
pub(crate) type ClickClosure = wasm_bindgen::closure::Closure<dyn FnMut(JsValue)>;

#[cfg(target_arch = "wasm32")]
pub(crate) type ReadyClosure = wasm_bindgen::closure::Closure<dyn FnMut()>;

#[cfg(target_arch = "wasm32")]
fn log_bridge_error(context: &str, error: JsValue) {
    let detail = error.as_string().unwrap_or_else(|| format!("{error:?}"));
    web_sys::console::error_1(&JsValue::from_str(&format!(
        "leptos_maplibre {context}: {detail}"
    )));
}

#[cfg(target_arch = "wasm32")]
fn parse_json(value: &serde_json::Value, context: &str) -> Option<JsValue> {
    let encoded = match serde_json::to_string(value) {
        Ok(encoded) => encoded,
        Err(error) => {
            log_bridge_error(context, JsValue::from_str(&error.to_string()));
            return None;
        }
    };
    match js_sys::JSON::parse(&encoded) {
        Ok(parsed) => Some(parsed),
        Err(error) => {
            log_bridge_error(context, error);
            None
        }
    }
}

#[cfg(target_arch = "wasm32")]
pub(crate) fn init_map(
    container: &web_sys::HtmlElement,
    options: &MapInitOptions,
) -> Option<MapHandle> {
    let options = match serde_json::to_value(options) {
        Ok(value) => value,
        Err(error) => {
            log_bridge_error("init_map_serialize", JsValue::from_str(&error.to_string()));
            return None;
        }
    };
    let Some(options) = parse_json(&options, "init_map_options_parse") else {
        return None;
    };
    match js_init_map(container, &options) {
        Ok(raw_handle) => Some(MapHandle(raw_handle)),
        Err(error) => {
            log_bridge_error("init_map", error);
            None
        }
    }
}

#[cfg(target_arch = "wasm32")]
pub(crate) fn destroy_map(handle: MapHandle) {
    if let Err(error) = js_destroy_map(handle.0) {
        log_bridge_error("destroy_map", error);
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[allow(dead_code)]
pub(crate) fn destroy_map(_handle: MapHandle) {}

#[cfg(target_arch = "wasm32")]
pub(crate) fn register_on_click(handle: MapHandle, callback: &ClickClosure) {
    if let Err(error) = js_register_on_click(handle.0, callback.as_ref().unchecked_ref()) {
        log_bridge_error("register_on_click", error);
    }
}

#[cfg(target_arch = "wasm32")]
pub(crate) fn unregister_on_click(handle: MapHandle) {
    if let Err(error) = js_unregister_on_click(handle.0) {
        log_bridge_error("unregister_on_click", error);
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[allow(dead_code)]
pub(crate) fn unregister_on_click(_handle: MapHandle) {}

#[cfg(target_arch = "wasm32")]
pub(crate) fn register_on_load(handle: MapHandle, callback: &ReadyClosure) {
    if let Err(error) = js_register_on_load(handle.0, callback.as_ref().unchecked_ref()) {
        log_bridge_error("register_on_load", error);
    }
}

#[cfg(target_arch = "wasm32")]
pub(crate) fn unregister_on_load(handle: MapHandle) {
    if let Err(error) = js_unregister_on_load(handle.0) {
        log_bridge_error("unregister_on_load", error);
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[allow(dead_code)]
pub(crate) fn unregister_on_load(_handle: MapHandle) {}

#[cfg(target_arch = "wasm32")]
pub(crate) fn register_on_map_events(handle: MapHandle, callback: &js_sys::Function) {
    if let Err(error) = js_register_on_map_events(handle.0, callback) {
        log_bridge_error("register_on_map_events", error);
    }
}

#[cfg(target_arch = "wasm32")]
pub(crate) fn unregister_on_map_events(handle: MapHandle) {
    if let Err(error) = js_unregister_on_map_events(handle.0) {
        log_bridge_error("unregister_on_map_events", error);
    }
}

#[cfg(target_arch = "wasm32")]
pub(crate) fn register_on_layer_events(
    handle: MapHandle,
    layer_id: &str,
    callback: &js_sys::Function,
) {
    if let Err(error) = js_register_on_layer_events(handle.0, layer_id, callback) {
        log_bridge_error("register_on_layer_events", error);
    }
}

#[cfg(target_arch = "wasm32")]
pub(crate) fn unregister_on_layer_events(handle: MapHandle, layer_id: &str) {
    if let Err(error) = js_unregister_on_layer_events(handle.0, layer_id) {
        log_bridge_error("unregister_on_layer_events", error);
    }
}

#[cfg(target_arch = "wasm32")]
pub(crate) fn parse_click_payload(payload: JsValue) -> Option<MapClickEvent> {
    let json_text = match js_sys::JSON::stringify(&payload) {
        Ok(json_text) => json_text,
        Err(error) => {
            log_bridge_error("parse_click_payload_stringify", error);
            return None;
        }
    };
    let Some(json_text) = json_text.as_string() else {
        log_bridge_error(
            "parse_click_payload_stringify",
            JsValue::from_str("stringify produced non-string"),
        );
        return None;
    };
    match serde_json::from_str::<MapClickEvent>(&json_text) {
        Ok(event) => Some(event),
        Err(error) => {
            log_bridge_error(
                "parse_click_payload_decode",
                JsValue::from_str(&error.to_string()),
            );
            None
        }
    }
}

pub(crate) fn set_style(handle: MapHandle, style_url: &str) {
    #[cfg(target_arch = "wasm32")]
    if let Err(error) = js_set_style(handle.0, style_url) {
        log_bridge_error("set_style", error);
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = handle;
        let _ = style_url;
    }
}

#[allow(dead_code)]
pub(crate) fn create_marker(handle: MapHandle, lng: f64, lat: f64, draggable: bool) -> Option<u32> {
    #[cfg(target_arch = "wasm32")]
    {
        match js_create_marker(handle.0, lng, lat, draggable) {
            Ok(marker_handle) if marker_handle != 0 => Some(marker_handle),
            Ok(_) => None,
            Err(error) => {
                log_bridge_error("create_marker", error);
                None
            }
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = handle;
        let _ = lng;
        let _ = lat;
        let _ = draggable;
        None
    }
}

#[allow(dead_code)]
pub(crate) fn update_marker(marker_handle: u32, lng: f64, lat: f64, draggable: bool) {
    #[cfg(target_arch = "wasm32")]
    if let Err(error) = js_update_marker(marker_handle, lng, lat, draggable) {
        log_bridge_error("update_marker", error);
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = marker_handle;
        let _ = lng;
        let _ = lat;
        let _ = draggable;
    }
}

#[allow(dead_code)]
pub(crate) fn remove_marker(marker_handle: u32) {
    #[cfg(target_arch = "wasm32")]
    if let Err(error) = js_remove_marker(marker_handle) {
        log_bridge_error("remove_marker", error);
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = marker_handle;
    }
}

#[allow(dead_code)]
pub(crate) fn create_popup(
    handle: MapHandle,
    lng: f64,
    lat: f64,
    html: &str,
    close_button: bool,
    close_on_click: bool,
) -> Option<u32> {
    #[cfg(target_arch = "wasm32")]
    {
        match js_create_popup(handle.0, lng, lat, html, close_button, close_on_click) {
            Ok(popup_handle) if popup_handle != 0 => Some(popup_handle),
            Ok(_) => None,
            Err(error) => {
                log_bridge_error("create_popup", error);
                None
            }
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = handle;
        let _ = lng;
        let _ = lat;
        let _ = html;
        let _ = close_button;
        let _ = close_on_click;
        None
    }
}

#[allow(dead_code)]
pub(crate) fn update_popup(popup_handle: u32, lng: f64, lat: f64, html: &str) {
    #[cfg(target_arch = "wasm32")]
    if let Err(error) = js_update_popup(popup_handle, lng, lat, html) {
        log_bridge_error("update_popup", error);
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = popup_handle;
        let _ = lng;
        let _ = lat;
        let _ = html;
    }
}

#[allow(dead_code)]
pub(crate) fn remove_popup(popup_handle: u32) {
    #[cfg(target_arch = "wasm32")]
    if let Err(error) = js_remove_popup(popup_handle) {
        log_bridge_error("remove_popup", error);
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = popup_handle;
    }
}

pub(crate) fn fly_to(
    handle: MapHandle,
    lng: f64,
    lat: f64,
    zoom: Option<f64>,
    duration_ms: Option<u32>,
) {
    #[cfg(target_arch = "wasm32")]
    if let Err(error) = js_fly_to(handle.0, lng, lat, zoom, duration_ms) {
        log_bridge_error("fly_to", error);
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = handle;
        let _ = lng;
        let _ = lat;
        let _ = zoom;
        let _ = duration_ms;
    }
}

pub(crate) fn jump_to(
    handle: MapHandle,
    lng: f64,
    lat: f64,
    zoom: Option<f64>,
    bearing: Option<f64>,
    pitch: Option<f64>,
) {
    #[cfg(target_arch = "wasm32")]
    if let Err(error) = js_jump_to(handle.0, lng, lat, zoom, bearing, pitch) {
        log_bridge_error("jump_to", error);
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = handle;
        let _ = lng;
        let _ = lat;
        let _ = zoom;
        let _ = bearing;
        let _ = pitch;
    }
}

pub(crate) fn ease_to(
    handle: MapHandle,
    lng: f64,
    lat: f64,
    zoom: Option<f64>,
    bearing: Option<f64>,
    pitch: Option<f64>,
    duration_ms: Option<u32>,
) {
    #[cfg(target_arch = "wasm32")]
    if let Err(error) = js_ease_to(handle.0, lng, lat, zoom, bearing, pitch, duration_ms) {
        log_bridge_error("ease_to", error);
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = handle;
        let _ = lng;
        let _ = lat;
        let _ = zoom;
        let _ = bearing;
        let _ = pitch;
        let _ = duration_ms;
    }
}

pub(crate) fn fit_bounds(
    handle: MapHandle,
    west: f64,
    south: f64,
    east: f64,
    north: f64,
    padding: Option<f64>,
    duration_ms: Option<u32>,
    max_zoom: Option<f64>,
) {
    #[cfg(target_arch = "wasm32")]
    if let Err(error) = js_fit_bounds(
        handle.0,
        west,
        south,
        east,
        north,
        padding,
        duration_ms,
        max_zoom,
    ) {
        log_bridge_error("fit_bounds", error);
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = handle;
        let _ = west;
        let _ = south;
        let _ = east;
        let _ = north;
        let _ = padding;
        let _ = duration_ms;
        let _ = max_zoom;
    }
}

pub(crate) fn add_geojson_source(
    handle: MapHandle,
    source_id: &str,
    geojson: &serde_json::Value,
    promote_id: Option<&str>,
) {
    #[cfg(target_arch = "wasm32")]
    {
        let Some(geojson) = parse_json(geojson, "add_geojson_source_parse") else {
            return;
        };
        if let Err(error) = js_add_geojson_source(handle.0, source_id, &geojson, promote_id) {
            log_bridge_error("add_geojson_source", error);
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = handle;
        let _ = source_id;
        let _ = geojson;
        let _ = promote_id;
    }
}

pub(crate) fn add_source(handle: MapHandle, source_id: &str, source_spec: &serde_json::Value) {
    #[cfg(target_arch = "wasm32")]
    {
        let Some(source_spec) = parse_json(source_spec, "add_source_parse") else {
            return;
        };
        if let Err(error) = js_add_source(handle.0, source_id, &source_spec) {
            log_bridge_error("add_source", error);
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = handle;
        let _ = source_id;
        let _ = source_spec;
    }
}

pub(crate) fn update_geojson_source(
    handle: MapHandle,
    source_id: &str,
    geojson: &serde_json::Value,
) {
    #[cfg(target_arch = "wasm32")]
    {
        let Some(geojson) = parse_json(geojson, "update_geojson_source_parse") else {
            return;
        };
        if let Err(error) = js_update_geojson_source(handle.0, source_id, &geojson) {
            log_bridge_error("update_geojson_source", error);
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = handle;
        let _ = source_id;
        let _ = geojson;
    }
}

pub(crate) fn remove_source(handle: MapHandle, source_id: &str) {
    #[cfg(target_arch = "wasm32")]
    if let Err(error) = js_remove_source(handle.0, source_id) {
        log_bridge_error("remove_source", error);
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = handle;
        let _ = source_id;
    }
}

pub(crate) fn add_layer(
    handle: MapHandle,
    layer_id: &str,
    layer_spec: &serde_json::Value,
    before_id: Option<&str>,
) {
    #[cfg(target_arch = "wasm32")]
    {
        let Some(layer_spec) = parse_json(layer_spec, "add_layer_parse") else {
            return;
        };
        if let Err(error) = js_add_layer(handle.0, layer_id, &layer_spec, before_id) {
            log_bridge_error("add_layer", error);
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = handle;
        let _ = layer_id;
        let _ = layer_spec;
        let _ = before_id;
    }
}

pub(crate) fn remove_layer(handle: MapHandle, layer_id: &str) {
    #[cfg(target_arch = "wasm32")]
    if let Err(error) = js_remove_layer(handle.0, layer_id) {
        log_bridge_error("remove_layer", error);
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = handle;
        let _ = layer_id;
    }
}

pub(crate) fn set_layout_property(
    handle: MapHandle,
    layer_id: &str,
    property_name: &str,
    value: &serde_json::Value,
) {
    #[cfg(target_arch = "wasm32")]
    {
        let Some(value) = parse_json(value, "set_layout_property_parse") else {
            return;
        };
        if let Err(error) = js_set_layout_property(handle.0, layer_id, property_name, &value) {
            log_bridge_error("set_layout_property", error);
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = handle;
        let _ = layer_id;
        let _ = property_name;
        let _ = value;
    }
}

pub(crate) fn set_paint_property(
    handle: MapHandle,
    layer_id: &str,
    property_name: &str,
    value: &serde_json::Value,
) {
    #[cfg(target_arch = "wasm32")]
    {
        let Some(value) = parse_json(value, "set_paint_property_parse") else {
            return;
        };
        if let Err(error) = js_set_paint_property(handle.0, layer_id, property_name, &value) {
            log_bridge_error("set_paint_property", error);
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = handle;
        let _ = layer_id;
        let _ = property_name;
        let _ = value;
    }
}

pub(crate) fn set_filter(handle: MapHandle, layer_id: &str, filter: Option<&serde_json::Value>) {
    #[cfg(target_arch = "wasm32")]
    {
        let filter = if let Some(filter) = filter {
            let Some(filter) = parse_json(filter, "set_filter_parse") else {
                return;
            };
            filter
        } else {
            JsValue::NULL
        };
        if let Err(error) = js_set_filter(handle.0, layer_id, &filter) {
            log_bridge_error("set_filter", error);
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = handle;
        let _ = layer_id;
        let _ = filter;
    }
}

pub(crate) fn set_layer_zoom_range(
    handle: MapHandle,
    layer_id: &str,
    min_zoom: Option<f64>,
    max_zoom: Option<f64>,
) {
    #[cfg(target_arch = "wasm32")]
    if let Err(error) = js_set_layer_zoom_range(handle.0, layer_id, min_zoom, max_zoom) {
        log_bridge_error("set_layer_zoom_range", error);
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = handle;
        let _ = layer_id;
        let _ = min_zoom;
        let _ = max_zoom;
    }
}

pub(crate) fn set_feature_state(
    handle: MapHandle,
    source_id: &str,
    source_layer: Option<&str>,
    feature_id: &serde_json::Value,
    state: &serde_json::Value,
) {
    #[cfg(target_arch = "wasm32")]
    {
        let Some(feature_id) = parse_json(feature_id, "set_feature_state_feature_id_parse") else {
            return;
        };
        let Some(state) = parse_json(state, "set_feature_state_state_parse") else {
            return;
        };
        if let Err(error) =
            js_set_feature_state(handle.0, source_id, source_layer, &feature_id, &state)
        {
            log_bridge_error("set_feature_state", error);
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = handle;
        let _ = source_id;
        let _ = source_layer;
        let _ = feature_id;
        let _ = state;
    }
}
