use mirl::platform::keycodes::KeyCode;

use crate::{FocusTaken, MouseState};

#[derive(Debug, Clone, Copy)]
/// A struct holding all information the modules are provided with
pub struct ModuleUpdateInfo<'a> {
    /// If a previous module already took focus
    pub focus_taken: FocusTaken,
    /// The current mouse position
    pub mouse_pos: Option<(i32, i32)>,
    /// The current mouse position on the screen, unlocalized
    pub real_mouse_pos: Option<(i32, i32)>,
    /// The mouse position since last frame - is (0, 0) when `mouse_pos` is None
    pub mouse_pos_delta: (i32, i32),
    /// The mouse scroll distance, (x, y)
    pub mouse_scroll: Option<(f32, f32)>,
    /// Info on what mouse buttons have been pressed
    pub mouse_info: &'a MouseState,
    /// All pressed keys
    pub pressed_keys: &'a Vec<KeyCode>,
    /// Delta time, what else to say about it?
    pub delta_time: f64,
    /// Clipboard data must be requested first
    pub clipboard_data: &'a Option<mirl::platform::file_system::FileData>,
    /// Closest container ID
    pub container_id: usize,
}
#[derive(Debug, Clone, PartialEq, Eq)]
/// The return further info that just [`Option::None`]/[`Option::Some`]
pub enum GuiReturnsModule<T: 'static> {
    /// All went well and you can use the module no problem
    AllGood(T),
    /// There was no module with this ID
    UnableToFindID(u32, String),
    /// There was an module with this ID, however the used type was not correct
    CastingAsWrongModule {
        /// The correct type the module should be casted as
        correct: String,
        /// The incorrect type the module was requested to be casted into
        wrong: String,
        /// The id of the module
        id: u32,
    },
    /// For when the unexpected happens
    Misc(String),
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
/// Other info that may be useful
pub struct ModuleDrawInfo {
    /// The id of the closest parent container
    pub container_id: usize,
}
