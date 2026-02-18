use leptos_maplibre::MapHandle;

#[cfg(target_arch = "wasm32")]
use crate::events::MapEvent;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsCast;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(module = "/bindings/js/src/map.ts")]
extern "C" {
    #[wasm_bindgen(catch, js_name = register_on_map_events)]
    fn js_register_on_map_events(handle: u32, cb: &js_sys::Function) -> Result<(), JsValue>;

    #[wasm_bindgen(catch, js_name = unregister_on_map_events)]
    fn js_unregister_on_map_events(handle: u32) -> Result<(), JsValue>;
}

#[cfg(target_arch = "wasm32")]
pub(crate) type MapEventClosure = wasm_bindgen::closure::Closure<dyn FnMut(JsValue)>;

#[cfg(target_arch = "wasm32")]
fn log_bridge_error(context: &str, error: JsValue) {
    let detail = error.as_string().unwrap_or_else(|| format!("{error:?}"));
    web_sys::console::error_1(&JsValue::from_str(&format!(
        "leptos_maplibre_ui {context}: {detail}"
    )));
}

#[cfg(target_arch = "wasm32")]
pub(crate) fn register_on_map_events(handle: MapHandle, callback: &MapEventClosure) {
    if let Err(error) = js_register_on_map_events(handle.0, callback.as_ref().unchecked_ref()) {
        log_bridge_error("register_on_map_events", error);
    }
}

#[cfg(target_arch = "wasm32")]
pub(crate) fn unregister_on_map_events(handle: MapHandle) {
    if let Err(error) = js_unregister_on_map_events(handle.0) {
        log_bridge_error("unregister_on_map_events", error);
    }
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
