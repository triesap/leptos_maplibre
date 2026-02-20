use leptos::mount::mount_to_body;
use leptos::prelude::*;
use leptos_maplibre_example_csr::App;

fn main() {
    let _ = console_log::init_with_level(log::Level::Info);
    console_error_panic_hook::set_once();
    mount_to_body(|| {
        view! {
            <App/>
        }
    });
}
