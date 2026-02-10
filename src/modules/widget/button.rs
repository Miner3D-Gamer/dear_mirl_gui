use mirl::{platform::mouse::ButtonState, prelude::Buffer, render};

use crate::{
    DearMirlGuiModule, FocusTaken, ModulePath,
    module_manager::{InsertionMode, get_formatting},
};
#[derive(Debug, Clone, PartialEq)]
#[allow(unpredictable_function_pointer_comparisons)]
/// A simple button
pub struct Button {
    /// A function that will be executed every time the button is interacted with
    pub interaction_function: fn(ButtonState),
    /// is currently hovering over the button
    pub hovering: usize,
    /// If the button is actively pressed
    pub pressed: ButtonState,
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
    /// Keep track of the menus so each one can be drawn correctly
    pub menus: Vec<(usize, bool)>,
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
        // height: usize,
        // width: Option<usize>,
        // font: Option<&mirl::dependencies::fontdue::Font>,
        // function_on_click: Option<fn()>,
        // function_on_release: Option<fn()>,
        // function_on_held: Option<fn(usize)>,
    ) -> Self {
        let formatting = get_formatting();
        let height = formatting.height;

        Self {
            pressed: ButtonState::default(),
            hovering: 0,
            width: render::get_text_width(
                &text,
                height as f32,
                &formatting.font,
            ) as usize,
            height,
            text,
            needs_redraw: std::cell::Cell::new(true),
            scroll: 0.0,
            scroll_multiplier: Self::DEFAULT_SCROLL_MULTIPLIER,
            color_change_on_hover: Self::DEFAULT_COLOR_CHANGE_ON_HOVER,
            color_change_on_clicking: Self::DEFAULT_COLOR_CHANGE_ON_CLICKING,
            threshold_before_text_scrolls:
                Self::DEFAULT_THRESHOLD_BEFORE_TEXT_SCROLLS,
            menus: Vec::new(),
            interaction_function: |_| {},
        }
    }
    #[must_use]
    /// Set the current width, use [with_height](Self::with_height) for setting the height
    pub const fn with_width(mut self, width: usize) -> Self {
        self.width = width;
        self
    }
    #[must_use]
    /// Set the current width, use [with_width](Self::with_width) for setting the width
    pub const fn with_height(mut self, height: usize) -> Self {
        self.height = height;
        self
    }
    #[must_use]
    /// Set what function shall be called when the button is interacted with
    pub const fn with_interaction_function(
        mut self,
        interaction_function: fn(ButtonState),
    ) -> Self {
        self.interaction_function = interaction_function;
        self
    }
    fn set_text(&mut self, text: String) {
        self.text = text;
    }
}

impl DearMirlGuiModule for Button {
    // fn as_any(&self) -> &dyn Any {
    //     self
    // }
    // fn as_any_mut(&mut self) -> &mut dyn Any {
    //     self
    // }
    fn set_need_redraw(&mut self, need_redraw: Vec<(usize, bool)>) {
        self.needs_redraw.set(super::misc::determine_need_redraw(need_redraw));
    }
    fn draw(
        &mut self,
        formatting: &crate::Formatting,
        info: &crate::ModuleDrawInfo,
    ) -> (Buffer, InsertionMode) {
        self.needs_redraw.set(false);
        let text_color = formatting.text_color;

        let mut color = formatting.foreground_color;
        //println!("Hover: {} Drawing for: {}", self.hovering, info.container_id);
        if self.hovering == info.container_id {
            color = mirl::graphics::adjust_brightness_hsl_of_rgb(
                color,
                self.color_change_on_hover,
            );
            if self.pressed.down {
                color = mirl::graphics::adjust_brightness_hsl_of_rgb(
                    color,
                    self.color_change_on_clicking,
                );
            }
        }
        let mut buffer =
            Buffer::new_empty_with_color((self.width, self.height), color);

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
                &mut buffer,
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
                &mut buffer,
                &self.text,
                (x as isize, 0),
                text_color,
                self.height as f32 / size_mul_y,
                &formatting.font,
            );
            self.needs_redraw.set(true);
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
        self.width as crate::DearMirlGuiCoordinateType
    }
    fn update(&mut self, info: &crate::ModuleUpdateInfo) -> crate::GuiOutput {
        self.scroll += info.delta_time * self.scroll_multiplier;
        self.scroll %= 1.0;

        if info.focus_taken.is_focus_taken() {
            self.pressed = ButtonState::default();
            if self.hovering == info.container_id {
                self.hovering = 0;
            }
            return crate::GuiOutput::empty();
        }
        let collision = mirl::math::geometry::Pos2D::<
            _,
            mirl::math::collision::Rectangle<_, false>,
        >::new(
            (0.0, 0.0),
            mirl::math::collision::Rectangle::new((
                self.width as f32,
                self.height as f32,
            )),
        );
        if let Some(mouse_position) = info.mouse_pos {
            let collides = collision.does_area_contain_point(mouse_position);
            if collides {
                if self.hovering != info.container_id {
                    self.needs_redraw.set(true);
                    self.hovering = info.container_id;
                }
                (self.interaction_function)(info.mouse_info.left);

                if (self.pressed.down && info.mouse_info.left.down)
                    || info.mouse_info.left.clicked
                {
                    // if !self.pressed {
                    //     self.needs_redraw.set(true);
                    // }
                    self.pressed.update(true);
                } else {
                    self.pressed.update(false);
                }
            } else {
                if self.hovering == info.container_id {
                    self.hovering = 0;
                    self.pressed.update(false);
                }
                if self.hovering != 0 {
                    self.needs_redraw.set(true);
                }
            }
        } else {
            self.pressed.update(false);
            self.hovering = 0;
        }
        if self.pressed.down {
            crate::GuiOutput::default(FocusTaken::FunctionallyTaken)
        } else if self.hovering == info.container_id {
            crate::GuiOutput::default(FocusTaken::VisuallyTaken)
        } else {
            crate::GuiOutput::empty()
        }
    }
    fn need_redraw(&mut self) -> bool {
        if self.needs_redraw.get() {
            self.needs_redraw.set(false);
            true
        } else {
            false
        }
    }
    fn apply_new_formatting(&mut self, _formatting: &crate::Formatting) {}
}

/// A trait to get/set values using the path instead of manually getting the module
pub trait ButtonModulePathSupport {
    /// Check if the button is pressed down (continues)
    fn is_down(&self) -> bool;
    /// Check if the button has been clicked (1 tick)
    fn clicked(&self) -> bool;
    /// Set the text of the button
    fn set_text(&self, text: String);
}

impl ButtonModulePathSupport for ModulePath<Button> {
    fn is_down(&self) -> bool {
        crate::module_manager::get_module_as_mut::<_, bool>(self, |button| {
            button.pressed.down
        })
        .unwrap_or_default()
    }
    fn clicked(&self) -> bool {
        crate::module_manager::get_module_as_mut::<_, bool>(self, |button| {
            button.pressed.clicked
        })
        .unwrap_or_default()
    }
    fn set_text(&self, text: String) {
        let _ =
            crate::module_manager::get_module_as_mut::<_, ()>(self, |button| {
                button.set_text(text);
            });
    }
}
