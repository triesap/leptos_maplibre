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
    let node_ref = NodeRef::<leptos::html::Div>::new();

    let style = style.unwrap_or_else(|| "width:100%; height:100%; min-height:400px;".to_string());

    #[cfg(target_arch = "wasm32")]
    {
        use send_wrapper::SendWrapper;
        use wasm_bindgen::closure::Closure;
        use wasm_bindgen::{JsCast, JsValue};

        let node_ref = node_ref;
        let options = options.clone();
        let on_ready = on_ready.clone();
        let on_click = on_click.clone();

        node_ref.on_load(move |root| {
            let Ok(container) = root.dyn_into::<web_sys::HtmlElement>() else {
                return;
            };
            let Some(handle) = crate::js::init_map(&container, &options) else {
                return;
            };

            let click_closure = on_click.as_ref().map(|on_click| {
                let on_click = on_click.clone();
                Closure::wrap(Box::new(move |payload: JsValue| {
                    if let Some(event) = crate::js::parse_click_payload(payload) {
                        on_click.run(event);
                    }
                }) as Box<dyn FnMut(_)>)
            });

            if let Some(click_closure) = click_closure.as_ref() {
                crate::js::register_on_click(handle, click_closure);
            }

            let ready_closure = on_ready.as_ref().map(|on_ready| {
                let on_ready = on_ready.clone();
                Closure::wrap(Box::new(move || {
                    on_ready.run(handle);
                }) as Box<dyn FnMut()>)
            });

            if let Some(ready_closure) = ready_closure.as_ref() {
                crate::js::register_on_load(handle, ready_closure);
            }

            let click_closure = click_closure.map(SendWrapper::new);
            let ready_closure = ready_closure.map(SendWrapper::new);

            on_cleanup(move || {
                crate::js::unregister_on_click(handle);
                crate::js::unregister_on_load(handle);
                if let Some(click_closure) = click_closure {
                    let _ = click_closure.take();
                }
                if let Some(ready_closure) = ready_closure {
                    let _ = ready_closure.take();
                }
                crate::js::destroy_map(handle);
            });
        });
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = options;
        let _ = on_ready;
        let _ = on_click;
    }

    view! {
        <div node_ref=node_ref class=class style=style></div>
    }
}
