use leptos_maplibre::MapHandle;

#[cfg(target_arch = "wasm32")]
use crate::events::{LayerEvent, MapEvent};

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

#[cfg(not(target_arch = "wasm32"))]
#[allow(dead_code)]
pub(crate) fn unregister_on_map_events(_handle: MapHandle) {}

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
