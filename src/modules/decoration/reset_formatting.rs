use mirl::extensions::*;

use crate::{Buffer, DearMirlGuiModule};

#[derive(Debug, Clone, PartialEq, Eq)]
/// Reset the doings of all previous modules that have defined an offset
///
/// TODO: This needs a rework to properly include the Y coordinate while still respecting the Y offsets of other modules!
pub struct ResetFormatting {
    offset: std::cell::Cell<(isize, isize)>,
}

impl ResetFormatting {
    #[allow(missing_docs)]
    #[must_use]
    pub fn new() -> Self {
        Self {
            offset: (0, 0).into(),
        }
    }
}
impl Default for ResetFormatting {
    fn default() -> Self {
        Self::new()
    }
}

impl DearMirlGuiModule for ResetFormatting {
    fn apply_new_formatting(&mut self, _formatting: &crate::Formatting) {}
    fn get_height(&self, _formatting: &crate::Formatting) -> isize {
        self.offset.get().1
    }
    fn get_width(&self, _formatting: &crate::Formatting) -> isize {
        self.offset.get().0
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
        let mut total_offset = (0, 0);
        for i in 0..current_idx {
            if let Some((_, module)) = modules.get_index(i) {
                let offset =
                    module.get_next_offset(modules, current_idx, formatting);
                // Uhh, that's not quite right.
                total_offset = total_offset.sub((offset.0, 0));
            }
        }
        self.offset.set(total_offset);

        total_offset
    }
}
