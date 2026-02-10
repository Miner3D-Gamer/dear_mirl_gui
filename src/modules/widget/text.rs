use mirl::{prelude::Buffer, render};

use crate::{DearMirlGuiModule, module_manager::InsertionMode};
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Clone, PartialEq)]
/// A simple text module
pub struct TextDisplay {
    /// The text it contains
    pub text: String,
    //#[allow(missing_docs)]
    //pub height: f32,
    // /// The color used
    // pub color: u32,
    #[allow(missing_docs)]
    pub needs_redraw: std::cell::Cell<bool>,
}
impl TextDisplay {
    #[allow(missing_docs)]
    #[must_use]
    pub fn new(
        text: &str,
        //height: usize,
        //color: Option<u32>
    ) -> Self {
        Self {
            text: text.to_string(),
            //height: height as f32,
            //color: color.unwrap_or(mirl::graphics::colors::WHITE),
            needs_redraw: std::cell::Cell::new(true),
        }
    }
    /// Set the text the module is displaying (tip: Use [`format!()`](format!) for easier formatting)
    pub fn set_text(&mut self, text: String) {
        if text != self.text {
            self.text = text;
            self.needs_redraw.set(true);
        }
    }
}

impl DearMirlGuiModule for TextDisplay {
    fn apply_new_formatting(&mut self, _formatting: &crate::Formatting) {}
    fn set_need_redraw(&mut self, need_redraw: Vec<(usize, bool)>) {
        self.needs_redraw.set(super::misc::determine_need_redraw(need_redraw));
    }
    fn draw(
        &mut self,
        formatting: &crate::Formatting,
        _info: &crate::ModuleDrawInfo,
    ) -> (Buffer, InsertionMode) {
        let width = mirl::render::get_text_width(
            &self.text,
            formatting.height as f32,
            &formatting.font,
        ) as usize;
        let height = mirl::render::get_text_height(
            &self.text,
            formatting.height as f32,
            &formatting.font,
        );
        let mut buffer = Buffer::new_empty((width, height as usize));
        render::draw_text_antialiased::<true>(
            &mut buffer,
            &self.text,
            (0, 0),
            formatting.text_color,
            formatting.height as f32,
            &formatting.font,
        );
        (buffer, InsertionMode::ReplaceAll)
    }
    fn get_height(
        &mut self,
        formatting: &crate::Formatting,
    ) -> crate::DearMirlGuiCoordinateType {
        mirl::render::get_text_height(
            &self.text,
            formatting.height as f32,
            &formatting.font,
        ) as crate::DearMirlGuiCoordinateType
    }
    fn get_width(
        &mut self,
        formatting: &crate::Formatting,
    ) -> crate::DearMirlGuiCoordinateType {
        mirl::render::get_text_width(
            &self.text,
            formatting.height as f32,
            &formatting.font,
        ) as crate::DearMirlGuiCoordinateType
    }
    fn update(&mut self, _info: &crate::ModuleUpdateInfo) -> crate::GuiOutput {
        crate::GuiOutput::empty()
    }

    fn need_redraw(&mut self) -> bool {
        if self.needs_redraw.get() {
            self.needs_redraw.set(false);
            true
        } else {
            false
        }
    }
}
