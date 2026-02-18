use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PopupProps {
    pub close_button: bool,
    pub close_on_click: bool,
}
