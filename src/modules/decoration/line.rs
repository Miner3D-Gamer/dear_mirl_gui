use crate::{Buffer, DearMirlGuiModule, render};

#[derive(Debug, Clone, PartialEq, Eq)]
/// Create a visual margin between modules
pub struct Separator {
    height: usize,
    width: usize,
    thickness: usize,
    is_vertical: bool,
    needs_redraw: std::cell::Cell<bool>,
}

impl Separator {
    const DEFAULT_LINE_THICKNESS: usize = 5;
    #[allow(missing_docs)]
    #[must_use]
    pub fn new(
        height: usize,
        width: usize,
        is_vertical: bool,
        line_thickness: Option<usize>,
    ) -> Self {
        Self {
            height,
            width,
            thickness: line_thickness.unwrap_or(Self::DEFAULT_LINE_THICKNESS),
            is_vertical,
            needs_redraw: true.into(),
        }
    }
}

impl DearMirlGuiModule for Separator {
    fn apply_new_formatting(&mut self, _formatting: &crate::Formatting) {}
    fn get_height(&self, _formatting: &crate::Formatting) -> isize {
        self.height as isize
    }
    fn get_width(&self, _formatting: &crate::Formatting) -> isize {
        self.width as isize
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
    fn draw(&self, formatting: &crate::Formatting) -> Buffer {
        let buffer = Buffer::new_empty(self.width, self.height);
        if self.is_vertical {
            let x = self.width / 2 - self.thickness / 2;
            render::draw_line_straight(
                &buffer,
                (x, 0),
                self.height,
                true,
                formatting.secondary_color,
                self.thickness as isize,
                false,
            );
        } else {
            let y = self.height / 2 - self.thickness / 2;
            render::draw_line_straight(
                &buffer,
                (0, y),
                self.width,
                false,
                formatting.secondary_color,
                self.thickness as isize,
                true,
            );
        }
        buffer
    }
}
