use leptos::prelude::*;
use leptos_maplibre::MapHandle;

#[component]
pub fn Marker(
    handle: MapHandle,
    #[prop(into)] lng: Signal<f64>,
    #[prop(into)] lat: Signal<f64>,
    #[prop(optional)] draggable: bool,
) -> impl IntoView {
    #[cfg(target_arch = "wasm32")]
    {
        let marker_handle = RwSignal::new(None::<u32>);

        Effect::new(move |_| {
            let lng = lng.get();
            let lat = lat.get();
            match marker_handle.get() {
                Some(marker_handle) => {
                    crate::js::update_marker(marker_handle, lng, lat, draggable);
                }
                None => {
                    if let Some(next_handle) = crate::js::create_marker(handle, lng, lat, draggable) {
                        marker_handle.set(Some(next_handle));
                    }
                }
            }
        });

        on_cleanup(move || {
            if let Some(marker_handle) = marker_handle.get_untracked() {
                crate::js::remove_marker(marker_handle);
            }
        });
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = handle;
        let _ = lng;
        let _ = lat;
        let _ = draggable;
    }

    view! { <></> }
}
