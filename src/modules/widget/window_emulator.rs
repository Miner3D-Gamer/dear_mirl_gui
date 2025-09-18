use mirl::extensions::*;

use crate::{Buffer, DearMirlGuiModule, gui::Directions};

#[derive(Debug, Clone)]
/// A wrapper between an internal gui accessed with the .gui field and the module system of another window
pub struct WindowEmulator<const FAST: bool, const USE_CACHE: bool> {
    /// The internal window
    pub gui: std::cell::RefCell<crate::gui::DearMirlGui<FAST, USE_CACHE>>,
}

impl<const FAST: bool, const USE_CACHE: bool> WindowEmulator<FAST, USE_CACHE> {
    #[allow(missing_docs)]
    #[must_use]
    pub fn new(gui: crate::gui::DearMirlGui<FAST, USE_CACHE>) -> Self {
        let mut gui = gui;

        gui.allow_dragging = false;

        let mut directions = Directions::all_false();
        directions.bottom = true;
        directions.right = true;
        directions.bottom_right = true;
        gui.resizing_allowed_in_directions = directions;

        Self {
            gui: gui.into(),
        }
    }
}

impl<const FAST: bool, const USE_CACHE: bool> DearMirlGuiModule
    for WindowEmulator<FAST, USE_CACHE>
{
    fn apply_new_formatting(&mut self, _formatting: &crate::Formatting) {}
    fn get_height(&self, _formatting: &crate::Formatting) -> isize {
        self.gui.borrow().get_height() as isize
    }
    fn get_width(&self, _formatting: &crate::Formatting) -> isize {
        self.gui.borrow().get_width() as isize
    }
    fn update(&mut self, info: &crate::ModuleInputs) -> crate::GuiOutput {
        let window_offset = {
            let gui = self.gui.borrow();
            (gui.x, gui.y)
        };
        //println!("{:?}, {:?}", window_offset, self.gui.borrow().last_mouse_pos);
        self.gui.borrow_mut().update(
            info.mouse_pos.map(|pos| pos.add(window_offset.tuple_2_into())),
            info.mouse_scroll,
            info.mouse_info.left.down,
            info.mouse_info.middle.down,
            info.mouse_info.right.down,
            info.pressed_keys,
            info.delta_time,
            info.clipboard_data,
        )
    }
    fn need_redraw(&self) -> bool {
        true
    }
    fn draw(&self, _formatting: &crate::Formatting) -> Buffer {
        self.gui.borrow_mut().draw()
    }
}
