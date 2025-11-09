use mirl::{Buffer, extensions::*, platform::keycodes::KeyCode};

use crate::{
    ButtonState, DearMirlGui, FocusTaken, GuiOutput, ModuleUpdateInfo,
    MouseData,
};

#[derive(Debug, Clone)]
/// Handling a single window is easy but dealing with even just two can get annoying. Let this fella help ya
pub struct WindowManager<const FAST: bool, const USE_CACHE: bool> {
    /// The locally stored windows sorted by z layering
    pub windows: Vec<DearMirlGui<FAST, USE_CACHE>>,
    /// If the left mouse button was pressed last frame
    pub last_left_mouse_down: bool,
    /// If the middle mouse button was pressed last frame
    pub last_middle_mouse_down: bool,
    /// If the right mouse button was pressed last frame
    pub last_right_mouse_down: bool,
    /// The last known mouse position
    pub last_mouse_pos: (i32, i32),
}
impl<const FAST: bool, const USE_CACHE: bool> WindowManager<FAST, USE_CACHE> {
    /// Create a new window manager
    #[must_use]
    pub const fn new(windows: Vec<DearMirlGui<FAST, USE_CACHE>>) -> Self {
        Self {
            windows,
            last_left_mouse_down: false,
            last_middle_mouse_down: false,
            last_right_mouse_down: false,
            last_mouse_pos: (0, 0),
        }
    }
    /// Update all windows inside
    ///
    /// # Panics
    /// When
    #[must_use]
    #[allow(clippy::panic)]
    #[allow(clippy::too_many_arguments, clippy::too_many_lines)] // Well, clippy... it's just big. :(
    pub fn update(
        &mut self,
        mouse_pos: Option<(i32, i32)>,
        mouse_scroll: Option<(f32, f32)>,
        left_mouse_down: bool,
        middle_mouse_down: bool,
        right_mouse_down: bool,
        pressed_keys: &Vec<KeyCode>,
        delta_time: f64,
        clipboard_data: &Option<mirl::platform::file_system::FileData>,
    ) -> GuiOutput {
        let mut output = GuiOutput::empty();
        let mouse_data = MouseData {
            left: ButtonState::new(left_mouse_down, self.last_left_mouse_down),
            middle: ButtonState::new(
                middle_mouse_down,
                self.last_middle_mouse_down,
            ),
            right: ButtonState::new(
                right_mouse_down,
                self.last_right_mouse_down,
            ),
        };
        let mouse_pos_delta =
            mouse_pos.unwrap_or((0, 0)).sub(self.last_mouse_pos);

        {
            self.last_mouse_pos = mouse_pos.unwrap_or_default();
            self.last_left_mouse_down = left_mouse_down;
            self.last_right_mouse_down = right_mouse_down;
            self.last_middle_mouse_down = middle_mouse_down;

            let mut input = ModuleUpdateInfo {
                mouse_pos,
                real_mouse_pos: mouse_pos,
                mouse_info: &mouse_data,
                focus_taken: FocusTaken::FocusFree,
                mouse_pos_delta,
                mouse_scroll,
                pressed_keys,
                delta_time,
                clipboard_data,
                container_id: 0,
            };
            let mut to_switch = None;
            for (idx, i) in self.windows.iter_mut().enumerate() {
                input.container_id = i.id;
                let o = i.update_using_module_data(input, &output);

                input.focus_taken |= o.focus_taken;
                if o.focus_taken.is_focus_taken() {
                    //println!("Output focus: {:?} ({}) ", o.focus_taken, i.title);
                    if o.focus_taken == FocusTaken::FunctionallyTaken
                        && to_switch.is_none()
                    {
                        #[cfg(feature = "focus_debug")]
                        if idx != 0 {
                            println!("Window has took focus: {}", i.title,);
                        }
                        to_switch = Some(idx);
                    }
                }
                output |= o;
            }
            if let Some(idx) = to_switch {
                self.windows.swap(0, idx);
            }
            output
        }
    }
    /// Set the size of all windows to see all inners
    pub fn set_size_to_see_all_modules(&mut self) {
        for i in &mut self.windows {
            i.set_size_to_see_all_modules();
        }
    }
    /// Draw all windows on the buffer
    pub fn draw_on_buffer(&mut self, buffer: &mut Buffer) {
        for i in self.windows.iter_mut().rev() {
            i.draw_on_buffer(buffer);
        }
    }
}
