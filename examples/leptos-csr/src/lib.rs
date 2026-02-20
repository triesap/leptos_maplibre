use leptos::prelude::*;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <main class="app_shell">
            <section class="panel map_panel">
                <h1>"leptos_maplibre csr demo"</h1>
                <p>"loading demo app"</p>
                <div class="map_shell"></div>
            </section>
        </main>
    }
}
