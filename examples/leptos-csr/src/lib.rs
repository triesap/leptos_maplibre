use leptos::prelude::*;
use leptos_maplibre::{
    MapClickEvent, MapHandle, MapInitOptions, MapView, add_geojson_source, add_layer,
    set_feature_state,
};
use leptos_maplibre_ui::{LayerEvent, MapEvents, Marker, Popup};
use serde_json::json;

const TOFINO_SOURCE_ID: &str = "tofino_demo_source";
const TOFINO_AREA_LAYER_ID: &str = "tofino_area_fill";
const TOFINO_ROUTE_LAYER_ID: &str = "tofino_route_line";
const TOFINO_POI_LAYER_ID: &str = "tofino_poi_circle";
const TOFINO_LABEL_LAYER_ID: &str = "tofino_labels";

#[component]
pub fn App() -> impl IntoView {
    let status = RwSignal::new(String::from("tofino map waiting for map load"));
    let map_handle = RwSignal::new(None::<MapHandle>);
    let marker_lng = RwSignal::new(-126.2622);
    let marker_lat = RwSignal::new(49.3579);
    let popup_html = RwSignal::new(String::from(
        "<strong>Hot Springs Cove</strong><br/>click a map feature",
    ));
    let selected_feature_id = RwSignal::new(None::<serde_json::Value>);

    let options = MapInitOptions {
        style_url: String::from("https://demotiles.maplibre.org/style.json"),
        center_lng: -126.08,
        center_lat: 49.26,
        zoom: 9.0,
        min_zoom: Some(7.0),
        max_zoom: Some(15.0),
        min_pitch: None,
        max_pitch: None,
        bounds: Some([-126.30, 49.12, -125.85, 49.38]),
        max_bounds: None,
        pitch: Some(12.0),
        bearing: Some(-12.0),
        bearing_snap: None,
        projection: None,
        render_world_copies: None,
        drag_pan: None,
        drag_rotate: None,
        pitch_with_rotate: None,
        zoom_on_double_click: None,
        cooperative_gestures: None,
        preserve_drawing_buffer: None,
        around_center: None,
        interactive: Some(true),
        attribution_control: Some(false),
        antialias: Some(true),
        native_controls: None,
    };

    let on_ready = {
        let status = status;
        let map_handle = map_handle;
        Callback::new(move |handle: MapHandle| {
            add_geojson_source(handle, TOFINO_SOURCE_ID, &tofino_geojson(), Some("id"));
            add_layer(
                handle,
                TOFINO_AREA_LAYER_ID,
                &tofino_area_fill_layer_spec(),
                None,
            );
            add_layer(
                handle,
                TOFINO_ROUTE_LAYER_ID,
                &tofino_route_line_layer_spec(),
                None,
            );
            add_layer(
                handle,
                TOFINO_POI_LAYER_ID,
                &tofino_poi_circle_layer_spec(),
                None,
            );
            add_layer(
                handle,
                TOFINO_LABEL_LAYER_ID,
                &tofino_label_layer_spec(),
                None,
            );
            map_handle.set(Some(handle));
            status.set(format!(
                "tofino map ready: handle {}, source and layers mounted",
                handle.0
            ));
        })
    };

    let on_click = {
        let status = status;
        let marker_lng = marker_lng;
        let marker_lat = marker_lat;
        let popup_html = popup_html;
        Callback::new(move |event: MapClickEvent| {
            marker_lng.set(event.lng);
            marker_lat.set(event.lat);
            popup_html.set(format!(
                "<strong>map click</strong><br/>lng {:.4}, lat {:.4}<br/>features {}",
                event.lng,
                event.lat,
                event.features.len()
            ));
            status.set(format!(
                "map click: lng {:.4}, lat {:.4}, features {}",
                event.lng,
                event.lat,
                event.features.len()
            ));
        })
    };

    let on_layer_click = {
        let status = status;
        let map_handle = map_handle;
        let selected_feature_id = selected_feature_id;
        let marker_lng = marker_lng;
        let marker_lat = marker_lat;
        let popup_html = popup_html;
        Callback::new(move |event: LayerEvent| {
            marker_lng.set(event.lng);
            marker_lat.set(event.lat);

            let feature_label = event
                .features
                .first()
                .and_then(|feature| feature.properties.get("label"))
                .and_then(serde_json::Value::as_str)
                .unwrap_or("feature");
            let feature_category = event
                .features
                .first()
                .and_then(|feature| feature.properties.get("category"))
                .and_then(serde_json::Value::as_str)
                .unwrap_or("unknown");

            if let Some(handle) = map_handle.get_untracked() {
                if let Some(previous_id) = selected_feature_id.get_untracked() {
                    set_feature_state(
                        handle,
                        TOFINO_SOURCE_ID,
                        None,
                        &previous_id,
                        &json!({"selected": false}),
                    );
                }
                if let Some(next_id) = event
                    .features
                    .first()
                    .and_then(|feature| feature.properties.get("id"))
                    .cloned()
                {
                    set_feature_state(
                        handle,
                        TOFINO_SOURCE_ID,
                        None,
                        &next_id,
                        &json!({"selected": true}),
                    );
                    selected_feature_id.set(Some(next_id));
                } else {
                    selected_feature_id.set(None);
                }
            }

            popup_html.set(format!(
                "<strong>{}</strong><br/>{}<br/>lng {:.4}, lat {:.4}",
                feature_label, feature_category, event.lng, event.lat
            ));
            status.set(format!(
                "selected {} ({}) at lng {:.4}, lat {:.4}",
                feature_label, feature_category, event.lng, event.lat
            ));
        })
    };

    view! {
        <main class="app_shell">
            <section class="panel map_panel">
                <h1>"leptos_maplibre csr demo"</h1>
                <p>{move || status.get()}</p>
                <div class="map_shell">
                    <MapView options=options on_ready=on_ready on_click=on_click />
                    {move || {
                        map_handle.get().map(|handle| {
                            view! {
                                <MapEvents
                                    handle=handle
                                    layer_id=String::from(TOFINO_POI_LAYER_ID)
                                    on_layer_click=on_layer_click
                                />
                                <Marker handle=handle lng=marker_lng lat=marker_lat draggable=true />
                                <Popup
                                    handle=handle
                                    lng=marker_lng
                                    lat=marker_lat
                                    html=popup_html
                                    close_button=true
                                    close_on_click=false
                                />
                            }
                        })
                    }}
                </div>
            </section>
        </main>
    }
}

fn tofino_geojson() -> serde_json::Value {
    json!({
        "type": "FeatureCollection",
        "features": [
            {
                "type": "Feature",
                "properties": {
                    "id": "tofino-harbour",
                    "kind": "poi",
                    "label": "Tofino Harbour",
                    "category": "town"
                },
                "geometry": { "type": "Point", "coordinates": [-125.9135, 49.1510] }
            },
            {
                "type": "Feature",
                "properties": {
                    "id": "hsc-landing",
                    "kind": "poi",
                    "label": "Maquinna Marine Provincial Park Landing",
                    "category": "landing"
                },
                "geometry": { "type": "Point", "coordinates": [-126.2588, 49.3558] }
            },
            {
                "type": "Feature",
                "properties": {
                    "id": "hsc-boardwalk",
                    "kind": "poi",
                    "label": "Boardwalk Trailhead",
                    "category": "trailhead"
                },
                "geometry": { "type": "Point", "coordinates": [-126.2607, 49.3567] }
            },
            {
                "type": "Feature",
                "properties": {
                    "id": "hsc-pools",
                    "kind": "poi",
                    "label": "Hot Springs Pools",
                    "category": "spring"
                },
                "geometry": { "type": "Point", "coordinates": [-126.2622, 49.3579] }
            },
            {
                "type": "Feature",
                "properties": {
                    "id": "boat-route",
                    "kind": "route",
                    "label": "Boat Route to Hot Springs Cove",
                    "category": "route"
                },
                "geometry": {
                    "type": "LineString",
                    "coordinates": [
                        [-125.9135, 49.1510],
                        [-126.0200, 49.2000],
                        [-126.1200, 49.2600],
                        [-126.2000, 49.3100],
                        [-126.2588, 49.3558]
                    ]
                }
            },
            {
                "type": "Feature",
                "properties": {
                    "id": "hsc-area",
                    "kind": "area",
                    "label": "Hot Springs Cove",
                    "category": "zone"
                },
                "geometry": {
                    "type": "Polygon",
                    "coordinates": [[
                        [-126.2700, 49.3480],
                        [-126.2460, 49.3480],
                        [-126.2460, 49.3650],
                        [-126.2700, 49.3650],
                        [-126.2700, 49.3480]
                    ]]
                }
            }
        ]
    })
}

fn tofino_area_fill_layer_spec() -> serde_json::Value {
    json!({
        "id": TOFINO_AREA_LAYER_ID,
        "type": "fill",
        "source": TOFINO_SOURCE_ID,
        "filter": ["==", ["get", "kind"], "area"],
        "paint": {
            "fill-color": ["case", ["boolean", ["feature-state", "selected"], false], "#f97316", "#0ea5e9"],
            "fill-opacity": 0.2
        }
    })
}

fn tofino_route_line_layer_spec() -> serde_json::Value {
    json!({
        "id": TOFINO_ROUTE_LAYER_ID,
        "type": "line",
        "source": TOFINO_SOURCE_ID,
        "filter": ["==", ["get", "kind"], "route"],
        "paint": {
            "line-color": "#2563eb",
            "line-width": 2.5,
            "line-dasharray": [2.0, 1.5]
        }
    })
}

fn tofino_poi_circle_layer_spec() -> serde_json::Value {
    json!({
        "id": TOFINO_POI_LAYER_ID,
        "type": "circle",
        "source": TOFINO_SOURCE_ID,
        "filter": ["==", ["get", "kind"], "poi"],
        "paint": {
            "circle-radius": ["case", ["boolean", ["feature-state", "selected"], false], 9.0, 6.0],
            "circle-color": ["case", ["boolean", ["feature-state", "selected"], false], "#f97316", "#2563eb"],
            "circle-stroke-width": 1.0,
            "circle-stroke-color": "#ffffff"
        }
    })
}

fn tofino_label_layer_spec() -> serde_json::Value {
    json!({
        "id": TOFINO_LABEL_LAYER_ID,
        "type": "symbol",
        "source": TOFINO_SOURCE_ID,
        "layout": {
            "text-field": ["get", "label"],
            "text-size": 12.0,
            "text-offset": [0.0, 1.2],
            "text-anchor": "top"
        },
        "paint": {
            "text-color": "#0f172a",
            "text-halo-color": "#ffffff",
            "text-halo-width": 1.2
        }
    })
}
