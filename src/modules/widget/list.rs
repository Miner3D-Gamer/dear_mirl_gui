use crate::{
    Any, Buffer, CursorStyle, KeyCode, DearMirlGuiItem, MouseData, render,
};
pub struct Selection {
    /// If this is empty and search_box is enabled, this can be used as a text box
    pub suggested: Vec<String>,
    // The currently selected suggestion -> Is usize::MAX when search string matches no suggested string
    pub selected: usize,
    /// What the user is currently searching
    pub search_string: String,
    /// Future idea: allow for different matching algorithms
    pub search_box: bool,
    /// If custom selections are allowed, anything can be written in the textbox
    pub allow_custom_selection: bool,
}
