use crate::{
    Any, Buffer, CursorStyle, KeyCode, DearMirlGuiItem, MouseData, render,
};
pub struct ColorPicker {
    pub width: usize,
    pub height: usize,
    pub original: u32,
    pub current: u32,
    pub selected: u8,
}
