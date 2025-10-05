// Just a little helper lib, it'll take like 3 days
// 1+2: background + dragging + resizing + text + sliders/progress
// 3: refined sliders + buttons
// 4: split sliders/progress bar into own modules + check box
// 5: Trying to compress 6.5k images into a reasonable size
// 6: "
// 7: "
// 8: Gave up. Added better formatting, grouped buttons/radio buttons, line divider, and anti new line
// 9: None
// 10: Alright this is getting too stupid for documentation
// 11: None
// 12: Windows are scrollable, window emulator added, fixed other stuff
// 13: Made scrolling a little more intuitive, de-hardcoded some values bc why not, polished some visuals
// 14: Improved formatting once more, fixed caching which improves performance by 5%-8% in the example window; it's not a lot but sure
// The Next two weeks: Add keybinds, add _all_ keybind functionality
// The week after that: Added new backend
// Week 7: Added crank for rotation, added lever for boolean input. Fixed a bunch of smaller issues

// Well, that took a little longer than expected. 7 Weeks to be exact.
// The lib isn't perfect, or tbh that great overall, but it works for my usecase. And as long as no other person uses this, I have no need to fix it

// Idea: Instead of each module that has a Buffer holding it's own data, there is a global storage each module can request/upload textures to

/// Magic stuff to make all modules work in harmony
pub mod extra;

pub use extra::{ModuleContainer, ModuleVTable};
use mirl::{
    extensions::*,
    misc::{
        corner_type_and_delta_to_metric_change, corner_type_to_cursor_style,
    },
    platform::{Buffer, CursorStyle, keycodes::KeyCode},
    render,
};

use crate::*;

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
    pub modules: Vec<String>,
    /// Toolbar modules - Arranged horizontally instead of vertically
    pub toolbar_modules: Vec<String>,
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
    /// Just the menu height, windows cannot be vertically smaller then this
    pub menu_height: usize,
    /// The minimum menu width - Menus cannot be horizontally smaller than this
    pub min_width: usize,
    /// If the window should be collapsed
    pub collapsed: bool,
    // /// A cache so toolbar modules don't need to redraw themselves every frame
    // pub toolbar_cache: Vec<Option<std::rc::Rc<Buffer>>>,
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
    /// NOT RECOMMENDED TO EDIT
    pub id: usize,
    /// A small cache for the total size - Not always 100% accurate
    pub size_to_see_all_modules: Option<(isize, isize)>,
}
#[allow(clippy::struct_excessive_bools, missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
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
    /// The height of the Menu/Text on the menu -> The colored part above the module area
    pub const DEFAULT_MENU_HEIGHT: usize = 20;
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
        modules: &Option<Vec<String>>,
        toolbar_modules: &Option<Vec<String>>,
        font: &fontdue::Font,
        menu_height: Option<usize>,
        min_width: Option<usize>,
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
            menu_height,
            min_width: min_width.unwrap_or_else(|| {
                mirl::render::get_text_width(title, menu_height as f32, font)
                    as usize
                    + menu_height
            }),
            collapsed,
            collapse_button_collision_is_circle: false,
            needs_redraw: std::cell::Cell::new(true),
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
            id: get_available_id(),

            size_to_see_all_modules: None,
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
        modules: &[String],
    ) -> Self {
        Self {
            title: title.to_string(),
            width,
            height,
            modules: modules.to_vec(),
            toolbar_modules: Vec::new(),
            last_mouse_pos: (0, 0),
            last_left_mouse_down: false,
            last_middle_mouse_down: false,
            last_right_mouse_down: false,
            x,
            y,
            resizing: false,
            at_corner: u8::MAX,
            dragging: false,
            menu_height: Self::DEFAULT_MENU_HEIGHT,
            min_width: render::get_text_width(
                title,
                Self::DEFAULT_MENU_HEIGHT as f32,
                font,
            ) as usize
                + Self::DEFAULT_MENU_HEIGHT,
            collapsed: false,
            collapse_button_collision_is_circle: false,
            needs_redraw: std::cell::Cell::new(false),
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
            id: get_available_id(),
            size_to_see_all_modules: None,
        }
    }
    /// Update the min width based on the length of the window title
    pub fn update_min_width(&mut self) {
        let formatting = get_formatting();
        self.min_width = render::get_text_width(
            &self.title,
            self.menu_height as f32,
            &formatting.font,
        ) as usize
            + self.menu_height;
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

    /// Add a module with type preservation
    pub fn add_module<T: DearMirlGuiModule + 'static, Type>(
        &mut self,
        name: &ModulePath<Type>,
    ) {
        self.modules.push(name.id());
        self.size_to_see_all_modules = None;
    }
    /// Remove a module
    pub fn remove_module(&mut self, name: &String) {
        let index = self.modules.find(name);
        if let Some(idx) = index {
            self.modules.remove(idx);
        }
        self.size_to_see_all_modules = None;
    }

    /// Add a module with type preservation to the toolbar
    pub fn add_toolbar_module(&mut self, name: &str) {
        self.toolbar_modules.push(name.to_string());
    }
    /// Remove a toolbar module
    pub fn remove_toolbar_module(&mut self, name: &String) {
        let index = self.toolbar_modules.find(name);
        if let Some(idx) = index {
            self.toolbar_modules.remove(idx);
        }
    }

    /// Automatically draw the window onto a `mirl::platform::Buffer`
    ///
    /// If nothing is showing up, maybe check the size if the gui
    pub fn draw_on_buffer(&mut self, buffer: &Buffer) {
        #[cfg(feature = "draw_debug")]
        println!("Before Drawing");
        let to_draw = self.render();
        #[cfg(feature = "draw_debug")]
        println!(
            "x{} y{} w{} h{}",
            self.x, self.y, to_draw.width, to_draw.height
        );
        if buffer
            .create_collision_isize::<false>(0, 0)
            .does_area_fully_include_other_area(
                &to_draw.create_collision_isize(self.x, self.y),
            )
        {
            render::draw_buffer_on_buffer_1_to_1::<false, false, false, false>(
                buffer,
                &to_draw,
                (self.x, self.y),
            );
        } else {
            render::draw_buffer_on_buffer_1_to_1::<true, false, false, false>(
                buffer,
                &to_draw,
                (self.x, self.y),
            );
        }
    }
    fn draw_menu(
        &self,
        buffer: &Buffer,
        font: &fontdue::Font,
        collapse_button_size: f64,
        collapse_button_color_change: f32,
    ) {
        const DRAW_MENU_ID: bool = true;
        let formatting = get_formatting();
        render::draw_rectangle::<{ crate::DRAW_SAFE }>(
            buffer,
            0,
            0,
            buffer.width as isize,
            self.menu_height as isize,
            formatting.foreground_color,
        );
        render::draw_text_antialiased_isize::<{ crate::DRAW_SAFE }>(
            buffer,
            &self.title,
            (
                formatting.horizontal_margin as isize
                    + self.menu_height as isize,
                -(formatting.vertical_margin as isize),
            ),
            formatting.text_color,
            (self.menu_height - formatting.vertical_margin) as f32,
            font,
        );

        let h = self.menu_height as isize / 2;
        render::draw_circle::<false, false>(
            buffer,
            h,
            h,
            (h as f64 * collapse_button_size) as isize,
            mirl::graphics::adjust_brightness_hsl_of_rgb(
                formatting.foreground_color,
                collapse_button_color_change,
            ), // THIS CRASHES WHEN THE CURRENT IS SMALLER THAN THE MENU HEIGHT
        );
        if DRAW_MENU_ID {
            render::draw_text_antialiased_isize::<true>(
                buffer,
                &format!(" {}", self.id),
                (
                    formatting.horizontal_margin as isize
                        + self.menu_height as isize
                        + mirl::render::get_text_width(
                            &self.title,
                            (self.menu_height - formatting.vertical_margin)
                                as f32,
                            font,
                        ) as isize,
                    -(formatting.vertical_margin as isize),
                ),
                formatting.text_color,
                (self.menu_height - formatting.vertical_margin) as f32,
                font,
            );
        }
    }
    #[must_use]
    #[allow(clippy::too_many_lines)]
    /// Use [`mirl::render::draw_buffer_on_buffer`](mirl::render::draw_buffer_on_buffer) (or [`draw_buffer_on_buffer_1_to_1`](mirl::render::draw_buffer_on_buffer_1_to_1)) to draw this buffer on any other
    pub fn render(&mut self) -> Buffer {
        let horizontal_context = false;
        #[cfg(feature = "draw_debug")]
        println!("Entered Draw");
        self.needs_redraw.set(false);
        if self.height < self.menu_height {
            return Buffer::generate_fallback(self.width, self.height, 4);
        }
        let collapse_button_size = 0.8;
        let collapse_button_color_change = -10.0;

        let formatting = get_formatting();

        let buffer = Buffer::new_empty_with_color(
            self.width,
            if self.collapsed {
                self.menu_height
            } else {
                self.height
            },
            formatting.background_color,
        );
        #[cfg(feature = "draw_debug")]
        println!("Created buffer");
        if self.collapsed {
            self.draw_menu(
                &buffer,
                &formatting.font,
                collapse_button_size,
                collapse_button_color_change,
            );
            return buffer;
        }

        let collision = buffer.create_collision_isize::<false>(0, 0);
        #[cfg(feature = "draw_debug")]
        println!("Starting on drawing modules");

        // Using Rc<Buffer> instead of Buffer, without caching, increased testing fps from a stable 62 to a stable 63 - What
        // let mut buffers = Vec::new();

        let info = ModuleDrawInfo {
            container_id: self.id,
        };
        let mut module_idx_cache = Vec::new();

        let static_vertical_offset = self.scroll_offset_y
            + (self.menu_height + formatting.horizontal_margin) as isize;
        let static_horizontal_offset = self.scroll_offset_x;

        let mut extra_vertical_offset = 0;
        let mut extra_horizontal_offset = 0;

        if let Ok(modules) = MODULES.read() {
            for module_name in &self.modules {
                let Some(module_idx) = get_idx_of_id(module_name) else {
                    continue;
                };
                module_idx_cache.push(module_idx);
                let module_container = &modules[module_idx];
                #[cfg(feature = "draw_debug")]
                println!(
                    "Currently working on {:?}",
                    mirl::misc::find_key_by_value(
                        &MODULE_INDEX.read().unwrap(),
                        &idx
                    )
                );
                let buf: (std::sync::Arc<Buffer>, InsertionMode);
                let insert_into_cache: bool;

                if USE_CACHE {
                    if module_container.need_redraw() {
                        let drawn_buffer =
                            module_container.draw(&formatting, &info);
                        buf = (
                            std::sync::Arc::new(drawn_buffer.0),
                            drawn_buffer.1,
                        );
                        insert_into_cache = true;
                    } else if let Some(cached_buf) =
                        get_image_cache(module_idx, self.id)
                    {
                        insert_into_cache = false;
                        buf = (cached_buf, InsertionMode::Simple);
                    } else {
                        let drawn_buffer =
                            module_container.draw(&formatting, &info);
                        buf = (
                            std::sync::Arc::new(drawn_buffer.0),
                            drawn_buffer.1,
                        );
                        insert_into_cache = true;
                    }
                } else {
                    let drawn_buffer =
                        module_container.draw(&formatting, &info);
                    buf = (std::sync::Arc::new(drawn_buffer.0), drawn_buffer.1);
                    insert_into_cache = true;
                }

                // if buf.1.2 {
                //     extra_horizontal_offset = buf.1.0;
                //     extra_vertical_offset = module_container
                //         .get_height(&formatting)
                //         + formatting.vertical_margin as isize
                //         + buf.1.1;
                // } else {
                //     extra_horizontal_offset += buf.1.0;
                //     extra_vertical_offset += module_container
                //         .get_height(&formatting)
                //         + formatting.vertical_margin as isize
                //         + buf.1.1;
                // }
                let x = formatting.horizontal_margin as isize
                    + extra_horizontal_offset
                    + static_horizontal_offset;
                let y = extra_vertical_offset + static_vertical_offset;

                let col = buf.0.create_collision_isize::<false>(x, y);
                let position = (x, y);

                if collision.does_area_fully_include_other_area(&col) {
                    if FAST {
                        render::draw_buffer_on_buffer_1_to_1::<
                            false,
                            true,
                            false,
                            false,
                        >(&buffer, &buf.0, position);
                    } else {
                        render::draw_buffer_on_buffer_1_to_1::<
                            false,
                            true,
                            true,
                            false,
                        >(&buffer, &buf.0, position);
                    }
                } else if FAST {
                    render::draw_buffer_on_buffer_1_to_1::<
                        true,
                        true,
                        false,
                        false,
                    >(&buffer, &buf.0, position);
                } else {
                    render::draw_buffer_on_buffer_1_to_1::<
                        true,
                        true,
                        true,
                        false,
                    >(&buffer, &buf.0, position);
                }
                if horizontal_context {
                    extra_horizontal_offset +=
                        module_container.get_width(&formatting);
                } else {
                    extra_vertical_offset += module_container
                        .get_height(&formatting)
                        + formatting.vertical_margin as isize;
                }

                module_container.modify_offset_cursor(
                    &modules,
                    &module_idx_cache,
                    &formatting,
                    (&mut extra_horizontal_offset, &mut extra_vertical_offset),
                );
                if insert_into_cache {
                    insert_into_image_cache(
                        module_idx,
                        self.id,
                        (*buf.0).clone(),
                        buf.1,
                    );
                }
            }
        }
        // #[cfg(feature = "draw_debug")]
        // println!("Done with drawing modules");

        #[cfg(feature = "draw_debug")]
        println!("4");
        self.draw_menu(
            &buffer,
            &formatting.font,
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
        mouse_info: &MouseData,
        over_collapse_button: bool,
    ) -> (Option<CursorStyle>, FocusTaken) {
        let mut cursor_style = None;
        let mut gui_in_focus = FocusTaken::FocusFree;
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
                gui_in_focus = FocusTaken::VisuallyTaken;
            }
            if (self.dragging && mouse_info.left.down)
                || (mouse_info.left.clicked
                    && collides
                    && !self.resizing
                    && !over_collapse_button)
            {
                gui_in_focus = FocusTaken::FunctionallyTaken;
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
        mouse_info: &MouseData,
        over_collapse_button: bool,
    ) -> (Option<CursorStyle>, FocusTaken) {
        let mut cursor_style = None;
        let mut gui_in_focus = FocusTaken::FocusFree;
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
                    return (None, FocusTaken::FocusFree);
                }
                // Do not show the option to drag if over the collapse button, that is so annoying
                if at_corner != u8::MAX
                    && (self.resizing || !over_collapse_button)
                {
                    cursor_style = corner_type_to_cursor_style(at_corner);
                    gui_in_focus = FocusTaken::VisuallyTaken;
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
                        gui_in_focus = FocusTaken::FunctionallyTaken;
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
    /// Intended to be used when updating multi GUIs in recession
    pub fn update_using_module_data(
        &mut self,
        module_input: ModuleUpdateInfo,
        module_outputs: &GuiOutput,
    ) -> GuiOutput {
        self.internal_update(module_input, module_outputs)
    }
    /// Please only provide the clipboard data when it is being requested
    ///
    /// You may lie about the input metrics however you like, if anything crashes, please report such to whomever is maintaining the gui or modules you used based on what crashed
    #[must_use]
    #[allow(clippy::too_many_arguments)]
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
        let mouse_pos_delta =
            mouse_pos.unwrap_or((0, 0)).sub(self.last_mouse_pos);

        self.internal_update(
            ModuleUpdateInfo {
                focus_taken: FocusTaken::FocusFree,
                mouse_pos,
                real_mouse_pos: mouse_pos,
                mouse_pos_delta,
                mouse_scroll,
                mouse_info: &mouse_data,
                pressed_keys,
                delta_time,
                clipboard_data,
                container_id: self.id,
            },
            &GuiOutput::empty(),
        )
    }
    // /// Creates a new module input instance
    // #[must_use]
    // #[allow(clippy::too_many_arguments)]
    // pub fn get_module_input<'a>(
    //     &mut self,
    //     mouse_pos: Option<(isize, isize)>,
    //     mouse_scroll: Option<(isize, isize)>,
    //     left_mouse_down: bool,
    //     middle_mouse_down: bool,
    //     right_mouse_down: bool,
    //     pressed_keys: &'a Vec<KeyCode>,
    //     delta_time: f64,
    //     clipboard_data: &'a Option<mirl::platform::file_system::FileData>,
    // ) -> ModuleInputs<'a> {
    //     let mouse_data = MouseData {
    //         left: ButtonState::new(left_mouse_down, self.last_left_mouse_down),
    //         middle: ButtonState::new(
    //             middle_mouse_down,
    //             self.last_middle_mouse_down,
    //         ),
    //         right: ButtonState::new(
    //             right_mouse_down,
    //             self.last_right_mouse_down,
    //         ),
    //     };
    //     let mouse_pos_delta =
    //         mouse_pos.unwrap_or((0, 0)).sub(self.last_mouse_pos);

    //     ModuleInputs {
    //         focus_taken: false,
    //         mouse_pos,
    //         real_mouse_pos: mouse_pos,
    //         mouse_pos_delta,
    //         mouse_scroll,
    //         mouse_info: & mouse_data,
    //         pressed_keys,
    //         delta_time,
    //         clipboard_data,
    //     }
    // }
    /// Get the size at which every module is visible from cache if possible
    pub fn get_size_to_see_all_modules_caches(&self) -> (isize, isize) {
        self.size_to_see_all_modules
            .map_or_else(|| self.get_size_to_see_all_modules(), |s| s)
    }

    /// Get the size at which every module is visible
    pub fn get_size_to_see_all_modules(&self) -> (isize, isize) {
        let mut min_width = 0;
        let mut min_height = 0;
        let horizontal_context = false;

        let mut extra_vertical_offset = 0;
        let mut extra_horizontal_offset = 0;

        let mut used_idx = Vec::new();
        let formatting = get_formatting();

        if let Ok(modules) = MODULES.read() {
            for module_name in &self.modules {
                let Some(module_idx) = get_idx_of_id(module_name) else {
                    continue;
                };
                used_idx.push(module_idx); // Fixed: should be module_idx, not idx
                let module_container = &modules[module_idx];

                // Calculate the actual position where this module would be drawn
                let x = formatting.horizontal_margin as isize
                    + extra_horizontal_offset;
                let y = extra_vertical_offset;

                // Calculate the total width and height this module would occupy
                let module_width = module_container.get_width(&formatting);
                let module_height = module_container.get_height(&formatting);

                let total_width = x + module_width;
                let total_height = y + module_height;

                if total_width > min_width {
                    min_width = total_width;
                }
                if total_height > min_height {
                    min_height = total_height;
                }

                // Update offsets for next module (matching the renderer logic)
                if horizontal_context {
                    extra_horizontal_offset += module_width;
                } else {
                    extra_vertical_offset +=
                        module_height + formatting.vertical_margin as isize;
                }

                module_container.modify_offset_cursor(
                    &modules,
                    &used_idx,
                    &formatting,
                    (&mut extra_horizontal_offset, &mut extra_vertical_offset),
                );
            }
        }

        // Add final margins
        (
            min_width + formatting.horizontal_margin as isize,
            min_height + formatting.vertical_margin as isize * 2,
        )

        // FOR TOOLBAR MODULES:
        // let mut min_width = 0;
        // let mut min_height = 0;

        // let mut current_pos = (0, 0);
        // for (idx, (_, item)) in self.modules.iter().enumerate() {
        //     let width = item.get_width(&formatting);
        //     let height = item.get_height(&formatting);
        //     let next_offset =
        //         item.get_next_offset(&self.modules, idx, &formatting);

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
        //         .add((formatting.horizontal_margin, 0).tuple_2_into());
        // }

        // (min_width , min_height+ formatting.vertical_margin as isize * 2)
    }
    /// Set the current size of the window to be able to see all modules
    pub fn set_size_to_see_all_modules(&mut self) {
        let size = self.get_size_to_see_all_modules();
        self.width = size.0.max(self.min_width as isize) as usize;
        self.height =
            size.1.max(self.menu_height as isize) as usize + self.menu_height;
    }
    #[allow(clippy::too_many_lines)] // Well, clippy... it's just... big. :(
    fn internal_update(
        &mut self,
        module_input: ModuleUpdateInfo,
        module_outputs: &GuiOutput,
    ) -> GuiOutput {
        let horizontal_context = false;
        let mut module_input = module_input;
        module_input.container_id = self.id;

        // There are so many checks for if mouse_pos isn't None, who wrote this???

        // You fool, it was I!

        // Roleplaying with yourself but with such a timely gap that it feels like there are multiple people working on this project

        let mut cursor_style = module_outputs.new_cursor_style;
        let mut gui_in_focus = module_input.focus_taken;
        if !gui_in_focus {
            //println!(">>>{:?}", gui_in_focus);
            let mut over_collapse_button = false;

            let collapse_button_collision =
                if let Some(current_mouse_pos) = module_input.mouse_pos {
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
                if module_input.mouse_info.left.clicked {
                    self.collapsed = !self.collapsed;
                    gui_in_focus |= FocusTaken::FunctionallyTaken;
                } else {
                    gui_in_focus |= FocusTaken::VisuallyTaken;
                }
            }

            // If the mouse position is invalid, reset it
            if self.last_mouse_pos == (isize::MIN, isize::MIN)
                && let Some(current_mouse_pos) = module_input.mouse_pos
            {
                self.last_mouse_pos = current_mouse_pos;
            }

            if let Some(current_mouse_pos) = module_input.mouse_pos {
                self.last_mouse_pos = current_mouse_pos;
            }

            // Dragging has priority, not because it should but because it's ordered like this

            // Handle dragging
            if self.allow_dragging {
                let dragging_output = self.handle_dragging(
                    module_input.mouse_pos,
                    module_input.mouse_pos_delta,
                    module_input.mouse_info,
                    over_collapse_button,
                );
                if dragging_output.0.is_some() && cursor_style.is_none() {
                    cursor_style = dragging_output.0;
                }
                #[cfg(feature = "focus_debug")]
                if gui_in_focus < dragging_output.1 {
                    println!(
                        "Dragging increased focus: {:?}",
                        dragging_output.1
                    );
                }
                gui_in_focus |= dragging_output.1;
            }
            // Handle resizing
            {
                let resizing_output = self.handle_resizing(
                    module_input.mouse_pos,
                    module_input.mouse_pos_delta,
                    module_input.mouse_info,
                    over_collapse_button,
                );
                if resizing_output.0.is_some() && cursor_style.is_none() {
                    cursor_style = resizing_output.0;
                }
                #[cfg(feature = "focus_debug")]
                if gui_in_focus < resizing_output.1 {
                    println!(
                        "Resizing increased focus: {:?}",
                        resizing_output.1
                    );
                }

                gui_in_focus |= resizing_output.1;
            }
        }
        let formatting = get_formatting();

        let mut hide_cursor = false;
        let mut text_input_selected = false;
        let mut new_cursor_position = None;
        let mut new_clipboard_data = None;
        let mut request_clipboard_data = false;

        let cursor_offset = (0, 0)
            .sub((formatting.horizontal_margin as isize, 0))
            .sub((self.x, self.y))
            .sub((0, self.menu_height as isize));

        let static_vertical_offset =
            self.scroll_offset_y + formatting.horizontal_margin as isize;
        let static_horizontal_offset = self.scroll_offset_x;

        let mut extra_vertical_offset = 0;
        let mut extra_horizontal_offset = 0;

        let mut module_idx_cache = Vec::new();
        if let Ok(modules) = MODULES.read() {
            for module_name in &self.modules {
                let Some(module_idx) = get_idx_of_id(module_name) else {
                    continue;
                };
                let module = &modules[module_idx];
                module_idx_cache.push(module_idx);

                //let height = module.get_height(&formatting);
                let position = module_input.real_mouse_pos.map(|input| {
                    input
                        .add(cursor_offset)
                        .sub((extra_horizontal_offset, extra_vertical_offset))
                        .sub((static_horizontal_offset, static_vertical_offset))
                });
                module_input.mouse_pos = position;

                // let infos = ModuleInputs {
                //     focus_taken: gui_in_focus,
                //     mouse_pos: position,
                //     mouse_pos_delta,
                //     mouse_scroll,
                //     mouse_info: &mouse_data,
                //     delta_time,
                //     formatting: &formatting,
                //     pressed_keys,
                //     clipboard_data,
                // };
                let module_output = module.update(&module_input);

                // Setting variables based on module output
                request_clipboard_data = request_clipboard_data
                    || module_output.request_clipboard_data;
                hide_cursor = hide_cursor || module_output.hide_cursor;
                if gui_in_focus < module_output.focus_taken {
                    // Dragging/Resizing has visual priority IF no module is taking functional focus -> It just looks nicer
                    if module_output.new_cursor_style.is_some() {
                        cursor_style = module_output.new_cursor_style;
                    }
                    #[cfg(feature = "focus_debug")]
                    println!(
                        "Module {:?} increased focus: from {:?} to {:?}",
                        mirl::misc::find_key_by_value(
                            &MODULE_INDEX.read().unwrap(),
                            &module_idx
                        ),
                        gui_in_focus,
                        module_output.focus_taken
                    );
                }
                gui_in_focus |= module_output.focus_taken;
                module_input.focus_taken = gui_in_focus;
                text_input_selected =
                    text_input_selected || module_output.text_input_selected;

                if module_output.new_cursor_position.is_some() {
                    new_cursor_position = module_output.new_cursor_position;
                }
                if module_output.new_clipboard_data.is_some() {
                    new_clipboard_data = module_output.new_clipboard_data;
                }

                if horizontal_context {
                    extra_horizontal_offset += module.get_width(&formatting);
                } else {
                    extra_vertical_offset += module.get_height(&formatting)
                        + formatting.vertical_margin as isize;
                }
                module.modify_offset_cursor(
                    &modules[..],
                    &module_idx_cache,
                    &formatting,
                    (&mut extra_horizontal_offset, &mut extra_vertical_offset),
                );
            }
        }

        if !gui_in_focus.is_focus_taken()
            && let Some(mouse_pos) = module_input.real_mouse_pos
        {
            let window_hit_box: mirl::math::collision::Rectangle<_, true> =
                mirl::math::collision::Rectangle::new(
                    self.x,
                    self.y,
                    self.get_width() as isize,
                    self.get_height() as isize,
                );
            if window_hit_box.does_area_contain_point(mouse_pos) {
                if let Some(scroll) = module_input.mouse_scroll
                    && scroll != (0, 0)
                {
                    if module_input.pressed_keys.contains(&KeyCode::LeftShift) {
                        self.scroll_offset_x +=
                            scroll.1 * self.horizontal_scroll_multiplier_x;
                        self.scroll_offset_y +=
                            scroll.0 * self.horizontal_scroll_multiplier_y;
                    } else {
                        self.scroll_offset_x +=
                            scroll.0 * self.vertical_scroll_multiplier_x;
                        self.scroll_offset_y +=
                            scroll.1 * self.vertical_scroll_multiplier_y;
                    }
                    // This should be done with a simple if else, why did I decide use a sorted list?
                    let size = self.get_size_to_see_all_modules();
                    let mut y_range = [
                        0,
                        self.height as isize
                            - size.1
                            - self.menu_height as isize,
                    ];
                    y_range.sort_unstable();
                    self.scroll_offset_y =
                        self.scroll_offset_y.clamp(y_range[0], y_range[1]);

                    let mut x_range = [0, self.width as isize - size.0];
                    x_range.sort_unstable();
                    self.scroll_offset_x =
                        self.scroll_offset_x.clamp(x_range[0], x_range[1]);
                    gui_in_focus = FocusTaken::FunctionallyTaken;
                } else {
                    // This is if the cursor is on top on the gui but not interacting with anything
                    if module_input.mouse_info.left.clicked {
                        gui_in_focus = FocusTaken::FunctionallyTaken;
                    } else {
                        gui_in_focus = FocusTaken::VisuallyTaken;
                    }
                }
            }
        }

        self.last_left_mouse_down = module_input.mouse_info.left.down;
        self.last_middle_mouse_down = module_input.mouse_info.middle.down;
        self.last_right_mouse_down = module_input.mouse_info.right.down;
        #[cfg(any(feature = "draw_debug", feature = "focus_debug"))]
        if gui_in_focus == FocusTaken::FunctionallyTaken {
            cursor_style = Some(CursorStyle::Cell);
        }
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
}
