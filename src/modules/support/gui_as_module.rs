use mirl::{extensions::*, misc::NormalDirections};

use crate::{Buffer, DearMirlGui, DearMirlGuiModule, InsertionMode};

// #[derive(Debug, Clone)]
// /// A wrapper between an internal gui accessed with the .gui field and the module system of another window
// pub struct WindowEmulator<const FAST: bool, const USE_CACHE: bool> {
//     /// The internal window
//     pub gui: std::cell::RefCell<crate::gui::DearMirlGui<FAST, USE_CACHE>>,
// }
#[allow(clippy::non_send_fields_in_send_ty)]
unsafe impl<const FAST: bool, const USE_CACHE: bool> std::marker::Send
    for crate::gui::DearMirlGui<FAST, USE_CACHE>
{
}

// impl<const FAST: bool, const USE_CACHE: bool> DearMirlGui<FAST, USE_CACHE> {
//     #[allow(missing_docs)]
//     #[must_use]
//     pub const fn new(gui: crate::gui::DearMirlGui<FAST, USE_CACHE>) -> Self {
//         let mut gui = gui;

//         gui.allow_dragging = false;

//         let mut directions = Directions::all_false();
//         directions.bottom = true;
//         directions.right = true;
//         directions.bottom_right = true;
//         gui.resizing_allowed_in_directions = directions;

//         Self {
//             gui: std::cell::RefCell::new(gui),
//         }
//     }
// }

impl<const FAST: bool, const USE_CACHE: bool> DearMirlGuiModule
    for DearMirlGui<FAST, USE_CACHE>
{
    fn apply_new_formatting(&mut self, _formatting: &crate::Formatting) {}
    fn set_need_redraw(&mut self, need_redraw: Vec<(usize, bool)>) {
        self.needs_redraw
            .set(crate::modules::misc::determine_need_redraw(need_redraw));
    }
    fn get_height(
        &mut self,
        _formatting: &crate::Formatting,
    ) -> crate::DearMirlGuiCoordinateType {
        Self::get_height(self) as crate::DearMirlGuiCoordinateType
    }
    fn get_width(
        &mut self,
        _formatting: &crate::Formatting,
    ) -> crate::DearMirlGuiCoordinateType {
        Self::get_width(self) as crate::DearMirlGuiCoordinateType
    }
    fn update(&mut self, info: &crate::ModuleUpdateInfo) -> crate::GuiOutput {
        let window_offset = { (self.x, self.y) };
        //println!("{:?}, {:?}", window_offset, self.gui.borrow().last_mouse_pos);
        self.update(
            info.mouse_pos.map(|pos| pos.add(window_offset.tuple_into())),
            info.mouse_scroll,
            info.mouse_info.left.down,
            info.mouse_info.middle.down,
            info.mouse_info.right.down,
            info.pressed_keys,
            info.delta_time,
            info.clipboard_data,
        )
    }
    fn need_redraw(&mut self) -> bool {
        true
    }
    fn draw(
        &mut self,
        _formatting: &crate::Formatting,
        _info: &crate::ModuleDrawInfo,
    ) -> (Buffer, InsertionMode) {
        (self.render(), InsertionMode::ReplaceAll)
    }
    fn added(&mut self, _container_id: usize) {
        self.allow_dragging = false;

        let mut directions = NormalDirections::all_false();
        directions.bottom = true;
        directions.right = true;
        directions.bottom_right = true;
        self.resizing_allowed_in_directions = directions;
    }
}
