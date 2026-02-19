use leptos_maplibre::{MapControlAnchor, MapHandle};

#[cfg(target_arch = "wasm32")]
use crate::events::{LayerEvent, MapEvent, MarkerDragEvent, PopupLifecycleEvent};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsCast;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsValue;

#[cfg(target_arch = "wasm32")]
pub(crate) type MapEventClosure = wasm_bindgen::closure::Closure<dyn FnMut(JsValue)>;

#[cfg(target_arch = "wasm32")]
#[allow(dead_code)]
pub(crate) type LayerEventClosure = wasm_bindgen::closure::Closure<dyn FnMut(JsValue)>;

#[cfg(target_arch = "wasm32")]
#[allow(dead_code)]
pub(crate) type MarkerDragClosure = wasm_bindgen::closure::Closure<dyn FnMut(JsValue)>;

#[cfg(not(target_arch = "wasm32"))]
#[allow(dead_code)]
pub(crate) type MarkerDragClosure = ();

#[cfg(target_arch = "wasm32")]
#[allow(dead_code)]
pub(crate) type PopupEventClosure = wasm_bindgen::closure::Closure<dyn FnMut(JsValue)>;

#[cfg(not(target_arch = "wasm32"))]
#[allow(dead_code)]
pub(crate) type PopupEventClosure = ();

#[cfg(target_arch = "wasm32")]
fn log_bridge_error(context: &str, error: JsValue) {
    let detail = error.as_string().unwrap_or_else(|| format!("{error:?}"));
    web_sys::console::error_1(&JsValue::from_str(&format!(
        "leptos_maplibre_ui {context}: {detail}"
    )));
}

#[cfg(target_arch = "wasm32")]
pub(crate) fn register_on_map_events(handle: MapHandle, callback: &MapEventClosure) {
    leptos_maplibre::register_on_map_events_js(handle, callback.as_ref().unchecked_ref());
}

#[cfg(target_arch = "wasm32")]
pub(crate) fn unregister_on_map_events(handle: MapHandle) {
    leptos_maplibre::unregister_on_map_events_js(handle);
}

#[cfg(target_arch = "wasm32")]
#[allow(dead_code)]
pub(crate) fn register_on_layer_events(
    handle: MapHandle,
    layer_id: &str,
    callback: &LayerEventClosure,
) {
    leptos_maplibre::register_on_layer_events_js(handle, layer_id, callback.as_ref().unchecked_ref());
}

#[cfg(target_arch = "wasm32")]
#[allow(dead_code)]
pub(crate) fn unregister_on_layer_events(handle: MapHandle, layer_id: &str) {
    leptos_maplibre::unregister_on_layer_events_js(handle, layer_id);
}

#[cfg(target_arch = "wasm32")]
#[allow(dead_code)]
pub(crate) fn add_native_control(
    handle: MapHandle,
    control_kind: &str,
    anchor: Option<MapControlAnchor>,
    options: Option<&serde_json::Value>,
) -> Option<u32> {
    leptos_maplibre::add_native_control_js(handle, control_kind, anchor, options)
}

#[cfg(target_arch = "wasm32")]
#[allow(dead_code)]
pub(crate) fn remove_native_control(control_handle: u32) {
    leptos_maplibre::remove_native_control_js(control_handle);
}

#[cfg(target_arch = "wasm32")]
#[allow(dead_code)]
pub(crate) fn create_marker(
    handle: MapHandle,
    lng: f64,
    lat: f64,
    draggable: bool,
    anchor: Option<&str>,
    offset_x: Option<f64>,
    offset_y: Option<f64>,
    rotation: Option<f64>,
) -> Option<u32> {
    leptos_maplibre::create_marker_js(
        handle, lng, lat, draggable, anchor, offset_x, offset_y, rotation,
    )
}

#[cfg(target_arch = "wasm32")]
#[allow(dead_code)]
pub(crate) fn update_marker(
    marker_handle: u32,
    lng: f64,
    lat: f64,
    draggable: bool,
    anchor: Option<&str>,
    offset_x: Option<f64>,
    offset_y: Option<f64>,
    rotation: Option<f64>,
) {
    leptos_maplibre::update_marker_js(
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
#[allow(dead_code)]
pub(crate) fn remove_marker(marker_handle: u32) {
    leptos_maplibre::remove_marker_js(marker_handle);
}

#[cfg(target_arch = "wasm32")]
#[allow(dead_code)]
pub(crate) fn register_on_marker_drag_events(
    marker_handle: u32,
    callback: &MarkerDragClosure,
) {
    leptos_maplibre::register_on_marker_drag_events_js(marker_handle, callback.as_ref().unchecked_ref());
}

#[cfg(target_arch = "wasm32")]
#[allow(dead_code)]
pub(crate) fn unregister_on_marker_drag_events(marker_handle: u32) {
    leptos_maplibre::unregister_on_marker_drag_events_js(marker_handle);
}

#[cfg(target_arch = "wasm32")]
#[allow(dead_code)]
pub(crate) fn create_popup(
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
    leptos_maplibre::create_popup_js(
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
#[allow(dead_code)]
pub(crate) fn update_popup(
    popup_handle: u32,
    lng: f64,
    lat: f64,
    html: &str,
    offset_x: Option<f64>,
    offset_y: Option<f64>,
    max_width: Option<f64>,
) {
    leptos_maplibre::update_popup_js(popup_handle, lng, lat, html, offset_x, offset_y, max_width);
}

#[cfg(target_arch = "wasm32")]
#[allow(dead_code)]
pub(crate) fn remove_popup(popup_handle: u32) {
    leptos_maplibre::remove_popup_js(popup_handle);
}

#[cfg(target_arch = "wasm32")]
#[allow(dead_code)]
pub(crate) fn register_on_popup_events(popup_handle: u32, callback: &PopupEventClosure) {
    leptos_maplibre::register_on_popup_events_js(popup_handle, callback.as_ref().unchecked_ref());
}

#[cfg(target_arch = "wasm32")]
#[allow(dead_code)]
pub(crate) fn unregister_on_popup_events(popup_handle: u32) {
    leptos_maplibre::unregister_on_popup_events_js(popup_handle);
}

#[cfg(not(target_arch = "wasm32"))]
#[allow(dead_code)]
pub(crate) fn unregister_on_map_events(_handle: MapHandle) {}

#[cfg(not(target_arch = "wasm32"))]
#[allow(dead_code)]
pub(crate) fn add_native_control(
    _handle: MapHandle,
    _control_kind: &str,
    _anchor: Option<MapControlAnchor>,
    _options: Option<&serde_json::Value>,
) -> Option<u32> {
    None
}

#[cfg(not(target_arch = "wasm32"))]
#[allow(dead_code)]
pub(crate) fn remove_native_control(_control_handle: u32) {}

#[cfg(not(target_arch = "wasm32"))]
#[allow(dead_code)]
pub(crate) fn create_marker(
    _handle: MapHandle,
    _lng: f64,
    _lat: f64,
    _draggable: bool,
) -> Option<u32> {
    None
}

#[cfg(not(target_arch = "wasm32"))]
#[allow(dead_code)]
pub(crate) fn update_marker(_marker_handle: u32, _lng: f64, _lat: f64, _draggable: bool) {}

#[cfg(not(target_arch = "wasm32"))]
#[allow(dead_code)]
pub(crate) fn remove_marker(_marker_handle: u32) {}

#[cfg(not(target_arch = "wasm32"))]
#[allow(dead_code)]
pub(crate) fn register_on_marker_drag_events(_marker_handle: u32, _callback: &MarkerDragClosure) {}

#[cfg(not(target_arch = "wasm32"))]
#[allow(dead_code)]
pub(crate) fn unregister_on_marker_drag_events(_marker_handle: u32) {}

#[cfg(not(target_arch = "wasm32"))]
#[allow(dead_code)]
pub(crate) fn create_popup(
    _handle: MapHandle,
    _lng: f64,
    _lat: f64,
    _html: &str,
    _close_button: bool,
    _close_on_click: bool,
    _anchor: Option<&str>,
    _offset_x: Option<f64>,
    _offset_y: Option<f64>,
    _max_width: Option<f64>,
) -> Option<u32> {
    None
}

#[cfg(not(target_arch = "wasm32"))]
#[allow(dead_code)]
pub(crate) fn update_popup(
    _popup_handle: u32,
    _lng: f64,
    _lat: f64,
    _html: &str,
    _offset_x: Option<f64>,
    _offset_y: Option<f64>,
    _max_width: Option<f64>,
) {
}

#[cfg(not(target_arch = "wasm32"))]
#[allow(dead_code)]
pub(crate) fn remove_popup(_popup_handle: u32) {}

#[cfg(not(target_arch = "wasm32"))]
#[allow(dead_code)]
pub(crate) fn register_on_popup_events(_popup_handle: u32, _callback: &PopupEventClosure) {}

#[cfg(not(target_arch = "wasm32"))]
#[allow(dead_code)]
pub(crate) fn unregister_on_popup_events(_popup_handle: u32) {}

#[cfg(target_arch = "wasm32")]
pub(crate) fn parse_map_event_payload(payload: JsValue) -> Option<MapEvent> {
    let json_text = match js_sys::JSON::stringify(&payload) {
        Ok(json_text) => json_text,
        Err(error) => {
            log_bridge_error("parse_map_event_payload_stringify", error);
            return None;
        }
    };

    let Some(json_text) = json_text.as_string() else {
        log_bridge_error(
            "parse_map_event_payload_stringify",
            JsValue::from_str("stringify produced non-string"),
        );
        return None;
    };

    match serde_json::from_str::<MapEvent>(&json_text) {
        Ok(event) => Some(event),
        Err(error) => {
            log_bridge_error(
                "parse_map_event_payload_decode",
                JsValue::from_str(&error.to_string()),
            );
            None
        }
    }
}

#[cfg(target_arch = "wasm32")]
#[allow(dead_code)]
pub(crate) fn parse_layer_event_payload(payload: JsValue) -> Option<LayerEvent> {
    let json_text = match js_sys::JSON::stringify(&payload) {
        Ok(json_text) => json_text,
        Err(error) => {
            log_bridge_error("parse_layer_event_payload_stringify", error);
            return None;
        }
    };

    let Some(json_text) = json_text.as_string() else {
        log_bridge_error(
            "parse_layer_event_payload_stringify",
            JsValue::from_str("stringify produced non-string"),
        );
        return None;
    };

    match serde_json::from_str::<LayerEvent>(&json_text) {
        Ok(event) => Some(event),
        Err(error) => {
            log_bridge_error(
                "parse_layer_event_payload_decode",
                JsValue::from_str(&error.to_string()),
            );
            None
        }
    }
}

#[cfg(target_arch = "wasm32")]
#[allow(dead_code)]
pub(crate) fn parse_marker_drag_event_payload(payload: JsValue) -> Option<MarkerDragEvent> {
    let json_text = match js_sys::JSON::stringify(&payload) {
        Ok(json_text) => json_text,
        Err(error) => {
            log_bridge_error("parse_marker_drag_event_payload_stringify", error);
            return None;
        }
    };

    let Some(json_text) = json_text.as_string() else {
        log_bridge_error(
            "parse_marker_drag_event_payload_stringify",
            JsValue::from_str("stringify produced non-string"),
        );
        return None;
    };

    match serde_json::from_str::<MarkerDragEvent>(&json_text) {
        Ok(event) => Some(event),
        Err(error) => {
            log_bridge_error(
                "parse_marker_drag_event_payload_decode",
                JsValue::from_str(&error.to_string()),
            );
            None
        }
    }
}

#[cfg(target_arch = "wasm32")]
#[allow(dead_code)]
pub(crate) fn parse_popup_lifecycle_event_payload(payload: JsValue) -> Option<PopupLifecycleEvent> {
    let json_text = match js_sys::JSON::stringify(&payload) {
        Ok(json_text) => json_text,
        Err(error) => {
            log_bridge_error("parse_popup_lifecycle_event_payload_stringify", error);
            return None;
        }
    };

    let Some(json_text) = json_text.as_string() else {
        log_bridge_error(
            "parse_popup_lifecycle_event_payload_stringify",
            JsValue::from_str("stringify produced non-string"),
        );
        return None;
    };

    match serde_json::from_str::<PopupLifecycleEvent>(&json_text) {
        Ok(event) => Some(event),
        Err(error) => {
            log_bridge_error(
                "parse_popup_lifecycle_event_payload_decode",
                JsValue::from_str(&error.to_string()),
            );
            None
        }
    }
}
