use mirl::prelude::Buffer;

use crate::{DearMirlGuiModule, module_manager::InsertionMode};
#[derive(Debug, Clone, PartialEq, Eq)]
/// Define a custom offset for the next modules
pub struct Image {
    /// The buffer it contains
    pub image: Buffer,
    #[allow(missing_docs)]
    pub needs_redraw: bool,
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
            needs_redraw: (true),
            width: size.0,
            height: size.1,
            resizing_method: resizing_method.unwrap_or_default(),
        }
    }
}

impl DearMirlGuiModule for Image {
    fn apply_new_formatting(&mut self, _formatting: &crate::Formatting) {}
    fn get_height(
        &mut self,
        _formatting: &crate::Formatting,
    ) -> crate::DearMirlGuiCoordinateType {
        self.height as crate::DearMirlGuiCoordinateType
    }
    fn set_need_redraw(&mut self, need_redraw: Vec<(usize, bool)>) {
        self.needs_redraw = super::misc::determine_need_redraw(need_redraw) ;
    }
    fn get_width(
        &mut self,
        _formatting: &crate::Formatting,
    ) -> crate::DearMirlGuiCoordinateType {
        self.width as crate::DearMirlGuiCoordinateType
    }
    fn update(&mut self, _info: &crate::ModuleUpdateInfo) -> crate::GuiOutput {
        crate::GuiOutput::empty()
    }
    fn need_redraw(&mut self) -> bool {
        self.needs_redraw
        // if self.needs_redraw.get() {
        //     self.needs_redraw.set(false);
        //     true
        // } else {
        //     false
        // }
    }
    fn draw(
        &mut self,
        _formatting: &crate::Formatting,
        _info: &crate::ModuleDrawInfo,
    ) -> (Buffer, InsertionMode) {
        self.needs_redraw = false ;
        (
            if self.width == self.image.width
                && self.height == self.image.height
            {
                self.image.clone()
            } else {
                self.image.resize_content(
                    (self.width, self.height),
                    self.resizing_method,
                )
            },
            InsertionMode::Simple,
        )
    }
}
