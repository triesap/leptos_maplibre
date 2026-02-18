use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MapEventKind {
    Move,
    Zoom,
    Idle,
    StyleData,
    Data,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LayerEventKind {
    Click,
    DoubleClick,
    ContextMenu,
    MouseEnter,
    MouseMove,
    MouseLeave,
}
