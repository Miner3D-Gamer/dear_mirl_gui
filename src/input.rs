use mirl::platform::{keycodes::KeyCode, mouse::MouseButtonState};

use crate::FocusTaken;

#[derive(Debug, Clone, Copy)]
/// A struct holding all information the modules are provided with
pub struct ModuleUpdateInfo<'a> {
    /// If a previous module already took focus
    pub focus_taken: FocusTaken,
    /// The current mouse position
    pub mouse_pos: Option<(f32, f32)>,
    /// The current mouse position on the screen, unlocalized
    pub real_mouse_pos: Option<(f32, f32)>,
    /// The mouse position since last frame - is (0, 0) when `mouse_pos` is None
    pub mouse_pos_delta: (f32, f32),
    /// The mouse scroll distance, (x, y)
    pub mouse_scroll: (f32, f32),
    /// Info on what mouse buttons have been pressed
    pub mouse_info: MouseButtonState,
    /// All pressed keys
    pub pressed_keys: &'a Vec<KeyCode>,
    /// Delta time, what else to say about it?
    pub delta_time: f64,
    /// Clipboard data must be requested first
    pub clipboard_data: &'a Option<mirl::platform::file_system::FileData>,
    /// Closest container ID
    pub container_id: usize,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
/// Other info that may be useful
pub struct ModuleDrawInfo {
    /// The id of the closest parent container
    pub container_id: usize,
}
