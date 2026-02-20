use leptos::prelude::*;
use leptos_maplibre::MapHandle;

use crate::events::PopupLifecycleEvent;
#[cfg(target_arch = "wasm32")]
use crate::events::PopupLifecycleEventKind;

#[component]
pub fn Popup(
    handle: MapHandle,
    #[prop(into)] lng: Signal<f64>,
    #[prop(into)] lat: Signal<f64>,
    #[prop(into)] html: Signal<String>,
    #[prop(optional)] close_button: bool,
    #[prop(optional)] close_on_click: bool,
    #[prop(optional, into)] anchor: Option<String>,
    #[prop(optional)] offset_x: Option<f64>,
    #[prop(optional)] offset_y: Option<f64>,
    #[prop(optional, into)] max_width: Option<Signal<f64>>,
    #[prop(optional)] on_open: Option<Callback<PopupLifecycleEvent>>,
    #[prop(optional)] on_close: Option<Callback<PopupLifecycleEvent>>,
) -> impl IntoView {
    #[cfg(target_arch = "wasm32")]
    {
        use send_wrapper::SendWrapper;
        use std::sync::Arc;
        use wasm_bindgen::JsValue;
        use wasm_bindgen::closure::Closure;

        let popup_handle = RwSignal::new(None::<u32>);
        let anchor = anchor.clone();

        let has_popup_callbacks = on_open.is_some() || on_close.is_some();
        let popup_event_callback = if has_popup_callbacks {
            Some(Arc::new(SendWrapper::new(Closure::wrap(
                Box::new(move |payload: JsValue| {
                    let Some(event) = crate::js::parse_popup_lifecycle_event_payload(payload)
                    else {
                        return;
                    };
                    match event.kind {
                        PopupLifecycleEventKind::Open => {
                            if let Some(on_open) = on_open.as_ref() {
                                on_open.run(event);
                            }
                        }
                        PopupLifecycleEventKind::Close => {
                            if let Some(on_close) = on_close.as_ref() {
                                on_close.run(event);
                            }
                        }
                    }
                }) as Box<dyn FnMut(_)>,
            ))))
        } else {
            None
        };
        let popup_event_callback_for_effect = popup_event_callback.clone();
        let popup_event_callback_for_cleanup = popup_event_callback.clone();

        Effect::new(move |_| {
            let lng = lng.get();
            let lat = lat.get();
            let html = html.get();
            let max_width = max_width.as_ref().map(|max_width| max_width.get());
            match popup_handle.get() {
                Some(popup_handle) => {
                    crate::js::update_popup(
                        popup_handle,
                        lng,
                        lat,
                        &html,
                        offset_x,
                        offset_y,
                        max_width,
                    );
                }
                None => {
                    if let Some(next_handle) = crate::js::create_popup(
                        handle,
                        lng,
                        lat,
                        &html,
                        close_button,
                        close_on_click,
                        anchor.as_deref(),
                        offset_x,
                        offset_y,
                        max_width,
                    ) {
                        if let Some(popup_event_callback) = popup_event_callback_for_effect.as_ref()
                        {
                            crate::js::register_on_popup_events(
                                next_handle,
                                popup_event_callback.as_ref(),
                            );
                        }
                        popup_handle.set(Some(next_handle));
                    }
                }
            }
        });

        on_cleanup(move || {
            if let Some(popup_handle) = popup_handle.get_untracked() {
                crate::js::unregister_on_popup_events(popup_handle);
                crate::js::remove_popup(popup_handle);
            }
            drop(popup_event_callback_for_cleanup);
        });
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = handle;
        let _ = lng;
        let _ = lat;
        let _ = html;
        let _ = close_button;
        let _ = close_on_click;
        let _ = anchor;
        let _ = offset_x;
        let _ = offset_y;
        let _ = max_width;
        let _ = on_open;
        let _ = on_close;
    }

    view! { <></> }
}
