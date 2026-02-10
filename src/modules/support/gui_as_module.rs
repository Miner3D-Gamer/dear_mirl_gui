use mirl::{
    directions::NormalDirections,
    math::{SetOne, SetZero},
    prelude::Buffer,
};

use crate::{DearMirlGui, DearMirlGuiModule, module_manager::InsertionMode};
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
        self.needs_redraw =
            crate::modules::misc::determine_need_redraw(need_redraw);
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
        self.update_using_module_data(*info, &crate::GuiOutput::empty())
    }
    fn need_redraw(&mut self) -> bool {
        self.needs_redraw
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
        self.x.set_zero();
        self.y.set_one();

        let mut directions = NormalDirections::all_false();
        directions.bottom = true;
        directions.right = true;
        directions.bottom_right = true;
        self.resizing_allowed_in_directions = directions;
    }
}
