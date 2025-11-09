use crate::{Buffer, DearMirlGuiModule, InsertionMode};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Make the next module reside next to the previous
pub struct SameLine {
    width: crate::DearMirlGuiCoordinateType,
}

impl SameLine {
    #[allow(missing_docs)]
    #[must_use]
    pub const fn new(margin: crate::DearMirlGuiCoordinateType) -> Self {
        Self {
            width: margin,
        }
    }
}
impl Default for SameLine {
    fn default() -> Self {
        Self::new(0)
    }
}

impl DearMirlGuiModule for SameLine {
    fn apply_new_formatting(&mut self, _formatting: &crate::Formatting) {}
    fn get_height(
        &mut self,
        _formatting: &crate::Formatting,
    ) -> crate::DearMirlGuiCoordinateType {
        0
    }
    fn get_width(
        &mut self,
        _formatting: &crate::Formatting,
    ) -> crate::DearMirlGuiCoordinateType {
        self.width
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
        let here = *used_idx.last().unwrap_or(&0);
        if here < 2 {
            return;
        }
        let Some(previous_idx) = used_idx.get(used_idx.len() - 2) else {
            return;
        };
        let previous_module = &modules[*previous_idx];
        *current.0 += self.width + previous_module.get_width(formatting);
        *current.1 += -(previous_module.get_height(formatting)
            + formatting.vertical_margin as crate::DearMirlGuiCoordinateType
                * 2);
    }
}
