use leptos::prelude::*;
use leptos_maplibre::MapHandle;

use crate::events::MapEvent;

#[component]
pub fn MapEvents(
    handle: MapHandle,
    #[prop(optional)] on_event: Option<Callback<MapEvent>>,
    #[prop(optional)] on_move: Option<Callback<MapEvent>>,
    #[prop(optional)] on_zoom: Option<Callback<MapEvent>>,
    #[prop(optional)] on_idle: Option<Callback<MapEvent>>,
    #[prop(optional)] on_style_data: Option<Callback<MapEvent>>,
    #[prop(optional)] on_data: Option<Callback<MapEvent>>,
) -> impl IntoView {
    #[cfg(target_arch = "wasm32")]
    {
        use send_wrapper::SendWrapper;
        use wasm_bindgen::closure::Closure;
        use wasm_bindgen::JsValue;

        let on_event = on_event.clone();
        let on_move = on_move.clone();
        let on_zoom = on_zoom.clone();
        let on_idle = on_idle.clone();
        let on_style_data = on_style_data.clone();
        let on_data = on_data.clone();

        on_mount(move || {
            let callback = Closure::wrap(Box::new(move |payload: JsValue| {
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
            }) as Box<dyn FnMut(_)>);

            crate::js::register_on_map_events(handle, &callback);
            let callback = SendWrapper::new(callback);

            on_cleanup(move || {
                crate::js::unregister_on_map_events(handle);
                let _ = callback.take();
            });
        });
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = handle;
        let _ = on_event;
        let _ = on_move;
        let _ = on_zoom;
        let _ = on_idle;
        let _ = on_style_data;
        let _ = on_data;
    }

    view! { <></> }
}
