use mirl::prelude::Buffer;

use crate::{DearMirlGuiModule, FocusTaken, module_manager::InsertionMode};
#[derive(Debug, Clone, PartialEq)]
#[allow(unpredictable_function_pointer_comparisons)]
/// A clickable image button
pub struct ImageButton {
    /// A function that will be executed every time the button is pressed, not released
    pub function: Option<fn()>,
    /// If the cursor is currently hovering over the button
    pub hovering: usize,
    /// If the button is actively pressed
    pub pressed: bool,
    /// The image buffer
    pub image: Buffer,
    /// The image will be resized to fit this width
    pub width: usize,
    /// The image will be resized to fit this height
    pub height: usize,
    /// How the image should be resized
    pub resizing_method: mirl::graphics::InterpolationMode,
    #[allow(missing_docs)]
    pub needs_redraw: std::cell::Cell<bool>,
    /// By how much the brightness of the image will change when the mouse is hover over the button
    pub brightness_change_on_hover: f32,
    /// By how much the brightness of the image will change when the mouse is clicking the button (In addition to the hover brightness change)
    pub brightness_change_on_clicking: f32,
}

impl ImageButton {
    const DEFAULT_BRIGHTNESS_CHANGE_ON_HOVER: f32 = 10.0;
    const DEFAULT_BRIGHTNESS_CHANGE_ON_CLICKING: f32 = -10.0;

    #[allow(missing_docs)]
    #[must_use]
    pub fn new(
        image: Buffer,
        size: Option<(usize, usize)>,
        resizing_method: Option<mirl::graphics::InterpolationMode>,
        function: Option<fn()>,
    ) -> Self {
        let size = size.unwrap_or((image.width, image.height));
        Self {
            function,
            pressed: false,
            hovering: 0,
            image,
            width: size.0,
            height: size.1,
            resizing_method: resizing_method.unwrap_or_default(),
            needs_redraw: std::cell::Cell::new(true),
            brightness_change_on_hover:
                Self::DEFAULT_BRIGHTNESS_CHANGE_ON_HOVER,
            brightness_change_on_clicking:
                Self::DEFAULT_BRIGHTNESS_CHANGE_ON_CLICKING,
        }
    }
}

impl DearMirlGuiModule for ImageButton {
    fn set_need_redraw(&mut self, need_redraw: Vec<(usize, bool)>) {
        self.needs_redraw.set(super::misc::determine_need_redraw(need_redraw));
    }

    fn draw(
        &mut self,
        _formatting: &crate::Formatting,
        info: &crate::ModuleDrawInfo,
    ) -> (Buffer, InsertionMode) {
        // Get the base image, resized if necessary
        let mut buffer = if self.width == self.image.width
            && self.height == self.image.height
        {
            self.image.clone()
        } else {
            self.image
                .resize_content((self.width, self.height), self.resizing_method)
        };

        if self.hovering == info.container_id {
            let mut darken = self.brightness_change_on_hover;

            if self.pressed {
                darken += self.brightness_change_on_clicking;
            }
            buffer.apply_filter(|x| {
                mirl::graphics::adjust_brightness_hsl_of_rgb(x, darken)
            });
        }

        (buffer, InsertionMode::Simple)
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
        if info.focus_taken.is_focus_taken() {
            self.pressed = false;
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
                self.hovering = info.container_id;

                // Execute the function when clicked
                if info.mouse_info.left.clicked
                    && let Some(function) = &self.function
                {
                    function();
                }

                // Handle pressed state
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
                if self.hovering != 0 {
                    self.needs_redraw.set(true);
                }
                self.pressed = false;
                if self.hovering == info.container_id {
                    self.hovering = 0;
                }
            }
        } else {
            self.pressed = false;
            self.hovering = 0;
        }

        if self.pressed {
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
