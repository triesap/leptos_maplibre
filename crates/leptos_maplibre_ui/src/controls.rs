use leptos::prelude::*;
use leptos_maplibre::{MapControlAnchor, MapHandle};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NativeControlKind {
    Navigation,
    Scale,
    Fullscreen,
    Geolocate,
    Attribution,
}

impl NativeControlKind {
    #[cfg(target_arch = "wasm32")]
    fn as_str(self) -> &'static str {
        match self {
            NativeControlKind::Navigation => "navigation",
            NativeControlKind::Scale => "scale",
            NativeControlKind::Fullscreen => "fullscreen",
            NativeControlKind::Geolocate => "geolocate",
            NativeControlKind::Attribution => "attribution",
        }
    }
}

#[component]
pub fn NativeControl(
    handle: MapHandle,
    kind: NativeControlKind,
    anchor: Option<MapControlAnchor>,
    options: Option<serde_json::Value>,
) -> impl IntoView {
    #[cfg(target_arch = "wasm32")]
    {
        let control_handle = RwSignal::new(None::<u32>);
        let options = options.clone();

        Effect::new(move |_| {
            if control_handle.get().is_none() {
                if let Some(next_handle) =
                    crate::js::add_native_control(handle, kind.as_str(), anchor, options.as_ref())
                {
                    control_handle.set(Some(next_handle));
                }
            }
        });

        on_cleanup(move || {
            if let Some(control_handle) = control_handle.get_untracked() {
                crate::js::remove_native_control(control_handle);
            }
        });
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = handle;
        let _ = kind;
        let _ = anchor;
        let _ = options;
    }

    view! { <></> }
}

#[component]
pub fn NavigationControl(
    handle: MapHandle,
    anchor: Option<MapControlAnchor>,
    options: Option<serde_json::Value>,
) -> impl IntoView {
    view! { <NativeControl handle=handle kind=NativeControlKind::Navigation anchor=anchor options=options /> }
}

#[component]
pub fn ScaleControl(
    handle: MapHandle,
    anchor: Option<MapControlAnchor>,
    options: Option<serde_json::Value>,
) -> impl IntoView {
    view! { <NativeControl handle=handle kind=NativeControlKind::Scale anchor=anchor options=options /> }
}

#[component]
pub fn FullscreenControl(
    handle: MapHandle,
    anchor: Option<MapControlAnchor>,
    options: Option<serde_json::Value>,
) -> impl IntoView {
    view! { <NativeControl handle=handle kind=NativeControlKind::Fullscreen anchor=anchor options=options /> }
}

#[component]
pub fn GeolocateControl(
    handle: MapHandle,
    anchor: Option<MapControlAnchor>,
    options: Option<serde_json::Value>,
) -> impl IntoView {
    view! { <NativeControl handle=handle kind=NativeControlKind::Geolocate anchor=anchor options=options /> }
}

#[component]
pub fn AttributionControl(
    handle: MapHandle,
    anchor: Option<MapControlAnchor>,
    options: Option<serde_json::Value>,
) -> impl IntoView {
    view! { <NativeControl handle=handle kind=NativeControlKind::Attribution anchor=anchor options=options /> }
}
