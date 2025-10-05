use crate::{Buffer, DearMirlGuiModule, InsertionMode};

#[derive(Debug, Clone, PartialEq, Eq)]
/// Define a custom offset for the next modules
pub struct Image {
    /// The buffer it contains
    pub image: Buffer,
    #[allow(missing_docs)]
    pub needs_redraw: std::cell::Cell<bool>,
    /// The image will be resized to fit this
    pub width: usize,
    /// The image will be resized to fit this
    pub height: usize,
    /// How the image should be resized
    pub resizing_method: mirl::graphics::InterpolationMode,
}

impl Image {
    #[allow(missing_docs)]
    #[must_use]
    pub fn new(
        image: Buffer,
        size: Option<(usize, usize)>,
        resizing_method: Option<mirl::graphics::InterpolationMode>,
    ) -> Self {
        let size = size.unwrap_or((image.width, image.height));
        Self {
            image,
            needs_redraw: std::cell::Cell::new(true),
            width: size.0,
            height: size.1,
            resizing_method: resizing_method.unwrap_or_default(),
        }
    }
}

impl DearMirlGuiModule for Image {
    fn apply_new_formatting(&mut self, _formatting: &crate::Formatting) {}
    fn get_height(&self, _formatting: &crate::Formatting) -> isize {
        self.height as isize
    }
    fn set_need_redraw(&self, need_redraw: Vec<(usize, bool)>) {
        self.needs_redraw.set(super::misc::determine_need_redraw(need_redraw));
    }
    fn get_width(&self, _formatting: &crate::Formatting) -> isize {
        self.width as isize
    }
    fn update(&mut self, _info: &crate::ModuleUpdateInfo) -> crate::GuiOutput {
        crate::GuiOutput::empty()
    }
    fn need_redraw(&self) -> bool {
        if self.needs_redraw.get() {
            self.needs_redraw.set(false);
            true
        } else {
            false
        }
    }
    fn draw(
        &mut self,
        _formatting: &crate::Formatting,
        _info: &crate::ModuleDrawInfo,
    ) -> (Buffer, InsertionMode) {
        self.needs_redraw.set(false);
        (
            if self.width == self.image.width
                && self.height == self.image.height
            {
                self.image.clone()
            } else {
                self.image.resize_content(
                    self.width,
                    self.height,
                    self.resizing_method,
                )
            },
            InsertionMode::Simple,
        )
    }
}
