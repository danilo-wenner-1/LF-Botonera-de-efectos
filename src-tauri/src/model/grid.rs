/// Modulo: types_grid.rs
/// Proposito: contrato de vista que Rust entrega para pintar la grilla.
use crate::model::ButtonData;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ButtonView {
    #[serde(flatten)]
    pub data: ButtonData,
    #[serde(default)]
    pub timer_label_key: String,
    #[serde(default)]
    pub type_icon: String,
    #[serde(default)]
    pub can_prelisten: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GridState {
    pub columns: u32,
    pub rows: u32,
    /// Tamaño de texto predeterminado de la pestaña (px). 0 = el global.
    #[serde(default)]
    pub font_size: u32,
    pub buttons: Vec<ButtonView>,
}
