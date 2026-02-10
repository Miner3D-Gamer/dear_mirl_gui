use mirl::{
    extensions::*,
    math::ConstZero,
    platform::{keycodes::KeyCode, mouse::MouseSnapShot},
    render::Buffer,
};

use crate::{
    DearMirlGui, DearMirlGuiModule, FocusTaken, GuiOutput, ModuleUpdateInfo,
};

#[derive(Debug, Clone)]
/// Handling a single window is easy but dealing with even just two can get annoying. Let this fella help ya
pub struct DearMirlGuiManager<const FAST: bool, const USE_CACHE: bool> {
    /// The locally stored windows sorted by z layering
    pub windows: Vec<DearMirlGui<FAST, USE_CACHE>>,
    /// If the left mouse button was pressed last frame
    pub last_left_mouse_down: bool,
    /// If the middle mouse button was pressed last frame
    pub last_middle_mouse_down: bool,
    /// If the right mouse button was pressed last frame
    pub last_right_mouse_down: bool,
    /// The last known mouse position
    pub last_mouse_pos: (f32, f32),
    /// If any window needs to be redrawn
    pub needs_redraw: bool,
}
impl<const FAST: bool, const USE_CACHE: bool>
    DearMirlGuiManager<FAST, USE_CACHE>
{
    /// Create a new window manager
    #[must_use]
    pub const fn new(windows: Vec<DearMirlGui<FAST, USE_CACHE>>) -> Self {
        Self {
            windows,
            last_left_mouse_down: false,
            last_middle_mouse_down: false,
            last_right_mouse_down: false,
            last_mouse_pos: (0.0, 0.0),
            needs_redraw: true,
        }
    }
    /// Update all windows inside
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::too_many_lines)] // Well, clippy... it's just big. :(
    pub fn update(
        &mut self,
        mouse_snapshot: &MouseSnapShot,
        pressed_keys: &Vec<KeyCode>,
        delta_time: f64,
        clipboard_data: &Option<mirl::platform::file_system::FileData>,
    ) -> GuiOutput {
        let mouse_data = mouse_snapshot.to_mouse_button_state(
            self.last_left_mouse_down,
            self.last_middle_mouse_down,
            self.last_right_mouse_down,
        );
        let mouse_pos_delta = mouse_snapshot
            .position
            .unwrap_or((0.0, 0.0))
            .sub(self.last_mouse_pos);

        {
            self.last_mouse_pos = mouse_snapshot.position.unwrap_or_default();
            self.last_left_mouse_down = mouse_snapshot.left_down;
            self.last_right_mouse_down = mouse_snapshot.right_down;
            self.last_middle_mouse_down = mouse_snapshot.middle_down;

            let inputs = ModuleUpdateInfo {
                mouse_pos: mouse_snapshot.position,
                real_mouse_pos: mouse_snapshot.position,
                mouse_info: mouse_data,
                focus_taken: FocusTaken::FocusFree,
                mouse_pos_delta,
                mouse_scroll: mouse_snapshot.scroll,
                pressed_keys,
                delta_time,
                clipboard_data,
                container_id: 0,
            };
            self.update_raw(&inputs)
        }
    }
    /// Update all windows inside
    #[must_use]
    pub fn update_raw(
        &mut self,
        inputs: &crate::ModuleUpdateInfo,
    ) -> GuiOutput {
        let mut output = GuiOutput::empty();
        let mouse_data = inputs.mouse_info;

        {
            if let Some(pos) = inputs.mouse_pos {
                self.last_mouse_pos = pos;
            }
            self.last_left_mouse_down = mouse_data.left.down;
            self.last_right_mouse_down = mouse_data.right.down;
            self.last_middle_mouse_down = mouse_data.middle.down;

            let mut input = *inputs;
            let mut to_switch = None;
            for (idx, i) in self.windows.iter_mut().enumerate() {
                input.container_id = i.id;
                let o = i.update_using_module_data(input, &output);
                self.needs_redraw |= i.need_redraw();

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
            // TODO: Is there any way of skipping overdraw on overlapping windows?
            i.draw_on_buffer(buffer);
        }
        self.needs_redraw = false;
    }
    fn get_bounds(
        &self,
    ) -> (
        crate::DearMirlGuiCoordinateType,
        crate::DearMirlGuiCoordinateType,
        crate::DearMirlGuiCoordinateType,
        crate::DearMirlGuiCoordinateType,
    ) {
        let mut top = crate::DearMirlGuiCoordinateType::ZERO;
        let mut right = crate::DearMirlGuiCoordinateType::ZERO;
        let mut left = crate::DearMirlGuiCoordinateType::ZERO;
        let mut bottom = crate::DearMirlGuiCoordinateType::ZERO;
        for i in &self.windows {
            if i.x < left {
                left = i.x;
            }
            let this_right =
                i.x + i.get_width() as crate::DearMirlGuiCoordinateType;
            if this_right > right {
                right = this_right;
            }
            if i.y < top {
                top = i.y;
            }
            let this_bottom =
                i.y + i.get_height() as crate::DearMirlGuiCoordinateType;
            if this_bottom > bottom {
                bottom = this_bottom;
            }
        }
        (top, left, right, bottom)
    }
    fn get_size_to_see_all_windows(
        &self,
    ) -> (crate::DearMirlGuiCoordinateType, crate::DearMirlGuiCoordinateType)
    {
        let bounds = self.get_bounds();
        let height = bounds.3 - bounds.0;
        let width = bounds.2 - bounds.1;
        (width, height)
    }
}

impl<const FAST: bool, const USE_CACHE: bool> DearMirlGuiModule
    for DearMirlGuiManager<FAST, USE_CACHE>
{
    fn draw(
        &mut self,
        _formatting: &crate::Formatting,
        _info: &crate::ModuleDrawInfo,
    ) -> (Buffer, crate::module_manager::InsertionMode) {
        let Some(size) = self.get_size_to_see_all_windows().try_tuple_into()
        else {
            return (
                Buffer::generate_fallback((32, 32), 2),
                crate::module_manager::InsertionMode::ReplaceAll,
            );
        };
        let mut buffer = Buffer::new_empty(size);
        self.draw_on_buffer(&mut buffer);
        (buffer, crate::module_manager::InsertionMode::ReplaceAll)
    }

    fn get_height(
        &mut self,
        _formatting: &crate::Formatting,
    ) -> crate::DearMirlGuiCoordinateType {
        self.get_size_to_see_all_windows().1
    }

    fn get_width(
        &mut self,
        _formatting: &crate::Formatting,
    ) -> crate::DearMirlGuiCoordinateType {
        self.get_size_to_see_all_windows().0
    }

    fn update(&mut self, inputs: &crate::ModuleUpdateInfo) -> crate::GuiOutput {
        self.update_raw(inputs)
    }

    fn need_redraw(&mut self) -> bool {
        self.needs_redraw
    }
}
