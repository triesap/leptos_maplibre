use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct MapHandle(pub u32);

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MapControlAnchor {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct NativeControlOptions {
    pub navigation: Option<MapControlAnchor>,
    pub scale: Option<MapControlAnchor>,
    pub fullscreen: Option<MapControlAnchor>,
    pub geolocate: Option<MapControlAnchor>,
    pub attribution: Option<MapControlAnchor>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct MapInitOptions {
    pub style_url: String,
    pub center_lng: f64,
    pub center_lat: f64,
    pub zoom: f64,
    pub min_zoom: Option<f64>,
    pub max_zoom: Option<f64>,
    pub min_pitch: Option<f64>,
    pub max_pitch: Option<f64>,
    pub bounds: Option<[f64; 4]>,
    pub max_bounds: Option<[f64; 4]>,
    pub pitch: Option<f64>,
    pub bearing: Option<f64>,
    pub bearing_snap: Option<f64>,
    pub projection: Option<String>,
    pub render_world_copies: Option<bool>,
    pub drag_pan: Option<bool>,
    pub drag_rotate: Option<bool>,
    pub pitch_with_rotate: Option<bool>,
    pub zoom_on_double_click: Option<bool>,
    pub cooperative_gestures: Option<bool>,
    pub preserve_drawing_buffer: Option<bool>,
    pub around_center: Option<bool>,
    pub interactive: Option<bool>,
    pub attribution_control: Option<bool>,
    pub antialias: Option<bool>,
    pub native_controls: Option<NativeControlOptions>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct FeatureHit {
    pub layer_id: String,
    pub properties: serde_json::Value,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct MapClickEvent {
    pub lng: f64,
    pub lat: f64,
    pub screen_x: f64,
    pub screen_y: f64,
    pub features: Vec<FeatureHit>,
}

#[cfg(test)]
mod tests {
    use super::{
        FeatureHit, MapClickEvent, MapControlAnchor, MapInitOptions, NativeControlOptions,
    };
    use serde_json::json;

    #[test]
    fn map_init_options_roundtrip() {
        let options = MapInitOptions {
            style_url: "https://demotiles.maplibre.org/style.json".to_string(),
            center_lng: 15.0,
            center_lat: 60.0,
            zoom: 6.0,
            min_zoom: Some(2.0),
            max_zoom: Some(18.0),
            min_pitch: Some(0.0),
            max_pitch: Some(60.0),
            bounds: Some([-179.0, -80.0, 179.0, 80.0]),
            max_bounds: Some([-160.0, -70.0, 160.0, 70.0]),
            pitch: Some(35.0),
            bearing: Some(15.0),
            bearing_snap: Some(7.0),
            projection: Some("mercator".to_string()),
            render_world_copies: Some(true),
            drag_pan: Some(true),
            drag_rotate: Some(true),
            pitch_with_rotate: Some(true),
            zoom_on_double_click: Some(true),
            cooperative_gestures: Some(false),
            preserve_drawing_buffer: Some(false),
            around_center: Some(false),
            interactive: Some(true),
            attribution_control: Some(false),
            antialias: Some(true),
            native_controls: Some(NativeControlOptions {
                navigation: Some(MapControlAnchor::TopRight),
                scale: Some(MapControlAnchor::BottomLeft),
                fullscreen: None,
                geolocate: None,
                attribution: Some(MapControlAnchor::BottomRight),
            }),
        };

        let encoded = serde_json::to_string(&options).expect("serialize map options");
        let decoded: MapInitOptions =
            serde_json::from_str(&encoded).expect("deserialize map options");
        assert_eq!(decoded, options);
    }

    #[test]
    fn map_click_event_roundtrip() {
        let event = MapClickEvent {
            lng: 11.2,
            lat: 58.9,
            screen_x: 256.0,
            screen_y: 384.0,
            features: vec![FeatureHit {
                layer_id: "lots-fill".to_string(),
                properties: json!({"lot":"SE-123"}),
            }],
        };

        let encoded = serde_json::to_string(&event).expect("serialize click");
        let decoded: MapClickEvent = serde_json::from_str(&encoded).expect("deserialize click");
        assert_eq!(decoded, event);
    }

    #[test]
    fn native_controls_default_hidden() {
        let options = MapInitOptions {
            style_url: "https://demotiles.maplibre.org/style.json".to_string(),
            center_lng: 0.0,
            center_lat: 0.0,
            zoom: 1.0,
            min_zoom: None,
            max_zoom: None,
            min_pitch: None,
            max_pitch: None,
            bounds: None,
            max_bounds: None,
            pitch: None,
            bearing: None,
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
            antialias: Some(false),
            native_controls: None,
        };

        assert!(options.native_controls.is_none());
    }
}
