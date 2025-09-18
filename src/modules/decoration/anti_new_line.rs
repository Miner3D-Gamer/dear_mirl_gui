use crate::{Buffer, DearMirlGuiModule};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Make the next module reside next to the previous
pub struct SameLine {
    width: isize,
}

impl SameLine {
    #[allow(missing_docs)]
    #[must_use]
    pub const fn new(margin: isize) -> Self {
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
    fn get_height(&self, _formatting: &crate::Formatting) -> isize {
        0
    }
    fn get_width(&self, _formatting: &crate::Formatting) -> isize {
        self.width
    }
    fn update(
        &mut self,
        _info: &crate::ModuleInputs,
    ) -> crate::GuiOutput {
        crate::GuiOutput::default(false)
    }
    fn need_redraw(&self) -> bool {
        false
    }
    fn draw(&self, _formatting: &crate::Formatting) -> Buffer {
        Buffer::new_empty(0, 0)
    }
    fn get_next_offset(
        &self,
        modules: &indexmap::IndexMap<String, crate::gui::ModuleContainer>,
        current_idx: usize,
        formatting: &crate::Formatting,
    ) -> (isize, isize) {
        if current_idx == 0 {
            return (0, 0);
        }
        let previous_module = &modules[current_idx - 1];
        (
            self.width + previous_module.get_width(formatting),
            -(previous_module.get_height(formatting)
                + formatting.vertical_margin as isize * 2),
        )
    }
}
