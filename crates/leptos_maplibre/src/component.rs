use leptos::prelude::*;

use crate::types::{MapClickEvent, MapHandle, MapInitOptions};

#[component]
pub fn MapView(
    options: MapInitOptions,
    #[prop(optional, into)] class: Option<String>,
    #[prop(optional, into)] style: Option<String>,
    #[prop(optional)] on_ready: Option<Callback<MapHandle>>,
    #[prop(optional)] on_click: Option<Callback<MapClickEvent>>,
) -> impl IntoView {
    let _ = options;
    let _ = on_ready;
    let _ = on_click;

    let style = style.unwrap_or_else(|| "width:100%; height:100%; min-height:400px;".to_string());

    view! {
        <div class=class style=style></div>
    }
}
