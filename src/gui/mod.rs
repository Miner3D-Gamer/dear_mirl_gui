// Just a little helper lib, it'll take like 3 days
// 1+2: background + dragging + resizing + text + sliders/progress
// 3: refined sliders + buttons
// 4: split sliders/progress bar into own modules + check box
// 5: Trying to compress 6.5k images into a reasonable size
// 6: "
// 7: "
// 8: Added better formatting, grouped buttons/radio buttons, line divider, and anti new line
// 9: None
// 10: Alright this is getting too stupid for documentation
// 11: None
// 12: Windows are scrollable, window emulator added, fixed other stuff
// 13: Made scrolling a little more intuitive, de-hardcoded some values bc why not, polished some visuals
// 14: Improved formatting once more, fixed caching which improves performance by 5%-8% in the example window; it's not a lot but sure
// The Next two weeks: Add keybinds, add _all_ keybind functionality
// Idea: Instead of each module that has a Buffer holding it's own data, there is a global storage each module can request/upload textures to

mod extra;

pub use extra::{ModuleContainer, ModuleVTable};
use mirl::{
    extensions::*,
    misc::{
        corner_type_and_delta_to_metric_change, corner_type_to_cursor_style,
    },
    platform::{Buffer, CursorStyle, keycodes::KeyCode},
    render,
};

type BufferAndOffset = (std::rc::Rc<Buffer>, (isize, isize));

use crate::{
    ButtonState, DearMirlGuiModule, Formatting, GuiOutput, GuiReturnsModule,
    ModuleInputs, MouseData,
};
#[derive(Debug, Clone)]
#[allow(clippy::struct_excessive_bools)] // This ain't no state machine!
/// A single window
pub struct DearMirlGui<const FAST: bool, const USE_CACHE: bool> {
    /// Title displayed on top of the menu bar
    pub title: String,
    /// Width of the window - modules that exceed this will be visually cut off
    pub width: usize,
    /// Height of the window - modules that exceed this will be visually cut off
    pub height: usize,
    /// X position on screen
    pub x: isize,
    /// Y position on screen
    pub y: isize,
    /// Added modules
    pub modules: indexmap::IndexMap<String, ModuleContainer>,
    /// Toolbar modules - Arranged horizontally instead of vertically
    pub toolbar_modules: indexmap::IndexMap<String, ModuleContainer>,
    /// Last saved mouse position - is [`(isize::MIN, isize::MIN)`](isize::MIN) when invalidated
    pub last_mouse_pos: (isize, isize),
    /// If the left mouse button was pressed last frame
    pub last_left_mouse_down: bool,
    /// of the middle mouse button was pressed last frame
    pub last_middle_mouse_down: bool,
    /// If the right mouse button was pressed last frame
    pub last_right_mouse_down: bool,
    /// If the user is currently resizing the window
    pub resizing: bool,
    /// At what corner the user currently has their mouse, is [`u8::MAX`] when not in range
    pub at_corner: u8,
    /// If the user is currently dragging the window
    pub dragging: bool,
    /// The color of the menu title
    /// The main color of the window, possibly used by modules if not custom color is defined
    /// The secondary color of the window, possibly used by modules if not custom color is defined
    /// Vertical margin between the modules and the edge
    /// Horizontal margin between modules and the edge
    pub formatting: Formatting,
    /// Just the menu height, windows cannot be vertically smaller then this
    pub menu_height: usize,
    /// The minimum menu width - Menus cannot be horizontally smaller than this
    pub min_width: usize,
    /// If the window should be collapsed
    pub collapsed: bool,
    /// A cache so modules don't need to redraw themselves every frame
    pub cache: Vec<Option<BufferAndOffset>>,
    /// A cache so toolbar modules don't need to redraw themselves every frame
    pub toolbar_cache: Vec<Option<std::rc::Rc<Buffer>>>,
    /// If the collapse button collision should be circular
    pub collapse_button_collision_is_circle: bool,
    /// If there have been any changes to the gui
    pub needs_redraw: std::cell::Cell<bool>,
    /// In what directions resizing is allowed
    pub resizing_allowed_in_directions: Directions,
    /// If you're allowed to drag the window
    pub allow_dragging: bool,
    /// Horizontal scroll offset
    pub scroll_offset_x: isize,
    /// Vertical scroll offset
    pub scroll_offset_y: isize,
    /// When scrolling horizontally, by how much should the x be multiplied
    pub horizontal_scroll_multiplier_x: isize,
    /// When scrolling horizontally, by how much should the y be multiplied
    pub horizontal_scroll_multiplier_y: isize,
    /// When scrolling vertically, by how much should the x be multiplied
    pub vertical_scroll_multiplier_x: isize,
    /// When scrolling vertically, by how much should the y be multiplied
    pub vertical_scroll_multiplier_y: isize,
}
#[allow(clippy::struct_excessive_bools, missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// A boolean for each simple Direction
pub struct Directions {
    pub top: bool,
    pub bottom: bool,
    pub left: bool,
    pub right: bool,
    pub top_left: bool,
    pub top_right: bool,
    pub bottom_left: bool,
    pub bottom_right: bool,
}
impl Directions {
    #[must_use]
    #[allow(clippy::fn_params_excessive_bools)] // Really clippy? 4 booleans is excessive in your eyes?
    /// Create a simple directional boolean struct
    pub const fn new(top: bool, bottom: bool, left: bool, right: bool) -> Self {
        Self {
            top,
            bottom,
            left,
            right,
            top_left: top && left,
            top_right: top && right,
            bottom_left: bottom && left,
            bottom_right: bottom && right,
        }
    }
    /// "Yes"
    #[must_use]
    pub const fn all_true() -> Self {
        Self {
            top: true,
            bottom: true,
            left: true,
            right: true,
            top_left: true,
            top_right: true,
            bottom_left: true,
            bottom_right: true,
        }
    }
    /// "No"
    #[must_use]
    pub const fn all_false() -> Self {
        Self {
            top: false,
            bottom: false,
            left: false,
            right: false,
            top_left: false,
            top_right: false,
            bottom_left: false,
            bottom_right: false,
        }
    }

    /// Check if a direction is true
    #[must_use]
    pub const fn is_direction_allowed_u8(&self, direction: u8) -> bool {
        match direction {
            0 => self.top_left,
            1 => self.top,
            2 => self.top_right,
            3 => self.right,
            4 => self.bottom_right,
            5 => self.bottom,
            6 => self.bottom_left,
            7 => self.left,
            _ => false,
        }
    }
}

impl<const FAST: bool, const USE_CACHE: bool> DearMirlGui<FAST, USE_CACHE> {
    /// The margin between individual modules and the edge
    pub const DEFAULT_HORIZONTAL_MARGIN: usize = 5;
    /// The margin between individual modules and the edge
    pub const DEFAULT_VERTICAL_MARGIN: usize = 5;
    /// The height of the Menu/Text on the menu -> The colored part above the module area
    pub const DEFAULT_MENU_HEIGHT: usize = 20;
    /// The color of the background
    pub const DEFAULT_BACKGROUND_COLOR: u32 =
        mirl::graphics::rgba_to_u32(10, 5, 20, 255);
    /// The color of the stuff that neither the background nor text
    pub const DEFAULT_STUFF_COLOR: u32 =
        mirl::graphics::rgba_to_u32(40, 30, 100, 255);
    /// The color of the text
    pub const DEFAULT_TEXT_COLOR: u32 = mirl::graphics::color_presets::WHITE;
    /// By how much the X part of the mouse scroll should be multiplied with when scrolling vertically
    pub const DEFAULT_VERTICAL_SCROLL_X_MULTIPLIER: isize = 1;
    /// By how much the Y part of the mouse scroll should be multiplied with when scrolling vertically
    pub const DEFAULT_VERTICAL_SCROLL_Y_MULTIPLIER: isize = 1;
    /// By how much the X part of the mouse scroll should be multiplied with when scrolling horizontally
    pub const DEFAULT_HORIZONTAL_SCROLL_X_MULTIPLIER: isize = 1;
    /// By how much the Y part of the mouse scroll should be multiplied with when scrolling horizontally
    pub const DEFAULT_HORIZONTAL_SCROLL_Y_MULTIPLIER: isize = 1;
    #[must_use]
    #[allow(clippy::too_many_arguments)]
    /// Create a new `DearMirlGui` window
    /// Just clone the `fontdue::Font`
    pub fn new_advanced(
        title: &str,
        x: isize,
        y: isize,
        width: usize,
        height: usize,
        modules: Option<indexmap::IndexMap<String, ModuleContainer>>,
        toolbar_modules: Option<indexmap::IndexMap<String, ModuleContainer>>,
        font: &fontdue::Font,
        horizontal_margin: Option<usize>,
        vertical_margin: Option<usize>,
        menu_height: Option<usize>,
        min_width: Option<usize>,
        main_color: Option<u32>,
        secondary_color: Option<u32>,
        menu_text_color: Option<u32>,
        allow_resizing_in_directions: Option<Directions>,
        allow_dragging: bool,
        collapsed: bool,
        horizontal_scroll_multiplier_x: Option<isize>,
        horizontal_scroll_multiplier_y: Option<isize>,
        vertical_scroll_multiplier_x: Option<isize>,
        vertical_scroll_multiplier_y: Option<isize>,
    ) -> Self {
        let menu_height = menu_height.unwrap_or(Self::DEFAULT_MENU_HEIGHT);
        Self {
            title: title.to_string(),
            width,
            height,
            modules: modules.clone().unwrap_or_default(),
            toolbar_modules: toolbar_modules.clone().unwrap_or_default(),
            last_mouse_pos: (0, 0),
            last_left_mouse_down: false,
            last_middle_mouse_down: false,
            last_right_mouse_down: false,
            x,
            y,
            resizing: false,
            at_corner: u8::MAX,
            dragging: false,
            formatting: Formatting {
                font: font.clone(),
                main_color: main_color
                    .unwrap_or(Self::DEFAULT_BACKGROUND_COLOR),
                secondary_color: secondary_color
                    .unwrap_or(Self::DEFAULT_STUFF_COLOR),
                text_color: menu_text_color.unwrap_or(Self::DEFAULT_TEXT_COLOR),
                misc_ui_color: 0,
                horizontal_margin: horizontal_margin
                    .unwrap_or(Self::DEFAULT_HORIZONTAL_MARGIN)
                    / 2,
                vertical_margin: vertical_margin
                    .unwrap_or(Self::DEFAULT_VERTICAL_MARGIN)
                    / 2,
            },
            menu_height,
            min_width: min_width.unwrap_or_else(|| {
                mirl::render::get_text_width(title, menu_height as f32, font)
                    as usize
                    + menu_height
            }),
            collapsed,
            cache: None.repeat_value(modules.unwrap_or_default().len()),
            toolbar_cache: None
                .repeat_value(toolbar_modules.unwrap_or_default().len()),
            collapse_button_collision_is_circle: false,
            needs_redraw: true.into(),
            resizing_allowed_in_directions: allow_resizing_in_directions
                .unwrap_or(Directions::all_true()),
            allow_dragging,
            scroll_offset_x: 0,
            scroll_offset_y: 0,
            horizontal_scroll_multiplier_x: horizontal_scroll_multiplier_x
                .unwrap_or(Self::DEFAULT_HORIZONTAL_SCROLL_X_MULTIPLIER),
            horizontal_scroll_multiplier_y: horizontal_scroll_multiplier_y
                .unwrap_or(Self::DEFAULT_HORIZONTAL_SCROLL_Y_MULTIPLIER),
            vertical_scroll_multiplier_x: vertical_scroll_multiplier_x
                .unwrap_or(Self::DEFAULT_VERTICAL_SCROLL_X_MULTIPLIER),
            vertical_scroll_multiplier_y: vertical_scroll_multiplier_y
                .unwrap_or(Self::DEFAULT_VERTICAL_SCROLL_Y_MULTIPLIER),
        }
    }

    #[must_use]
    #[allow(clippy::too_many_arguments)]
    /// Create a new `DearMirlGui` window - use [`Self::new_advanced`] for more control
    pub fn new_simple(
        title: &str,
        x: isize,
        y: isize,
        width: usize,
        height: usize,
        font: &fontdue::Font,
    ) -> Self {
        Self {
            title: title.to_string(),
            width,
            height,
            modules: indexmap::IndexMap::new(),
            toolbar_modules: indexmap::IndexMap::new(),
            last_mouse_pos: (0, 0),
            last_left_mouse_down: false,
            last_middle_mouse_down: false,
            last_right_mouse_down: false,
            x,
            y,
            resizing: false,
            at_corner: u8::MAX,
            dragging: false,
            formatting: Formatting {
                font: font.clone(),
                main_color: Self::DEFAULT_BACKGROUND_COLOR,
                secondary_color: Self::DEFAULT_STUFF_COLOR,
                text_color: Self::DEFAULT_TEXT_COLOR,
                misc_ui_color: 0,
                horizontal_margin: Self::DEFAULT_HORIZONTAL_MARGIN / 2,
                vertical_margin: Self::DEFAULT_VERTICAL_MARGIN / 2,
            },
            menu_height: Self::DEFAULT_MENU_HEIGHT,
            min_width: render::get_text_width(
                title,
                Self::DEFAULT_MENU_HEIGHT as f32,
                font,
            ) as usize
                + Self::DEFAULT_MENU_HEIGHT,
            collapsed: false,
            cache: Vec::new(),
            toolbar_cache: Vec::new(),
            collapse_button_collision_is_circle: false,
            needs_redraw: false.into(),
            allow_dragging: true,
            resizing_allowed_in_directions: Directions::all_true(),
            scroll_offset_x: 0,
            scroll_offset_y: 0,
            horizontal_scroll_multiplier_x:
                Self::DEFAULT_HORIZONTAL_SCROLL_X_MULTIPLIER,
            horizontal_scroll_multiplier_y:
                Self::DEFAULT_HORIZONTAL_SCROLL_Y_MULTIPLIER,
            vertical_scroll_multiplier_x:
                Self::DEFAULT_VERTICAL_SCROLL_X_MULTIPLIER,
            vertical_scroll_multiplier_y:
                Self::DEFAULT_VERTICAL_SCROLL_Y_MULTIPLIER,
        }
    }
    /// Update the min width based on the length of the window title
    pub fn update_min_width(&mut self) {
        self.min_width = render::get_text_width(
            &self.title,
            self.menu_height as f32,
            &self.formatting.font,
        ) as usize
            + self.menu_height;
    }
    /// Checks if any module needs to be redrawn
    pub fn needs_redraw(&self) -> bool {
        if self.needs_redraw.get() {
            return true;
        }
        for (_, module) in &self.modules {
            if module.need_redraw() {
                return true;
            }
        }
        false
    }
    /// Get the current height of the window
    pub const fn get_height(&self) -> usize {
        if self.collapsed {
            self.menu_height
        } else {
            self.height
        }
    }
    /// Get the current width of the window
    pub const fn get_width(&self) -> usize {
        self.width
    }

    /// Replace all cache modules with None
    pub fn reset_cache(&mut self) {
        self.cache = None.repeat_value(self.cache.len());
    }
    // /// Move the window relative to it's current position
    // pub const fn move_by(&mut self, xy: (isize, isize)) {
    //     self.x += xy.0;
    //     self.y += xy.1;
    // }

    /// Add a module with type preservation
    pub fn add_module<T: DearMirlGuiModule + 'static>(
        &mut self,
        name: &str,
        module: T,
    ) {
        self.modules.insert(name.to_string(), ModuleContainer::new(module));
        self.cache.push(None);
    }
    /// Remove a module
    pub fn remove_module<T: DearMirlGuiModule + 'static>(
        &mut self,
        name: &String,
    ) {
        let index = self.modules.get_index_of(name);
        if let Some(idx) = index {
            self.modules.shift_remove_index(idx);
            self.cache.remove(idx);
        }
    }

    /// Add a module with type preservation to the toolbar
    pub fn add_toolbar_module<T: DearMirlGuiModule + 'static>(
        &mut self,
        name: &str,
        module: T,
    ) {
        self.toolbar_modules
            .insert(name.to_string(), ModuleContainer::new(module));
        self.toolbar_cache.push(None);
    }
    /// Remove a toolbar module
    pub fn remove_toolbar_module<T: DearMirlGuiModule + 'static>(
        &mut self,
        name: &str,
    ) {
        let index = self.toolbar_modules.get_index_of(&name.to_string());
        if let Some(idx) = index {
            self.toolbar_modules.shift_remove_index(idx);
            self.toolbar_cache.remove(idx);
        }
    }

    /// Get typed access to a module using closure (immutable)
    pub fn get_module<R>(
        &self,
        name: &str,
        f: impl FnOnce(&dyn DearMirlGuiModule) -> R,
    ) -> Option<R> {
        self.modules.get(name).map(|module| module.with_ref(f))
    }

    /// Get typed access to a module using closure (mutable)
    pub fn get_module_mut<R>(
        &self,
        name: &str,
        f: impl FnOnce(&mut dyn DearMirlGuiModule) -> R,
    ) -> Option<R> {
        self.modules.get(name).map(|module| module.with_ref_mut(f))
    }

    /// Get typed access to a toolbar module using closure (immutable)
    pub fn with_toolbar_module<R>(
        &self,
        name: &str,
        f: impl FnOnce(&dyn DearMirlGuiModule) -> R,
    ) -> Option<R> {
        self.toolbar_modules.get(name).map(|module| module.with_ref(f))
    }

    /// Get typed access to a toolbar module using closure (mutable)
    pub fn with_toolbar_module_mut<R>(
        &self,
        name: &str,
        f: impl FnOnce(&mut dyn DearMirlGuiModule) -> R,
    ) -> Option<R> {
        self.needs_redraw.set(true);
        self.toolbar_modules.get(name).map(|module| module.with_ref_mut(f))
    }

    /// Automatically draw the window onto a `mirl::platform::Buffer`
    pub fn draw_on_buffer(&mut self, buffer: &Buffer) {
        let to_draw = self.draw();
        if buffer
            .create_collision_isize::<false>(0, 0)
            .does_area_fully_include_other_area(
                &to_draw.create_collision_isize(self.x, self.y),
            )
        {
            render::draw_buffer_on_buffer_1_to_1::<false, false, false>(
                buffer,
                &to_draw,
                (self.x, self.y),
            );
        } else {
            render::draw_buffer_on_buffer_1_to_1::<true, false, false>(
                buffer,
                &to_draw,
                (self.x, self.y),
            );
        }
    }
    /// Usage Example:
    /// ```ignore
    ///                                 // Module Type, Function Return, Module ID, Function
    ///                                 // v                       v     v            v
    /// let progress = gui.get_module_as::<crate::modules::Slider, f32> ("module_id", |slider| slider.progress);
    /// println!("Current progress: {progress}");
    /// ```
    #[must_use]
    pub fn get_module_as<T: 'static, R>(
        &self,
        name: &str,
        f: impl FnOnce(&T) -> R,
    ) -> GuiReturnsModule<R> {
        self.needs_redraw.set(true);

        let module = self.get_module(name, |module| {
            (
                module.as_any().downcast_ref::<T>().map(f),
                std::any::type_name::<T>(),
                module.what_am_i(),
            )
        });
        module.map_or_else(
            || GuiReturnsModule::UnableToFindID(name.to_string()),
            |value| {
                let (val, wrongly_used, correct) = value;
                val.map_or_else(
                    || GuiReturnsModule::CastingAsWrongModule {
                        wrong: wrongly_used.to_string(),
                        correct: correct.to_string(),
                        id: name.to_string(),
                    },
                    |output| GuiReturnsModule::AllGood(output),
                )
            },
        )
    }

    /// Usage Example:
    /// ```ignore
    ///                   // Module Type, Function Return, Module ID, Function
    ///                   // v                       v     v           v
    /// .get_module_as_mut::<crate::modules::Slider, ()>  ("slider_1", |slider| {
    ///        slider.progress += delta_time as f32 / 10.0;
    ///        slider.progress = slider.progress.clamp(0.0, 1.0);
    ///        if slider.progress == 1.0 {
    ///            slider.progress = 0.0
    ///        }
    ///    })
    /// ```
    #[must_use]
    pub fn get_module_as_mut<T: 'static, R>(
        &self,
        name: &str,
        f: impl FnOnce(&mut T) -> R,
    ) -> GuiReturnsModule<R> {
        let module = self.get_module_mut(name, |module| {
            (
                module.as_any_mut().downcast_mut::<T>().map(f),
                std::any::type_name::<T>(),
                module.what_am_i(),
            )
        });
        module.map_or_else(
            || GuiReturnsModule::UnableToFindID(name.to_string()),
            |value| {
                let (val, wrongly_used, correct) = value;
                val.map_or_else(
                    || GuiReturnsModule::CastingAsWrongModule {
                        wrong: wrongly_used.to_string(),
                        correct: correct.to_string(),
                        id: name.to_string(),
                    },
                    |output| GuiReturnsModule::AllGood(output),
                )
            },
        )
    }
    fn draw_menu(
        &self,
        buffer: &Buffer,
        font: &fontdue::Font,
        collapse_button_size: f64,
        collapse_button_color_change: f32,
    ) {
        render::draw_rectangle::<{ crate::DRAW_SAFE }>(
            buffer,
            0,
            0,
            buffer.width as isize,
            self.menu_height as isize,
            self.formatting.secondary_color,
        );
        render::draw_text_antialiased_isize::<{ crate::DRAW_SAFE }>(
            buffer,
            &self.title,
            (
                self.formatting.horizontal_margin as isize
                    + self.menu_height as isize,
                -(self.formatting.vertical_margin as isize),
            ),
            self.formatting.text_color,
            (self.menu_height - self.formatting.vertical_margin) as f32,
            font,
        );

        let h = self.menu_height / 2;
        render::draw_circle::<false>(
            buffer,
            h,
            h,
            (h as f64 * collapse_button_size) as isize,
            mirl::graphics::adjust_brightness_hsl_of_rgb(
                self.formatting.secondary_color,
                collapse_button_color_change,
            ),
            false, // THIS CRASHES WHEN THE CURRENT IS SMALLER THAN THE MENU HEIGHT
        );
    }
    #[must_use]
    #[allow(clippy::too_many_lines)]
    /// Use [`mirl::render::draw_buffer_on_buffer`](mirl::render::draw_buffer_on_buffer) (or [`draw_buffer_on_buffer_1_to_1`](mirl::render::draw_buffer_on_buffer_1_to_1)) to draw this buffer on any other
    pub fn draw(&mut self) -> Buffer {
        self.needs_redraw.set(false);
        if self.height < self.menu_height {
            return Buffer::generate_fallback(self.width, self.height, 4);
        }

        let collapse_button_size = 0.8;
        let collapse_button_color_change = -10.0;

        let buffer = Buffer::new_empty_with_color(
            self.width,
            if self.collapsed {
                self.menu_height
            } else {
                self.height
            },
            self.formatting.main_color,
        );

        if self.collapsed {
            self.draw_menu(
                &buffer,
                &self.formatting.font,
                collapse_button_size,
                collapse_button_color_change,
            );
            return buffer;
        }

        let collision = buffer.create_collision_isize::<false>(0, 0);

        // Using Rc<Buffer> instead of Buffer, without caching, increased testing fps from a stable 62 to a stable 63 - What
        let mut buffers = Vec::new();
        let mut offset =
            (self.menu_height + self.formatting.horizontal_margin) as isize;

        for (idx, (_, i)) in (&self.modules).into_iter().enumerate() {
            let buf: (std::rc::Rc<Buffer>, (isize, isize)) = if USE_CACHE {
                let p = &self.cache[idx];
                if i.need_redraw() {
                    (
                        std::rc::Rc::new(i.draw(&self.formatting)),
                        i.get_next_offset(&self.modules, idx, &self.formatting),
                    )
                } else if let Some(frame) = p {
                    frame.clone()
                } else {
                    (
                        std::rc::Rc::new(i.draw(&self.formatting)),
                        i.get_next_offset(&self.modules, idx, &self.formatting),
                    )
                }
            } else {
                (
                    std::rc::Rc::new(i.draw(&self.formatting)),
                    i.get_next_offset(&self.modules, idx, &self.formatting),
                )
            };
            if USE_CACHE {
                self.cache[idx] = Some(buf.clone());
            }
            let height = i.get_height(&self.formatting);
            buffers.push((buf, offset));
            offset += height + self.formatting.vertical_margin as isize;
        }

        let mut extra_vertical_offset = self.scroll_offset_y;
        let mut extra_horizontal_offset = self.scroll_offset_x;

        for ((buf, previous_module_offset), vertical_offset) in buffers {
            let x = self.formatting.horizontal_margin as isize
                + extra_horizontal_offset;
            let y = vertical_offset + extra_vertical_offset;

            let col = buf.create_collision_isize::<false>(x, y);
            let position = (x, y);

            if collision.does_area_fully_include_other_area(&col) {
                render::draw_buffer_on_buffer_1_to_1::<false, FAST, false>(
                    &buffer, &buf, position,
                );
            } else {
                render::draw_buffer_on_buffer_1_to_1::<true, FAST, false>(
                    &buffer, &buf, position,
                );
            }
            extra_horizontal_offset += previous_module_offset.0;
            extra_vertical_offset += previous_module_offset.1;
        }
        self.draw_menu(
            &buffer,
            &self.formatting.font,
            collapse_button_size,
            collapse_button_color_change,
        );

        buffer
    }
    #[must_use]
    const fn handle_dragging(
        &mut self,
        mouse_pos: Option<(isize, isize)>,
        mouse_pos_delta: (isize, isize),
        mouse_info: MouseData,
        over_collapse_button: bool,
    ) -> (Option<CursorStyle>, bool) {
        let mut cursor_style = None;
        let mut gui_in_focus = false;
        // WINDOW DRAGGING
        if let Some(current_mouse_position) = mouse_pos {
            let menu_metrics: mirl::math::collision::Rectangle<isize, false> =
                mirl::math::collision::Rectangle::new(
                    self.x + self.menu_height as isize,
                    self.y,
                    self.width as isize - self.menu_height as isize,
                    self.menu_height as isize,
                );
            let collides =
                menu_metrics.does_area_contain_point(current_mouse_position);

            if collides {
                cursor_style = Some(CursorStyle::Copy);
            }
            if (self.dragging && mouse_info.left.down)
                || (mouse_info.left.clicked
                    && collides
                    && !self.resizing
                    && !over_collapse_button)
            {
                gui_in_focus = true;
                //cursor_style = Some(CursorStyle::ClosedHand);
                cursor_style = Some(CursorStyle::AllScroll);
                self.dragging = true;
                // let mouse_pos_delta =
                //     current_mouse_position.sub(self.last_mouse_pos);
                self.x += mouse_pos_delta.0;
                self.y += mouse_pos_delta.1;
            } else {
                self.dragging = false;
            }
        }
        // If no mouse position is set, invalidate it
        if let Some(current_mouse_pos) = mouse_pos {
            self.last_mouse_pos = current_mouse_pos;
        } else {
            self.last_mouse_pos = (isize::MIN, isize::MIN);
        }
        (
            if over_collapse_button {
                None
            } else {
                cursor_style
            },
            gui_in_focus,
        )
    }
    #[must_use]
    const fn handle_resizing(
        &mut self,
        mouse_pos: Option<(isize, isize)>,
        mouse_pos_delta: (isize, isize),
        mouse_info: MouseData,
        over_collapse_button: bool,
    ) -> (Option<CursorStyle>, bool) {
        let mut cursor_style = None;
        let mut gui_in_focus = false;
        if let Some(mouse_position) = mouse_pos {
            // Resize
            let hit_box: mirl::math::collision::Rectangle<isize, true> =
                mirl::math::collision::Rectangle::new(
                    self.x,
                    self.y,
                    self.width as isize,
                    self.height as isize,
                );
            let at_corner;
            // WINDOW RESIZING
            if self.dragging {
                self.at_corner = u8::MAX;
                self.resizing = false;
            } else {
                if self.at_corner == u8::MAX {
                    at_corner = hit_box.get_edge_position(mouse_position, 5);
                } else {
                    at_corner = self.at_corner;
                }
                if !self
                    .resizing_allowed_in_directions
                    .is_direction_allowed_u8(at_corner)
                {
                    return (None, false);
                }
                // Do not show the option to drag if over the collapse button, that is so annoying
                if at_corner != u8::MAX
                    && (self.resizing || !over_collapse_button)
                {
                    cursor_style = corner_type_to_cursor_style(at_corner);
                }
                if (self.resizing && mouse_info.left.down)
                    || (mouse_info.left.clicked
                        && at_corner != u8::MAX
                        && !over_collapse_button
                        && !self.dragging)
                {
                    self.resizing = true;
                    self.at_corner = at_corner;
                    if mouse_pos.is_some() {
                        gui_in_focus = true;
                        // let mouse_pos_delta =
                        //     current_mouse_pos.sub(self.last_mouse_pos);
                        let metric_change =
                            corner_type_and_delta_to_metric_change(
                                at_corner,
                                mouse_pos_delta,
                            );
                        let width = self.width as isize + metric_change.2;
                        if width < self.min_width as isize {
                            self.width = self.min_width;
                        } else {
                            self.width = width as usize;
                            self.x += metric_change.0;
                        }
                        let height = self.height as isize + metric_change.3;
                        if height < self.menu_height as isize {
                            self.height = self.menu_height;
                        } else {
                            self.height = height as usize;
                            self.y += metric_change.1;
                        }
                    }
                } else {
                    self.at_corner = u8::MAX;
                    self.resizing = false;
                }
            }
        }
        (
            if over_collapse_button {
                None
            } else {
                cursor_style
            },
            gui_in_focus,
        )
    }
    /// Please only provide the clipboard data when it is being requested
    ///
    /// You may lie about the input metrics however you like, if anything crashes, please report such to whomever is maintaining this eye strain
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::too_many_lines)] // Well, clippy... it's just big. :(
    pub fn update(
        &mut self,
        mouse_pos: Option<(isize, isize)>,
        mouse_scroll: Option<(isize, isize)>,
        left_mouse_down: bool,
        middle_mouse_down: bool,
        right_mouse_down: bool,
        pressed_keys: &Vec<KeyCode>,
        delta_time: f64,
        clipboard_data: &Option<mirl::platform::file_system::FileData>,
    ) -> GuiOutput {
        let hover_over_menu_takes_focus = false;

        // There are so many checks for if mouse_pos isn't None, who wrote this???
        let mouse_data = MouseData {
            left: ButtonState::new(left_mouse_down, self.last_left_mouse_down),
            middle: ButtonState::new(
                middle_mouse_down,
                self.last_middle_mouse_down,
            ),
            right: ButtonState::new(
                right_mouse_down,
                self.last_right_mouse_down,
            ),
        };

        let mut cursor_style = None;

        let mut over_collapse_button = false;

        let collapse_button_collision =
            if let Some(current_mouse_pos) = mouse_pos {
                if self.collapse_button_collision_is_circle {
                    let t = self.menu_height as isize / 2;
                    mirl::math::collision::Circle::<_, false>::new(
                        self.x + t,
                        self.y + t,
                        t,
                    )
                    .does_area_contain_point(current_mouse_pos)
                } else {
                    mirl::math::collision::Rectangle::<_, false>::new(
                        self.x,
                        self.y,
                        self.menu_height as isize,
                        self.menu_height as isize,
                    )
                    .does_area_contain_point(current_mouse_pos)
                }
            } else {
                false
            };
        if !(self.dragging || self.resizing) && collapse_button_collision {
            over_collapse_button = true;
            cursor_style = Some(CursorStyle::ContextMenu);
            if mouse_data.left.clicked {
                self.collapsed = !self.collapsed;
            }
        }

        // If the mouse position is invalid, reset it
        if self.last_mouse_pos == (isize::MIN, isize::MIN)
            && let Some(current_mouse_pos) = mouse_pos
        {
            self.last_mouse_pos = current_mouse_pos;
        }

        let mouse_pos_delta =
            mouse_pos.unwrap_or((0, 0)).sub(self.last_mouse_pos);

        if let Some(current_mouse_pos) = mouse_pos {
            self.last_mouse_pos = current_mouse_pos;
        }

        let mut gui_in_focus = false;

        // Dragging has priority, not because it should but because it's ordered like this

        // Handle dragging
        if self.allow_dragging {
            let dragging_output = self.handle_dragging(
                mouse_pos,
                mouse_pos_delta,
                mouse_data,
                over_collapse_button,
            );
            if dragging_output.0.is_some() && cursor_style.is_none() {
                cursor_style = dragging_output.0;
            }
            gui_in_focus = gui_in_focus
                || dragging_output.1
                || (dragging_output.0.is_some() && hover_over_menu_takes_focus);
        }
        // Handle resizing
        {
            let resizing_output = self.handle_resizing(
                mouse_pos,
                mouse_pos_delta,
                mouse_data,
                over_collapse_button,
            );
            if resizing_output.0.is_some() && cursor_style.is_none() {
                cursor_style = resizing_output.0;
            }

            gui_in_focus = gui_in_focus
                || resizing_output.1
                || (resizing_output.0.is_some() && hover_over_menu_takes_focus);
        }

        let mut offset = 0;

        let mut hide_cursor = false;
        let mut text_input_selected = false;
        let mut new_cursor_position = None;
        let mut new_clipboard_data = None;
        let mut request_clipboard_data = false;

        let cursor_offset = (0, 0)
            .sub((self.formatting.horizontal_margin as isize, 0))
            .sub((self.x, self.y))
            .sub((0, self.menu_height as isize));

        let mut extra_vertical_offset = self.scroll_offset_y;
        let mut extra_horizontal_offset = self.scroll_offset_x;

        for (idx, (_, module)) in self.modules.iter().enumerate() {
            let height = module.get_height(&self.formatting);
            let position = mouse_pos.map(|input| {
                input
                    .sub((0, offset))
                    .add(cursor_offset)
                    .sub((extra_horizontal_offset, extra_vertical_offset))
            });
            let infos = ModuleInputs {
                focus_taken: gui_in_focus,
                mouse_pos: position,
                mouse_pos_delta,
                mouse_scroll,
                mouse_info: &mouse_data,
                delta_time,
                formatting: &self.formatting,
                pressed_keys,
                clipboard_data,
            };
            let module_output = module.update(&infos);

            // Setting variables based on module output
            // Dragging/Resizing has priority
            if module_output.new_cursor_style.is_some()
                && cursor_style.is_none()
            {
                cursor_style = module_output.new_cursor_style;
            }
            request_clipboard_data =
                request_clipboard_data || module_output.request_clipboard_data;
            hide_cursor = hide_cursor || module_output.hide_cursor;
            gui_in_focus = gui_in_focus || module_output.focus_taken;
            text_input_selected =
                text_input_selected || module_output.text_input_selected;

            if module_output.new_cursor_position.is_some() {
                new_cursor_position = module_output.new_cursor_position;
            }
            if module_output.new_clipboard_data.is_some() {
                new_clipboard_data = module_output.new_clipboard_data;
            }
            offset += height + self.formatting.vertical_margin as isize;

            // We gotta add the "previous_module_offset" at the end so it is previous offset for the next module. That sentence is way more complicated than it should be
            let previous_module_offset_value =
                module.get_next_offset(&self.modules, idx, &self.formatting);

            extra_horizontal_offset += previous_module_offset_value.0;
            extra_vertical_offset += previous_module_offset_value.1;
        }

        if !gui_in_focus
            && let Some(scroll) = mouse_scroll
            && let Some(mouse_pos) = mouse_pos
        {
            let window_hit_box: mirl::math::collision::Rectangle<_, false> =
                mirl::math::collision::Rectangle::new(
                    self.x,
                    self.y,
                    self.width as isize,
                    self.height as isize,
                );
            if window_hit_box.does_area_contain_point(mouse_pos) {
                if pressed_keys.contains(&KeyCode::LeftShift) {
                    self.scroll_offset_x +=
                        scroll.1 * self.horizontal_scroll_multiplier_x;
                    self.scroll_offset_y +=
                        scroll.0 * self.horizontal_scroll_multiplier_y;
                } else {
                    self.scroll_offset_x +=
                        scroll.0 * self.vertical_scroll_multiplier_x;
                    self.scroll_offset_y +=
                        scroll.1 * self.vertical_scroll_multiplier_x;
                }
                // This should be done with a simple if else, why did I decide use a sorted list?
                let size = self.get_size_to_see_all_modules();
                let mut y_range = [0, self.height as isize - size.1];
                y_range.sort_unstable();
                self.scroll_offset_y =
                    self.scroll_offset_y.clamp(y_range[0], y_range[1]);

                let mut x_range = [0, self.width as isize - size.0];
                x_range.sort_unstable();
                self.scroll_offset_x =
                    self.scroll_offset_x.clamp(x_range[0], x_range[1]);
                gui_in_focus = true;
            }
        }

        self.last_left_mouse_down = left_mouse_down;
        self.last_middle_mouse_down = middle_mouse_down;
        self.last_right_mouse_down = right_mouse_down;
        GuiOutput {
            new_cursor_style: cursor_style,
            focus_taken: gui_in_focus,
            new_cursor_position,
            hide_cursor,
            new_clipboard_data,
            text_input_selected,
            request_clipboard_data,
        }
    }

    /// Get the size at which every module is visible
    pub fn get_size_to_see_all_modules(&self) -> (isize, isize) {
        let mut min_width = 0;
        let mut min_height = 0;

        let mut current_pos = (0, 0);
        for (idx, (_, item)) in self.modules.iter().enumerate() {
            let width = item.get_width(&self.formatting);
            let height = item.get_height(&self.formatting);
            let next_offset =
                item.get_next_offset(&self.modules, idx, &self.formatting);

            let t_width = width + current_pos.0;
            let t_height = height + current_pos.1;
            if t_width > min_width {
                min_width = t_width;
            }
            if t_height > min_height {
                min_height = t_height;
            }
            current_pos = current_pos
                .add(next_offset)
                .add((0, height))
                .add((0, self.formatting.vertical_margin).tuple_2_into());
        }

        (
            min_width + self.formatting.horizontal_margin as isize * 2,
            min_height + self.formatting.vertical_margin as isize,
        )
        // FOR TOOLBAR MODULES:
        // let mut min_width = 0;
        // let mut min_height = 0;

        // let mut current_pos = (0, 0);
        // for (idx, (_, item)) in self.modules.iter().enumerate() {
        //     let width = item.get_width(&self.formatting);
        //     let height = item.get_height(&self.formatting);
        //     let next_offset =
        //         item.get_next_offset(&self.modules, idx, &self.formatting);

        //     let t_width = width + current_pos.0;
        //     let t_height = height + current_pos.1;
        //     if t_width > min_width {
        //         min_width = t_width;
        //     }
        //     if t_height > min_height {
        //         min_height = t_height;
        //     }
        //     current_pos = current_pos
        //         .add(next_offset)
        //         .add((width, 0))
        //         .add((self.formatting.horizontal_margin, 0).tuple_2_into());
        // }

        // (min_width , min_height+ self.formatting.vertical_margin as isize * 2)
    }
    /// Set the current size of the window to be able to see all modules
    pub fn set_size_to_see_all_modules(&mut self) {
        let size = self.get_size_to_see_all_modules();
        self.width = size.0.max(self.min_width as isize) as usize;
        self.height =
            size.1.max(self.menu_height as isize) as usize + self.menu_height;
    }
    /// Apply current formatting to modules
    pub fn apply_formatting_to_modules(&mut self) {
        for (_, module) in &mut self.modules {
            module.apply_new_formatting(&self.formatting);
        }
        for (_, module) in &mut self.toolbar_modules {
            module.apply_new_formatting(&self.formatting);
        }
    }
}
