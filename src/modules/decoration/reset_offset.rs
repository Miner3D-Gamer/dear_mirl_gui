//use mirl::extensions::*;

use crate::{Buffer, DearMirlGuiModule, InsertionMode};

#[derive(Debug, Clone, PartialEq, Eq)]
/// Reset the doings of all previous modules that have defined an offset
///
/// TODO: This needs a rework to properly include the Y coordinate while still respecting the Y offsets of other modules!
pub struct ResetOffset {
    offset: std::cell::Cell<(isize, isize)>,
}

impl ResetOffset {
    #[allow(missing_docs)]
    #[must_use]
    pub const fn new() -> Self {
        Self {
            offset: std::cell::Cell::new((0, 0)),
        }
    }
}
impl Default for ResetOffset {
    fn default() -> Self {
        Self::new()
    }
}

impl DearMirlGuiModule for ResetOffset {
    fn apply_new_formatting(&mut self, _formatting: &crate::Formatting) {}
    fn get_height(
        &mut self,
        _formatting: &crate::Formatting,
    ) -> crate::DearMirlGuiCoordinateType {
        self.offset.get().1 as crate::DearMirlGuiCoordinateType
    }
    fn set_need_redraw(&mut self, _need_redraw: Vec<(usize, bool)>) {}
    fn get_width(
        &mut self,
        _formatting: &crate::Formatting,
    ) -> crate::DearMirlGuiCoordinateType {
        self.offset.get().0 as crate::DearMirlGuiCoordinateType
    }
    fn update(&mut self, _info: &crate::ModuleUpdateInfo) -> crate::GuiOutput {
        crate::GuiOutput::empty()
    }
    fn need_redraw(&mut self) -> bool {
        false
    }
    fn draw(
        &mut self,
        _formatting: &crate::Formatting,
        _info: &crate::ModuleDrawInfo,
    ) -> (Buffer, InsertionMode) {
        (Buffer::new_empty((0, 0)), InsertionMode::Simple)
    }
    fn modify_offset_cursor(
        &mut self,
        _modules: &[crate::gui::extra::ModuleContainer],
        _used_idx: &Vec<usize>,
        _formatting: &crate::Formatting,
        current: (
            &mut crate::DearMirlGuiCoordinateType,
            &mut crate::DearMirlGuiCoordinateType,
        ),
    ) {
        // let here = used_idx.last().copied().unwrap_or_default();
        // if here == 0 {
        //     return;
        // }
        *current.0 = 0;
    }
}
