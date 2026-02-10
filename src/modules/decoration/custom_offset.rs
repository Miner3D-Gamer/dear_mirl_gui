use crate::{DearMirlGuiModule, module_manager::InsertionMode};
use mirl::prelude::Buffer;

type FunctionModifyCursor = fn(
    &[crate::gui::extra::ModuleContainer],
    &Vec<usize>,
    &crate::Formatting,
    (
        &mut crate::DearMirlGuiCoordinateType,
        &mut crate::DearMirlGuiCoordinateType,
    ),
);

#[derive(Debug, Clone, Copy)]
/// Define a custom offset for the next modules
pub struct CustomOffset {
    /// Take control over the draw/update cursor position yourself
    pub function: FunctionModifyCursor,
}

impl CustomOffset {
    #[allow(missing_docs)]
    #[must_use]
    pub const fn new(function: FunctionModifyCursor) -> Self {
        Self {
            function,
        }
    }
}

impl DearMirlGuiModule for CustomOffset {
    fn apply_new_formatting(&mut self, _formatting: &crate::Formatting) {}
    fn get_height(
        &mut self,
        _formatting: &crate::Formatting,
    ) -> crate::DearMirlGuiCoordinateType {
        mirl::math::ConstNumbers128::CONST_0
    }
    fn get_width(
        &mut self,
        _formatting: &crate::Formatting,
    ) -> crate::DearMirlGuiCoordinateType {
        mirl::math::ConstNumbers128::CONST_0
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
        modules: &[crate::gui::extra::ModuleContainer],
        used_idx: &Vec<usize>,
        formatting: &crate::Formatting,
        current: (
            &mut crate::DearMirlGuiCoordinateType,
            &mut crate::DearMirlGuiCoordinateType,
        ),
    ) {
        (self.function)(modules, used_idx, formatting, current);
    }
}
