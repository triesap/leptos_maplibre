use leptos::prelude::*;
use leptos_maplibre::MapHandle;

use crate::events::{LayerEvent, MapEvent};

#[cfg(target_arch = "wasm32")]
use crate::events::LayerEventKind;

#[component]
pub fn MapEvents(
    handle: MapHandle,
    #[prop(optional, into)] layer_id: Option<String>,
    #[prop(optional)] on_event: Option<Callback<MapEvent>>,
    #[prop(optional)] on_move: Option<Callback<MapEvent>>,
    #[prop(optional)] on_zoom: Option<Callback<MapEvent>>,
    #[prop(optional)] on_idle: Option<Callback<MapEvent>>,
    #[prop(optional)] on_style_data: Option<Callback<MapEvent>>,
    #[prop(optional)] on_data: Option<Callback<MapEvent>>,
    #[prop(optional)] on_layer_event: Option<Callback<LayerEvent>>,
    #[prop(optional)] on_layer_click: Option<Callback<LayerEvent>>,
    #[prop(optional)] on_layer_double_click: Option<Callback<LayerEvent>>,
    #[prop(optional)] on_layer_context_menu: Option<Callback<LayerEvent>>,
    #[prop(optional)] on_layer_mouse_enter: Option<Callback<LayerEvent>>,
    #[prop(optional)] on_layer_mouse_move: Option<Callback<LayerEvent>>,
    #[prop(optional)] on_layer_mouse_leave: Option<Callback<LayerEvent>>,
) -> impl IntoView {
    #[cfg(target_arch = "wasm32")]
    {
        use send_wrapper::SendWrapper;
        use wasm_bindgen::closure::Closure;
        use wasm_bindgen::JsValue;

        let layer_id = layer_id.clone();
        let on_event = on_event.clone();
        let on_move = on_move.clone();
        let on_zoom = on_zoom.clone();
        let on_idle = on_idle.clone();
        let on_style_data = on_style_data.clone();
        let on_data = on_data.clone();
        let on_layer_event = on_layer_event.clone();
        let on_layer_click = on_layer_click.clone();
        let on_layer_double_click = on_layer_double_click.clone();
        let on_layer_context_menu = on_layer_context_menu.clone();
        let on_layer_mouse_enter = on_layer_mouse_enter.clone();
        let on_layer_mouse_move = on_layer_mouse_move.clone();
        let on_layer_mouse_leave = on_layer_mouse_leave.clone();

        let has_map_callbacks = on_event.is_some()
            || on_move.is_some()
            || on_zoom.is_some()
            || on_idle.is_some()
            || on_style_data.is_some()
            || on_data.is_some();
        let map_callback = if has_map_callbacks {
            Some(Closure::wrap(Box::new(move |payload: JsValue| {
                let Some(event) = crate::js::parse_map_event_payload(payload) else {
                    return;
                };

                if let Some(on_event) = on_event.as_ref() {
                    on_event.run(event.clone());
                }

                match event.kind {
                    crate::events::MapEventKind::Move => {
                        if let Some(on_move) = on_move.as_ref() {
                            on_move.run(event);
                        }
                    }
                    crate::events::MapEventKind::Zoom => {
                        if let Some(on_zoom) = on_zoom.as_ref() {
                            on_zoom.run(event);
                        }
                    }
                    crate::events::MapEventKind::Idle => {
                        if let Some(on_idle) = on_idle.as_ref() {
                            on_idle.run(event);
                        }
                    }
                    crate::events::MapEventKind::StyleData => {
                        if let Some(on_style_data) = on_style_data.as_ref() {
                            on_style_data.run(event);
                        }
                    }
                    crate::events::MapEventKind::Data => {
                        if let Some(on_data) = on_data.as_ref() {
                            on_data.run(event);
                        }
                    }
                }
            }) as Box<dyn FnMut(_)>))
        } else {
            None
        };
        if let Some(map_callback) = map_callback.as_ref() {
            crate::js::register_on_map_events(handle, map_callback);
        }
        let has_map_registration = map_callback.is_some();
        let map_callback = map_callback.map(SendWrapper::new);

        let has_layer_callbacks = on_layer_event.is_some()
            || on_layer_click.is_some()
            || on_layer_double_click.is_some()
            || on_layer_context_menu.is_some()
            || on_layer_mouse_enter.is_some()
            || on_layer_mouse_move.is_some()
            || on_layer_mouse_leave.is_some();
        let layer_callback = if layer_id.is_some() && has_layer_callbacks {
            Some(Closure::wrap(Box::new(move |payload: JsValue| {
                let Some(event) = crate::js::parse_layer_event_payload(payload) else {
                    return;
                };

                if let Some(on_layer_event) = on_layer_event.as_ref() {
                    on_layer_event.run(event.clone());
                }

                match event.kind {
                    LayerEventKind::Click => {
                        if let Some(on_layer_click) = on_layer_click.as_ref() {
                            on_layer_click.run(event);
                        }
                    }
                    LayerEventKind::DoubleClick => {
                        if let Some(on_layer_double_click) = on_layer_double_click.as_ref() {
                            on_layer_double_click.run(event);
                        }
                    }
                    LayerEventKind::ContextMenu => {
                        if let Some(on_layer_context_menu) = on_layer_context_menu.as_ref() {
                            on_layer_context_menu.run(event);
                        }
                    }
                    LayerEventKind::MouseEnter => {
                        if let Some(on_layer_mouse_enter) = on_layer_mouse_enter.as_ref() {
                            on_layer_mouse_enter.run(event);
                        }
                    }
                    LayerEventKind::MouseMove => {
                        if let Some(on_layer_mouse_move) = on_layer_mouse_move.as_ref() {
                            on_layer_mouse_move.run(event);
                        }
                    }
                    LayerEventKind::MouseLeave => {
                        if let Some(on_layer_mouse_leave) = on_layer_mouse_leave.as_ref() {
                            on_layer_mouse_leave.run(event);
                        }
                    }
                }
            }) as Box<dyn FnMut(_)>))
        } else {
            None
        };
        if let (Some(layer_id), Some(layer_callback)) = (layer_id.as_deref(), layer_callback.as_ref()) {
            crate::js::register_on_layer_events(handle, layer_id, layer_callback);
        }
        let has_layer_registration = layer_callback.is_some();
        let layer_callback = layer_callback.map(SendWrapper::new);

        on_cleanup(move || {
            if has_map_registration {
                crate::js::unregister_on_map_events(handle);
            }
            if has_layer_registration {
                if let Some(layer_id) = layer_id.as_deref() {
                    crate::js::unregister_on_layer_events(handle, layer_id);
                }
            }
            if let Some(map_callback) = map_callback {
                let _ = map_callback.take();
            }
            if let Some(layer_callback) = layer_callback {
                let _ = layer_callback.take();
            }
        });
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = handle;
        let _ = layer_id;
        let _ = on_event;
        let _ = on_move;
        let _ = on_zoom;
        let _ = on_idle;
        let _ = on_style_data;
        let _ = on_data;
        let _ = on_layer_event;
        let _ = on_layer_click;
        let _ = on_layer_double_click;
        let _ = on_layer_context_menu;
        let _ = on_layer_mouse_enter;
        let _ = on_layer_mouse_move;
        let _ = on_layer_mouse_leave;
    }

    view! { <></> }
}
