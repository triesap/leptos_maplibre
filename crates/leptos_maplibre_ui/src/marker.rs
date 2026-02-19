use leptos::prelude::*;
use leptos_maplibre::MapHandle;

use crate::events::MarkerDragEvent;
#[cfg(target_arch = "wasm32")]
use crate::events::MarkerDragEventKind;

#[component]
pub fn Marker(
    handle: MapHandle,
    #[prop(into)] lng: Signal<f64>,
    #[prop(into)] lat: Signal<f64>,
    #[prop(optional)] draggable: bool,
    #[prop(optional, into)] anchor: Option<String>,
    #[prop(optional)] offset_x: Option<f64>,
    #[prop(optional)] offset_y: Option<f64>,
    #[prop(optional, into)] rotation: Option<Signal<f64>>,
    #[prop(optional)] on_drag_start: Option<Callback<MarkerDragEvent>>,
    #[prop(optional)] on_drag: Option<Callback<MarkerDragEvent>>,
    #[prop(optional)] on_drag_end: Option<Callback<MarkerDragEvent>>,
) -> impl IntoView {
    #[cfg(target_arch = "wasm32")]
    {
        use std::rc::Rc;
        use wasm_bindgen::closure::Closure;
        use wasm_bindgen::JsValue;

        let marker_handle = RwSignal::new(None::<u32>);
        let anchor = anchor.clone();

        let has_drag_callbacks =
            on_drag_start.is_some() || on_drag.is_some() || on_drag_end.is_some();
        let marker_drag_callback = if has_drag_callbacks {
            Some(Rc::new(Closure::wrap(Box::new(move |payload: JsValue| {
                let Some(event) = crate::js::parse_marker_drag_event_payload(payload) else {
                    return;
                };

                match event.kind {
                    MarkerDragEventKind::DragStart => {
                        if let Some(on_drag_start) = on_drag_start.as_ref() {
                            on_drag_start.run(event);
                        }
                    }
                    MarkerDragEventKind::Drag => {
                        if let Some(on_drag) = on_drag.as_ref() {
                            on_drag.run(event);
                        }
                    }
                    MarkerDragEventKind::DragEnd => {
                        if let Some(on_drag_end) = on_drag_end.as_ref() {
                            on_drag_end.run(event);
                        }
                    }
                }
            }) as Box<dyn FnMut(_)>)))
        } else {
            None
        };
        let marker_drag_callback_for_effect = marker_drag_callback.clone();
        let marker_drag_callback_for_cleanup = marker_drag_callback.clone();

        Effect::new(move |_| {
            let lng = lng.get();
            let lat = lat.get();
            let rotation = rotation.as_ref().map(|rotation| rotation.get());
            match marker_handle.get() {
                Some(marker_handle) => {
                    crate::js::update_marker(
                        marker_handle,
                        lng,
                        lat,
                        draggable,
                        anchor.as_deref(),
                        offset_x,
                        offset_y,
                        rotation,
                    );
                }
                None => {
                    if let Some(next_handle) = crate::js::create_marker(
                        handle,
                        lng,
                        lat,
                        draggable,
                        anchor.as_deref(),
                        offset_x,
                        offset_y,
                        rotation,
                    ) {
                        if let Some(marker_drag_callback) = marker_drag_callback_for_effect.as_ref() {
                            crate::js::register_on_marker_drag_events(next_handle, marker_drag_callback);
                        }
                        marker_handle.set(Some(next_handle));
                    }
                }
            }
        });

        on_cleanup(move || {
            if let Some(marker_handle) = marker_handle.get_untracked() {
                crate::js::unregister_on_marker_drag_events(marker_handle);
                crate::js::remove_marker(marker_handle);
            }
            drop(marker_drag_callback_for_cleanup);
        });
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = handle;
        let _ = lng;
        let _ = lat;
        let _ = draggable;
        let _ = anchor;
        let _ = offset_x;
        let _ = offset_y;
        let _ = rotation;
        let _ = on_drag_start;
        let _ = on_drag;
        let _ = on_drag_end;
    }

    view! { <></> }
}
