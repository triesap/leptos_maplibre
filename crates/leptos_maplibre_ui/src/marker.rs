use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MarkerProps {
    pub lng: f64,
    pub lat: f64,
    pub draggable: bool,
}
