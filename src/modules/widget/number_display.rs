use crate::{Buffer, DearMirlGuiModule, InsertionMode, render};

/// The number types allowed in `NumberDisplay`
pub trait NumberDisplayNumberType:
    std::fmt::Debug + std::marker::Send + 'static + std::string::ToString
{
}
impl<T: std::fmt::Debug + std::marker::Send + 'static + std::string::ToString>
    NumberDisplayNumberType for T
{
}

#[derive(Debug, Clone, PartialEq)]
/// A simple text module
pub struct NumberDisplay<NumberDisplayNumberType> {
    /// The text it contains
    pub number: NumberDisplayNumberType,
    /// How many numbers should be displayed
    pub slots: usize,
    #[allow(missing_docs)]
    pub height: f32,
    #[allow(missing_docs)]
    pub needs_redraw: std::cell::Cell<bool>,
}
impl<T: NumberDisplayNumberType> NumberDisplay<T> {
    #[allow(missing_docs)]
    #[must_use]
    pub const fn new(number: T, slots: usize, height: f32) -> Self {
        Self {
            number,
            height,
            slots,
            needs_redraw: std::cell::Cell::new(true),
        }
    }
}

impl<T: NumberDisplayNumberType> DearMirlGuiModule for NumberDisplay<T> {
    fn apply_new_formatting(&mut self, _formatting: &crate::Formatting) {}
    fn set_need_redraw(&mut self, need_redraw: Vec<(usize, bool)>) {
        self.needs_redraw.set(super::misc::determine_need_redraw(need_redraw));
    }
    fn draw(
        &mut self,
        formatting: &crate::Formatting,
        _info: &crate::ModuleDrawInfo,
    ) -> (Buffer, InsertionMode) {
        let text_color = mirl::graphics::colors::WHITE;
        let mut buffer = Buffer::new_empty_with_color(
            (
                self.get_width(formatting) as usize,
                self.get_height(formatting) as usize,
            ),
            formatting.foreground_color,
        );
        let text_height = self.height - (formatting.vertical_margin * 2) as f32;
        let text_width = self.height / 2.0;

        let mut text = self.number.to_string();
        let mut skip = self.slots as isize - text.chars().count() as isize;

        if skip < 0 {
            text = text.chars().skip(skip.unsigned_abs()).collect();
            skip = 0;
        }
        let mut offset = skip as f32 * text_width;
        for char in text.chars() {
            render::draw_text_antialiased::<true>(
                &mut buffer,
                &char.to_string(),
                (offset as usize, 0),
                text_color,
                text_height,
                &formatting.font,
            );
            offset += text_width;
        }
        (buffer, InsertionMode::ReplaceAll)
    }
    fn get_height(
        &mut self,
        _formatting: &crate::Formatting,
    ) -> crate::DearMirlGuiCoordinateType {
        self.height as crate::DearMirlGuiCoordinateType
    }
    fn get_width(
        &mut self,
        _formatting: &crate::Formatting,
    ) -> crate::DearMirlGuiCoordinateType {
        (self.height as crate::DearMirlGuiCoordinateType / 2)
            * self.slots as crate::DearMirlGuiCoordinateType
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
