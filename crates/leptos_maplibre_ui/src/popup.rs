use leptos::prelude::*;
use leptos_maplibre::MapHandle;

#[component]
pub fn Popup(
    handle: MapHandle,
    #[prop(into)] lng: Signal<f64>,
    #[prop(into)] lat: Signal<f64>,
    #[prop(into)] html: Signal<String>,
    #[prop(optional)] close_button: bool,
    #[prop(optional)] close_on_click: bool,
) -> impl IntoView {
    #[cfg(target_arch = "wasm32")]
    {
        let popup_handle = RwSignal::new(None::<u32>);

        Effect::new(move |_| {
            let lng = lng.get();
            let lat = lat.get();
            let html = html.get();
            match popup_handle.get() {
                Some(popup_handle) => {
                    crate::js::update_popup(popup_handle, lng, lat, &html);
                }
                None => {
                    if let Some(next_handle) =
                        crate::js::create_popup(handle, lng, lat, &html, close_button, close_on_click)
                    {
                        popup_handle.set(Some(next_handle));
                    }
                }
            }
        });

        on_cleanup(move || {
            if let Some(popup_handle) = popup_handle.get_untracked() {
                crate::js::remove_popup(popup_handle);
            }
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
    }

    view! { <></> }
}
