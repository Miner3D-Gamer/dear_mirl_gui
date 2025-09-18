use crate::{Buffer, DearMirlGuiModule};

#[derive(Debug, Clone, PartialEq, Eq)]
/// Define a custom offset for the next modules
pub struct Image {
    /// The buffer it contains
    pub image: Buffer,
    #[allow(missing_docs)]
    pub needs_redraw: std::cell::Cell<bool>,
}

impl Image {
    #[allow(missing_docs)]
    #[must_use]
    pub fn new(image: Buffer) -> Self {
        Self {
            image,
            needs_redraw: true.into(),
        }
    }
}

impl DearMirlGuiModule for Image {
    fn apply_new_formatting(&mut self, _formatting: &crate::Formatting) {}
    fn get_height(&self, _formatting: &crate::Formatting) -> isize {
        self.image.height as isize
    }
    fn get_width(&self, _formatting: &crate::Formatting) -> isize {
        self.image.width as isize
    }
    fn update(&mut self, _info: &crate::ModuleInputs) -> crate::GuiOutput {
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
        self.image.clone()
    }
}
