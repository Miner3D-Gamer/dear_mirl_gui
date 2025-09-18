use crate::{Buffer, DearMirlGuiModule, render};
#[derive(Debug, Clone, PartialEq)]
#[allow(unpredictable_function_pointer_comparisons)]
/// A simple button
pub struct Button {
    /// A function that will be executed every time the button is pressed, not released
    pub function: Option<fn()>,
    /// If the cursor is currently hovering over the button
    pub hovering: bool,
    /// If the button is actively pressed
    pub pressed: bool,
    #[allow(missing_docs)]
    pub width: usize,
    #[allow(missing_docs)]
    pub height: usize,
    /// The text the button contains
    pub text: String,
    #[allow(missing_docs)]
    pub needs_redraw: std::cell::Cell<bool>,
    /// For the text scroll animation
    pub scroll: f64,
    /// By how much the scroll should be scaled
    pub scroll_multiplier: f64,
    /// By how much the color of the button will change when the mouse is hover over the button
    pub color_change_on_hover: f32,
    /// By how much the color of the button will change when the mouse is clicking the button (In addition to the hover color change)
    pub color_change_on_clicking: f32,
    /// At some point you just can't read the text anymore if it is scaled to miniature sizes so instead it'll scroll
    pub threshold_before_text_scrolls: f32,
}
impl Button {
    const DEFAULT_COLOR_CHANGE_ON_HOVER: f32 = -5.0;
    const DEFAULT_COLOR_CHANGE_ON_CLICKING: f32 = -5.0;
    const DEFAULT_THRESHOLD_BEFORE_TEXT_SCROLLS: f32 = 0.6;
    const DEFAULT_SCROLL_MULTIPLIER: f64 = 0.1;
    #[allow(missing_docs)]
    #[must_use]
    pub fn new(
        text: String,
        height: usize,
        width: Option<usize>,
        function: Option<fn()>,
        font: Option<&fontdue::Font>,
    ) -> Self {
        let width_to_height_mul = 3;
        Self {
            function,
            pressed: false,
            hovering: false,
            width: width.unwrap_or_else(|| {
                font.map_or_else(
                    || height * width_to_height_mul,
                    |font| {
                        render::get_text_width(&text, height as f32, font)
                            as usize
                    },
                )
            }),
            height,
            text,
            needs_redraw: true.into(),
            scroll: 0.0,
            scroll_multiplier: Self::DEFAULT_SCROLL_MULTIPLIER,
            color_change_on_hover: Self::DEFAULT_COLOR_CHANGE_ON_HOVER,
            color_change_on_clicking: Self::DEFAULT_COLOR_CHANGE_ON_CLICKING,
            threshold_before_text_scrolls:
                Self::DEFAULT_THRESHOLD_BEFORE_TEXT_SCROLLS,
        }
    }
}

impl DearMirlGuiModule for Button {
    // fn as_any(&self) -> &dyn Any {
    //     self
    // }
    // fn as_any_mut(&mut self) -> &mut dyn Any {
    //     self
    // }
    fn draw(&self, formatting: &crate::Formatting) -> Buffer {
        let text_color = formatting.text_color;

        let mut color = formatting.secondary_color;
        if self.hovering {
            color = mirl::graphics::adjust_brightness_hsl_of_rgb(
                color,
                self.color_change_on_hover,
            );
        }
        if self.pressed {
            color = mirl::graphics::adjust_brightness_hsl_of_rgb(
                color,
                self.color_change_on_clicking,
            );
        }
        let buffer =
            Buffer::new_empty_with_color(self.width, self.height, color);

        let text_height = render::get_text_height(
            &self.text,
            self.height as f32,
            &formatting.font,
        );
        let text_width = render::get_text_width(
            &self.text,
            self.height as f32,
            &formatting.font,
        );

        let bounding_width = self.width - formatting.horizontal_margin * 2;
        let bounding_height = self.height - formatting.vertical_margin * 2;

        let size_mul_x = (text_width as usize / bounding_width) as f32;
        let size_mul_y = text_height / bounding_height as f32;

        let size_mul = 1.0 / size_mul_y.max(size_mul_x);

        let adjusted_width = text_width * size_mul;
        let adjusted_height = text_height * size_mul;

        // If the text fits within all if good
        if self.threshold_before_text_scrolls < size_mul {
            let pos = mirl::math::get_center_position_of_object_for_object(
                adjusted_width,
                adjusted_height,
                bounding_width as f32,
                bounding_height as f32,
            );

            render::draw_text_antialiased::<{ crate::DRAW_SAFE }>(
                &buffer,
                &self.text,
                (pos.0 as usize, pos.1 as usize),
                text_color,
                self.height as f32 * size_mul,
                &formatting.font,
            );
        } else {
            // It's not perfect since x_at_end is broken but oh well
            let sin_x = (self.scroll * 2.0 * std::f64::consts::PI).sin();
            let amplitude =
                ((text_width as usize - bounding_width) as f32 / 2.0).max(0.0);
            let center = (bounding_width as f32 - text_width) / 2.0;

            let x_at_end = sin_x as f32 * amplitude;
            let x_at_start = (sin_x as f32).mul_add(amplitude, center);

            let progress = f64::midpoint(sin_x, 1.0);

            let x =
                mirl::math::interpolate(x_at_end, x_at_start, progress as f32);

            render::draw_text_antialiased_isize::<{ crate::DRAW_SAFE }>(
                &buffer,
                &self.text,
                (x as isize, 0),
                text_color,
                self.height as f32 / size_mul_y,
                &formatting.font,
            );
            self.needs_redraw.set(true);
        }

        buffer
    }
    fn get_height(&self, _formatting: &crate::Formatting) -> isize {
        self.height as isize
    }
    fn get_width(&self, _formatting: &crate::Formatting) -> isize {
        self.width as isize
    }
    fn update(&mut self, info: &crate::ModuleInputs) -> crate::GuiOutput {
        self.scroll += info.delta_time * self.scroll_multiplier;
        self.scroll %= 1.0;

        if info.focus_taken {
            self.pressed = false;
            self.hovering = false;
            return crate::GuiOutput::default(false);
        }
        let collision = mirl::math::collision::Rectangle::<_, false>::new(
            0,
            0,
            self.width as isize,
            self.height as isize,
        );
        if let Some(mouse_position) = info.mouse_pos {
            let collides = collision.does_area_contain_point(mouse_position);
            if collides {
                self.hovering = true;
                if info.mouse_info.left.clicked
                    && let Some(function) = &self.function
                {
                    function();
                }
                if (self.pressed && info.mouse_info.left.down)
                    || info.mouse_info.left.clicked
                {
                    if !self.pressed {
                        self.needs_redraw.set(true);
                    }
                    self.pressed = true;
                } else {
                    self.pressed = false;
                    self.needs_redraw.set(true);
                }
            } else {
                if self.hovering {
                    self.needs_redraw.set(true);
                }
                self.pressed = false;
                self.hovering = false;
            }
        } else {
            self.pressed = false;
            self.hovering = false;
        }

        crate::GuiOutput::default(self.pressed)
    }
    fn need_redraw(&self) -> bool {
        if self.needs_redraw.get() {
            self.needs_redraw.set(false);
            true
        } else {
            false
        }
    }
    fn apply_new_formatting(&mut self, _formatting: &crate::Formatting) {}
}
