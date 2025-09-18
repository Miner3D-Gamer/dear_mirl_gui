use crate::{Buffer, DearMirlGuiModule};

#[derive(Debug, Clone, PartialEq, Eq)]
/// Define a custom offset for the next modules
pub struct CustomOffset {
    /// Offset X
    pub width: isize,
    /// Offset Y
    pub height: isize,
    /// If the module should be redrawn
    pub needs_redraw: std::cell::Cell<bool>,
}

impl CustomOffset {
    #[allow(missing_docs)]
    #[must_use]
    pub fn new(offset_x: isize, offset_y: isize) -> Self {
        Self {
            width: offset_x,
            height: offset_y,
            needs_redraw: true.into(),
        }
    }
}

impl DearMirlGuiModule for CustomOffset {
    fn apply_new_formatting(&mut self, _formatting: &crate::Formatting) {}
    fn get_height(&self, _formatting: &crate::Formatting) -> isize {
        self.height
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
        if self.needs_redraw.get() {
            self.needs_redraw.set(false);
            true
        } else {
            false
        }
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
            -(previous_module.get_height(formatting)),
        )
    }
}
