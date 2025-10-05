use crate::{Buffer, DearMirlGuiModule, InsertionMode, render};

#[derive(Debug, Clone, PartialEq)]
/// A simple text module
pub struct Text {
    /// The text it contains
    pub text: String,
    #[allow(missing_docs)]
    pub height: f32,
    /// The color used
    pub color: u32,
    #[allow(missing_docs)]
    pub needs_redraw: std::cell::Cell<bool>,
}
impl Text {
    #[allow(missing_docs)]
    #[must_use]
    pub fn new(text: &str, height: usize, color: Option<u32>) -> Self {
        Self {
            text: text.to_string(),
            height: height as f32,
            color: color.unwrap_or(mirl::graphics::color_presets::WHITE),
            needs_redraw: std::cell::Cell::new(true),
        }
    }
    /// Set the text the module is displaying (tip: [`format!()`](format!) exists)
    pub fn set_text(&mut self, text: String) {
        self.text = text;
        self.needs_redraw = std::cell::Cell::new(true);
    }
}

impl DearMirlGuiModule for Text {
    fn apply_new_formatting(&mut self, _formatting: &crate::Formatting) {}
    fn set_need_redraw(&self, need_redraw: Vec<(usize, bool)>) {
        self.needs_redraw.set(super::misc::determine_need_redraw(need_redraw));
    }
    fn draw(
        &mut self,
        formatting: &crate::Formatting,
        _info: &crate::ModuleDrawInfo,
    ) -> (Buffer, InsertionMode) {
        let width = mirl::render::get_text_width(
            &self.text,
            self.height,
            &formatting.font,
        ) as usize;
        let height = mirl::render::get_text_height(
            &self.text,
            self.height,
            &formatting.font,
        );
        let buffer = Buffer::new_empty(width, height as usize);
        render::draw_text_antialiased::<true>(
            &buffer,
            &self.text,
            (0, 0),
            self.color,
            self.height,
            &formatting.font,
        );
        (buffer, InsertionMode::ReplaceAll)
    }
    fn get_height(&self, formatting: &crate::Formatting) -> isize {
        mirl::render::get_text_height(&self.text, self.height, &formatting.font)
            as isize
    }
    fn get_width(&self, formatting: &crate::Formatting) -> isize {
        mirl::render::get_text_width(&self.text, self.height, &formatting.font)
            as isize
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
}
