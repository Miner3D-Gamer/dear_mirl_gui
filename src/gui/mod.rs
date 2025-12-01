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
// The lib isn't perfect, or tbh that great overall, but it works for my use case. And as long as no other person uses this, I have no need to fix it

// Idea: Instead of each module that has a Buffer holding it's own data, there is a global storage each module can request/upload textures to

/// Magic stuff to make all modules work in harmony
pub mod extra;

pub use extra::ModuleContainer;
use mirl::{
    directions::NormalDirections,
    directions::misc::{
        corner_type_and_delta_to_metric_change, corner_type_to_cursor_style,
    },
    extensions::*,
    misc::keybinds::KeyBind,
    platform::{Buffer, CursorStyle, keycodes::KeyCode},
    render,
};

use crate::*;
#[derive(Debug, Clone, PartialEq)]
/// Actions that the gui can execute upon request
pub enum Actions {
    /// Selects the next module (below), if no module is selected: Selects the first module
    SelectNextModule {
        /// If the selection should wrap back to the end if at the top
        wrap: bool,
        /// How many modules to cycle forwards -> Use 1 to select the next module
        skips: usize,
    },
    /// Selects the previous module (above), if no module is selected: Selects the last module
    SelectPreviousModule {
        /// If the selection should wrap back to the top if at the end
        wrap: bool,
        /// How many modules to cycle backward -> Use 1 to select the previous module
        skips: usize,
    },
    /// Re-selects the last selected module if no module is selected. If there is no last module it will select the first one
    RestoreSelected,
    /// Deselects current module
    ClearModuleSelection,
    /// Sets the module index to the specified idx
    ///
    /// For a `SelectFirstModule`, set this to 0
    GoToModule(usize),
    /// Selects the last module in the gui
    SelectLastModule,
    /// Sets the module index to the specified module id -> If it fails to find the id it will deselect
    SelectModule(String),
    /// Set the camera position to the top most position
    ScrollToTop,
    /// Set the camera position to the bottom most position
    ScrollToBottom,
    /// Set the camera position to the right most position
    ScrollToRight,
    /// Set the camera position to the left most position
    ScrollToLeft,
    /// Activate/Deactivate a virtual cursor (None = Toggle, true = enable, false = disable) for when a module does not support keyboard only
    VirtualCursorToggle(Option<bool>),
    /// Higher number means the camera moves right and the modules go left
    VirtualCursorMoveVertically(f32),
    /// Higher number means the camera descends and the modules move up
    VirtualCursorMoveHorizontally(f32),
    /// Set the current virtual cursor position
    VirtualCursorSetPosition {
        /// The horizontal axis, left/right
        x: f32,
        /// The vertical axis, up/down
        y: f32,
    },
    /// Simulates a left mouse click (mouse down for x frames)
    VirtualCursorLeftMouseClick(usize),
    /// Simulates a right mouse click (mouse down for x frames)
    VirtualCursorRightMouseClick(usize),
    /// Simulates a middle mouse click (mouse down for x frame)
    VirtualCursorMiddleMouseClick(usize),
    /// Simulates a holding down the left mouse button
    VirtualCursorLeftMouseToggle,
    /// Simulates a holding down the right mouse button
    VirtualCursorRightMouseToggle,
    /// Simulates a holding down the middle mouse button
    VirtualCursorMiddleMouseToggle,
    /// Toggle the collapsed (None = Toggle, true = collapse, false = uncollapse)
    ToggleCollapse(Option<bool>),
    /// Puts the selected module inside the camera
    CenterCameraOnSelected,
    ///  Higher number means the camera descends and the modules move up
    ScrollVertically(f32),
    /// Higher number means the camera moves right and the modules go left
    ScrollHorizontally(f32),
}

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
    pub x: crate::DearMirlGuiCoordinateType,
    /// Y position on screen
    pub y: crate::DearMirlGuiCoordinateType,
    /// Added modules
    pub modules: Vec<u32>,
    /// Toolbar modules - Arranged horizontally instead of vertically
    pub toolbar_modules: Vec<u32>,
    /// Last saved mouse position - is [`(crate::DearMirlGuiCoordinateType::MIN, crate::DearMirlGuiCoordinateType::MIN)`](crate::DearMirlGuiCoordinateType::MIN) when invalidated
    pub last_mouse_pos: (i32, i32),
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
    pub resizing_allowed_in_directions: NormalDirections,
    /// If you're allowed to drag the window
    pub allow_dragging: bool,
    /// Horizontal scroll offset
    pub camera: mirl::misc::ScrollableCamera,
    /// NOT RECOMMENDED TO EDIT
    pub id: usize,
    /// A small cache for the total size - Not always 100% accurate
    pub size_to_see_all_modules: Option<(
        crate::DearMirlGuiCoordinateType,
        crate::DearMirlGuiCoordinateType,
    )>,
    /// A set of keybinds for non mouse input purposes
    pub keybinds: Vec<KeyBind<Actions>>,
}

impl<const FAST: bool, const USE_CACHE: bool> DearMirlGui<FAST, USE_CACHE> {
    /// The height of the Menu/Text on the menu -> The colored part above the module area
    pub const DEFAULT_MENU_HEIGHT: usize = 20;
    /// By how much the X part of the mouse scroll should be multiplied with when scrolling vertically
    pub const DEFAULT_VERTICAL_SCROLL_X_MULTIPLIER: f32 = 2.0;
    /// By how much the Y part of the mouse scroll should be multiplied with when scrolling vertically
    pub const DEFAULT_VERTICAL_SCROLL_Y_MULTIPLIER: f32 = 1.0;
    /// If the content inside the window should be scrollable in the window is bigger than the content
    pub const DEFAULT_FREE_SCROLL: bool = true;
    /// The variable is very literal in meaning
    pub const HORIZONTAL_CONTEXT_SWITCHES_CAMERA_SCROLL_MULTIPLIERS: bool =
        true;
    /// When the content is smaller than the container, should the content be scrollable?
    pub const ALLOW_FREE_SCROLL: bool = true;
    #[must_use]
    #[allow(clippy::too_many_arguments)]
    /// Create a new `DearMirlGui` window
    /// Just clone the `fontdue::Font`
    pub fn new_advanced(
        title: &str,
        x: crate::DearMirlGuiCoordinateType,
        y: crate::DearMirlGuiCoordinateType,
        width: usize,
        height: usize,
        modules: Vec<u32>,
        toolbar_modules: Vec<u32>,
        font: &fontdue::Font,
        menu_height: Option<usize>,
        min_width: Option<usize>,
        allow_resizing_in_directions: Option<NormalDirections>,
        allow_dragging: bool,
        collapsed: bool,
        scroll_multiplier_x: Option<f32>,
        scroll_multiplier_y: Option<f32>,
        keybinds: Vec<KeyBind<Actions>>,
        horizontal_context_switches_camera_scroll_multipliers: Option<bool>,
        allow_free_scrolling: Option<bool>,
    ) -> Self {
        let menu_height = menu_height.unwrap_or(Self::DEFAULT_MENU_HEIGHT);
        let mut gui = Self {
            title: title.to_string(),
            width,
            height,
            modules: Vec::new(),
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
                .unwrap_or(NormalDirections::all_true()),
            allow_dragging,
            camera: mirl::misc::ScrollableCamera {
                container_width: width as f32,
                container_height: height as f32,
                content_width: 0.0,
                content_height: 0.0,
                offset_x: 0.0,
                offset_y: 0.0,
                scroll_multiplier_x: scroll_multiplier_x
                .unwrap_or(Self::DEFAULT_VERTICAL_SCROLL_X_MULTIPLIER),
                scroll_multiplier_y: scroll_multiplier_y
                .unwrap_or(Self::DEFAULT_VERTICAL_SCROLL_Y_MULTIPLIER),
                horizontal_context_switch_multipliers: horizontal_context_switches_camera_scroll_multipliers.unwrap_or(Self::HORIZONTAL_CONTEXT_SWITCHES_CAMERA_SCROLL_MULTIPLIERS),
                allow_free_scroll: allow_free_scrolling.unwrap_or(Self::ALLOW_FREE_SCROLL),
            },
            id: get_available_id(),

            size_to_see_all_modules: None,
            keybinds,
        };

        for module in modules {
            gui.add_module(module);
        }
        for module in toolbar_modules {
            gui.add_toolbar_module(module);
        }
        gui
    }

    #[must_use]
    #[allow(clippy::too_many_arguments)]
    /// Create a new `DearMirlGui` window - use [`Self::new_advanced`] for more control
    pub fn new_simple(
        title: &str,
        pos: (
            crate::DearMirlGuiCoordinateType,
            crate::DearMirlGuiCoordinateType,
        ),
        modules: &[u32],
    ) -> Self {
        let mut gui = Self {
            title: title.to_string(),
            width: 0,
            height: 0,
            modules: Vec::new(),
            toolbar_modules: Vec::new(),
            last_mouse_pos: (0, 0),
            last_left_mouse_down: false,
            last_middle_mouse_down: false,
            last_right_mouse_down: false,
            x: pos.0,
            y: pos.1,
            resizing: false,
            at_corner: u8::MAX,
            dragging: false,
            menu_height: Self::DEFAULT_MENU_HEIGHT,
            min_width: render::get_text_width(
                title,
                Self::DEFAULT_MENU_HEIGHT as f32,
                &get_formatting().font,
            ) as usize
                + Self::DEFAULT_MENU_HEIGHT,
            collapsed: false,
            collapse_button_collision_is_circle: false,
            needs_redraw: std::cell::Cell::new(false),
            allow_dragging: true,
            resizing_allowed_in_directions: NormalDirections::all_true(),
            camera: mirl::misc::ScrollableCamera {
                container_width: 0.0,
                container_height: 0.0,
                content_width: 0.0,
                content_height: 0.0,
                offset_x: 0.0,
                offset_y: 0.0,
                scroll_multiplier_x: (Self::DEFAULT_VERTICAL_SCROLL_X_MULTIPLIER),
                scroll_multiplier_y: (Self::DEFAULT_VERTICAL_SCROLL_Y_MULTIPLIER),
                horizontal_context_switch_multipliers: (Self::HORIZONTAL_CONTEXT_SWITCHES_CAMERA_SCROLL_MULTIPLIERS),
                allow_free_scroll: (Self::ALLOW_FREE_SCROLL),
            },
            id: get_available_id(),
            size_to_see_all_modules: None,
            keybinds: get_default_keybinds(),
        };
        for module in modules {
            gui.add_module(*module);
        }
        gui.set_size_to_see_all_modules();
        gui
    }
    /// Update the min width based on the length of the window title
    pub fn update_min_width(&mut self) {
        let formatting = get_formatting();
        self.min_width = render::get_text_width(
            &self.title,
            self.menu_height as f32,
            &formatting.font,
        ) as usize
            + self.menu_height
            + formatting.horizontal_margin * 3;
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
    pub fn add_module(&mut self, name: u32) {
        add(&mut self.modules, name, self.id);
        self.size_to_see_all_modules = None;
    }
    /// Remove a module
    pub fn remove_module(&mut self, name: u32) {
        remove(&mut self.modules, name, self.id);
        self.size_to_see_all_modules = None;
    }

    /// Add a module with type preservation to the toolbar
    pub fn add_toolbar_module(&mut self, name: u32) {
        add(&mut self.toolbar_modules, name, self.id);
        self.size_to_see_all_modules = None;
    }
    /// Remove a toolbar module
    pub fn remove_toolbar_module(&mut self, name: u32) {
        remove(&mut self.toolbar_modules, name, self.id);
        self.size_to_see_all_modules = None;
    }

    /// Automatically draw the window onto a `mirl::platform::Buffer`
    ///
    /// If nothing is showing up, maybe check the size if the gui
    pub fn draw_on_buffer(&mut self, buffer: &mut Buffer) -> Option<()> {
        #[cfg(feature = "draw_debug")]
        println!("Before Drawing");
        let to_draw = self.render();
        #[cfg(feature = "draw_debug")]
        println!(
            "x{} y{} w{} h{}",
            self.x, self.y, to_draw.width, to_draw.height
        );
        if buffer
            .create_collision::<false, crate::DearMirlGuiCoordinateType>(0, 0)?
            .does_area_fully_include_other_area(
                &to_draw.create_collision::<false, _>(self.x, self.y)?,
            )
        {
            render::draw_buffer_on_buffer_1_to_1::<false, false, false, false>(
                buffer,
                &to_draw,
                (self.x, self.y).try_tuple_into()?,
            );
        } else {
            render::draw_buffer_on_buffer_1_to_1::<true, false, false, false>(
                buffer,
                &to_draw,
                (self.x, self.y).try_tuple_into()?,
            );
        }
        Some(())
    }
    fn draw_menu(
        &self,
        buffer: &mut Buffer,
        font: &fontdue::Font,
        collapse_button_size: f64,
        collapse_button_color_change: f32,
    ) {
        let formatting = get_formatting();
        render::draw_rectangle::<{ crate::DRAW_SAFE }>(
            buffer,
            (0, 0),
            (buffer.width as isize, self.menu_height as isize),
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
            (h, h),
            (h as f64 * collapse_button_size) as isize,
            mirl::graphics::adjust_brightness_hsl_of_rgb(
                formatting.foreground_color,
                collapse_button_color_change,
            ), // THIS CRASHES WHEN THE CURRENT IS SMALLER THAN THE MENU HEIGHT
        );

        #[cfg(feature = "debug-window")]
        {
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
    /// Use [`draw_buffer_on_buffer`](mirl::render::draw_buffer_on_buffer) (or [`draw_buffer_on_buffer_1_to_1`](mirl::render::draw_buffer_on_buffer_1_to_1)) to draw this buffer on any other
    pub fn render(&mut self) -> Buffer {
        let horizontal_context = false;
        #[cfg(feature = "draw_debug")]
        println!("Entered Draw");
        self.needs_redraw.set(false);
        if self.height < self.menu_height {
            return Buffer::generate_fallback((self.width, self.height), 4);
        }
        let collapse_button_size = 0.8;
        let collapse_button_color_change = -10.0;

        let formatting = get_formatting();

        let mut buffer = Buffer::new_empty_with_color(
            (
                self.width,
                if self.collapsed {
                    self.menu_height
                } else {
                    self.height
                },
            ),
            formatting.background_color,
        );
        #[cfg(feature = "draw_debug")]
        println!("Created buffer");
        if self.collapsed {
            self.draw_menu(
                &mut buffer,
                &formatting.font,
                collapse_button_size,
                collapse_button_color_change,
            );
            return buffer;
        }

        let collision: mirl::math::collision::Rectangle<
            crate::DearMirlGuiCoordinateType,
            false,
        > = buffer.create_collision::<false, _>(0, 0).unwrap_or_default();
        #[cfg(feature = "draw_debug")]
        println!("Starting on drawing modules");

        // Using Rc<Buffer> instead of Buffer, without caching, increased testing fps from a stable 62 to a stable 63 - What
        // let mut buffers = Vec::new();

        let info = ModuleDrawInfo {
            container_id: self.id,
        };
        let mut module_idx_cache = Vec::new();

        let static_vertical_offset = self.camera.offset_y
            + (self.menu_height + formatting.horizontal_margin) as f32;
        let static_horizontal_offset = self.camera.offset_x;

        let mut extra_vertical_offset: crate::DearMirlGuiCoordinateType = 0;
        let mut extra_horizontal_offset: crate::DearMirlGuiCoordinateType = 0;

        if let Ok(modules) = MODULES.read() {
            for module_name in &self.modules {
                let Some(module_idx) = get_idx_of_id(*module_name) else {
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
                let x = formatting.horizontal_margin
                    as crate::DearMirlGuiCoordinateType
                    + extra_horizontal_offset
                    + static_horizontal_offset
                        as crate::DearMirlGuiCoordinateType;
                let y = extra_vertical_offset
                    + static_vertical_offset
                        as crate::DearMirlGuiCoordinateType;

                let col = buf
                    .0
                    .create_collision::<false, _>(x, y)
                    .unwrap_or_default();
                let position = (x, y);

                if collision.does_area_fully_include_other_area(&col) {
                    if FAST {
                        render::draw_buffer_on_buffer_1_to_1::<
                            false,
                            true,
                            false,
                            false,
                        >(
                            &mut buffer,
                            &buf.0,
                            position.try_tuple_into().unwrap_or_default(),
                        );
                    } else {
                        render::draw_buffer_on_buffer_1_to_1::<
                            false,
                            true,
                            true,
                            false,
                        >(
                            &mut buffer,
                            &buf.0,
                            position.try_tuple_into().unwrap_or_default(),
                        );
                    }
                } else if FAST {
                    render::draw_buffer_on_buffer_1_to_1::<
                        true,
                        true,
                        false,
                        false,
                    >(
                        &mut buffer,
                        &buf.0,
                        position.try_tuple_into().unwrap_or_default(),
                    );
                } else {
                    render::draw_buffer_on_buffer_1_to_1::<
                        true,
                        true,
                        true,
                        false,
                    >(
                        &mut buffer,
                        &buf.0,
                        position.try_tuple_into().unwrap_or_default(),
                    );
                }
                if horizontal_context {
                    extra_horizontal_offset +=
                        module_container.get_width(&formatting);
                } else {
                    extra_vertical_offset += module_container
                        .get_height(&formatting)
                        + formatting.vertical_margin
                            as crate::DearMirlGuiCoordinateType;
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
            &mut buffer,
            &formatting.font,
            collapse_button_size,
            collapse_button_color_change,
        );

        buffer
    }
    #[must_use]
    fn handle_dragging(
        &mut self,
        mouse_pos: Option<(i32, i32)>,
        mouse_pos_delta: (i32, i32),
        mouse_info: &MouseState,
        over_collapse_button: bool,
    ) -> (Option<CursorStyle>, FocusTaken) {
        let mut cursor_style = None;
        let mut gui_in_focus = FocusTaken::FocusFree;
        // WINDOW DRAGGING
        if let Some(current_mouse_position) = mouse_pos {
            let menu_metrics: mirl::math::collision::Rectangle<i32, false> =
                mirl::math::collision::Rectangle::new(
                    self.x as i32 + self.menu_height as i32,
                    self.y as i32,
                    self.width as i32 - self.menu_height as i32,
                    self.menu_height as i32,
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
                // FIXME: Maybe use f32 for xy for more precision
                self.x += mouse_pos_delta.0 as crate::DearMirlGuiCoordinateType;
                self.y += mouse_pos_delta.1 as crate::DearMirlGuiCoordinateType;
            } else {
                self.dragging = false;
            }
        }
        // If no mouse position is set, invalidate it
        if let Some(current_mouse_pos) = mouse_pos {
            self.last_mouse_pos =
                current_mouse_pos.try_tuple_into().unwrap_or_default();
        } else {
            self.last_mouse_pos = (i32::MIN, i32::MIN);
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
    fn handle_resizing(
        &mut self,
        mouse_pos: Option<(i32, i32)>,
        mouse_pos_delta: (i32, i32),
        mouse_info: &MouseState,
        over_collapse_button: bool,
    ) -> (Option<CursorStyle>, FocusTaken) {
        let mut cursor_style = None;
        let mut gui_in_focus = FocusTaken::FocusFree;
        if let Some(mouse_position) = mouse_pos {
            // Resize
            let hit_box: mirl::math::collision::Rectangle<i32, true> =
                mirl::math::collision::Rectangle::new(
                    self.x as i32,
                    self.y as i32,
                    self.width as i32,
                    self.height as i32,
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
                            corner_type_and_delta_to_metric_change::<
                                crate::DearMirlGuiCoordinateType,
                            >(
                                at_corner,
                                mouse_pos_delta
                                    .const_try_tuple_into()
                                    .unwrap_or_default(),
                            );
                        let width = self.width
                            as crate::DearMirlGuiCoordinateType
                            + metric_change.2;
                        if width
                            < self.min_width as crate::DearMirlGuiCoordinateType
                        {
                            self.width = self.min_width;
                        } else {
                            self.width = width as usize;
                            self.x += metric_change.0;
                        }
                        let height = self.height
                            as crate::DearMirlGuiCoordinateType
                            + metric_change.3;
                        if height
                            < self.menu_height
                                as crate::DearMirlGuiCoordinateType
                        {
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
        mouse_pos: Option<(i32, i32)>,
        mouse_scroll: Option<(f32, f32)>,
        left_mouse_down: bool,
        middle_mouse_down: bool,
        right_mouse_down: bool,
        pressed_keys: &Vec<KeyCode>,
        delta_time: f64,
        clipboard_data: &Option<mirl::platform::file_system::FileData>,
    ) -> GuiOutput {
        let mouse_data = MouseState {
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
    pub fn get_size_to_see_all_modules_caches(
        &self,
    ) -> (crate::DearMirlGuiCoordinateType, crate::DearMirlGuiCoordinateType)
    {
        self.size_to_see_all_modules
            .map_or_else(|| self.get_size_to_see_all_modules(), |s| s)
    }

    /// Get the size at which every module is visible
    pub fn get_size_to_see_all_modules(
        &self,
    ) -> (crate::DearMirlGuiCoordinateType, crate::DearMirlGuiCoordinateType)
    {
        let mut min_width = 0;
        let mut min_height = 0;
        let horizontal_context = false;

        let mut extra_vertical_offset = 0;
        let mut extra_horizontal_offset = 0;

        let mut used_idx = Vec::new();
        let formatting = get_formatting();

        if let Ok(modules) = MODULES.read() {
            for module_name in &self.modules {
                let Some(module_idx) = get_idx_of_id(*module_name) else {
                    continue;
                };
                used_idx.push(module_idx); // Fixed: should be module_idx, not idx
                let module_container = &modules[module_idx];

                // Calculate the actual position where this module would be drawn
                let x = formatting.horizontal_margin
                    as crate::DearMirlGuiCoordinateType
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
                    extra_vertical_offset += module_height
                        + formatting.vertical_margin
                            as crate::DearMirlGuiCoordinateType;
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
            min_width
                + formatting.horizontal_margin
                    as crate::DearMirlGuiCoordinateType,
            min_height
                + formatting.vertical_margin
                    as crate::DearMirlGuiCoordinateType
                    * 2,
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
        //         .add((formatting.horizontal_margin, 0).try_tuple_into());
        // }

        // (min_width , min_height+ formatting.vertical_margin as isize * 2)
    }
    /// Set the current size of the window to be able to see all modules
    pub fn set_size_to_see_all_modules(&mut self) {
        let size = self.get_size_to_see_all_modules();
        self.width =
            size.0.max(self.min_width as crate::DearMirlGuiCoordinateType)
                as usize;
        self.height =
            size.1.max(self.menu_height as crate::DearMirlGuiCoordinateType)
                as usize
                + self.menu_height;
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
        // But there aren't...
        // Is this like a form of schizophrenia or a parasocial thing?
        // Temporal collab :fire: :fire: :fire:

        let mut gui_in_focus = module_input.focus_taken;
        let mut cursor_style = module_outputs.new_cursor_style;

        if !gui_in_focus {
            //println!(">>>{:?}", gui_in_focus);
            let mut over_collapse_button = false;

            let collapse_button_collision = if let Some(current_mouse_pos) =
                module_input.mouse_pos
            {
                if self.collapse_button_collision_is_circle {
                    let t = self.menu_height as i32 / 2;
                    mirl::math::collision::Circle::<_, false>::new(
                        self.x as i32 + t,
                        self.y as i32 + t,
                        t,
                    )
                    .does_area_contain_point(
                        current_mouse_pos.try_tuple_into().unwrap_or_default(),
                    )
                } else {
                    mirl::math::collision::Rectangle::<_, false>::new(
                        self.x as i32,
                        self.y as i32,
                        self.menu_height as i32,
                        self.menu_height as i32,
                    )
                    .does_area_contain_point(
                        current_mouse_pos.try_tuple_into().unwrap_or_default(),
                    )
                }
            } else {
                false
            };

            if self.collapsed {
                gui_in_focus = FocusTaken::FunctionallyTaken;
            }
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
            if self.last_mouse_pos == (i32::MIN, i32::MIN)
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
            if !self.collapsed {
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
            .sub((formatting.horizontal_margin as i32, 0))
            .sub((self.x, self.y).try_tuple_into().unwrap_or_default())
            .sub((0, self.menu_height as i32));

        let static_vertical_offset =
            self.camera.offset_y as i32 + formatting.horizontal_margin as i32;
        let static_horizontal_offset = self.camera.offset_x as i32;

        let mut extra_vertical_offset: crate::DearMirlGuiCoordinateType = 0;
        let mut extra_horizontal_offset: crate::DearMirlGuiCoordinateType = 0;

        let mut module_idx_cache = Vec::new();
        if let Ok(modules) = MODULES.read() {
            for module_name in &self.modules {
                let Some(module_idx) = get_idx_of_id(*module_name) else {
                    continue;
                };
                let module = &modules[module_idx];
                module_idx_cache.push(module_idx);

                //let height = module.get_height(&formatting);
                let position = module_input.real_mouse_pos.map(|input| {
                    input
                        .add(cursor_offset)
                        .sub(
                            (extra_horizontal_offset, extra_vertical_offset)
                                .try_tuple_into()
                                .unwrap_or_default(),
                        )
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
                        + formatting.vertical_margin
                            as crate::DearMirlGuiCoordinateType;
                }
                module.modify_offset_cursor(
                    &modules[..],
                    &module_idx_cache,
                    &formatting,
                    (&mut extra_horizontal_offset, &mut extra_vertical_offset),
                );
            }
        }

        if gui_in_focus != FocusTaken::FunctionallyTaken
            && let Some(mouse_pos) = module_input.real_mouse_pos
        {
            let window_hit_box: mirl::math::collision::Rectangle<_, true> =
                mirl::math::collision::Rectangle::new(
                    self.x as i32,
                    self.y as i32,
                    self.get_width(&formatting) as i32,
                    self.get_height(&formatting) as i32,
                );
            if (window_hit_box.does_area_contain_point(mouse_pos)
                || self.resizing)
                && let Some(scroll) = module_input.mouse_scroll
            {
                if scroll == (0.0, 0.0) {
                    // This is if the cursor is on top on the gui but not interacting with anything
                    if module_input.mouse_info.left.clicked {
                        gui_in_focus = FocusTaken::FunctionallyTaken;
                    } else {
                        gui_in_focus = FocusTaken::VisuallyTaken;
                    }
                } else {
                    let size = self
                        .get_size_to_see_all_modules()
                        .try_tuple_into()
                        .unwrap_or_default();

                    self.camera.container_width = Self::get_width(self) as f32;
                    self.camera.container_height =
                        Self::get_height(self) as f32 - self.menu_height as f32;
                    self.camera.content_height = size.1;
                    self.camera.content_width = size.0;
                    self.camera.scroll(
                        scroll,
                        !module_input
                            .pressed_keys
                            .contains(&KeyCode::LeftShift),
                    );
                    // // println!(
                    // //     "{} {}",
                    // //     self.camera.offset_x, self.camera.offset_y
                    // // );
                    // if module_input.pressed_keys.contains(&KeyCode::LeftShift) {
                    //     self.camera.offset_x +=
                    //         scroll.1 * self.horizontal_scroll_multiplier_x;
                    //     self.camera.offset_y +=
                    //         scroll.0 * self.horizontal_scroll_multiplier_y;
                    // } else {
                    //     self.camera.offset_x +=
                    //         scroll.0 * self.vertical_scroll_multiplier_x;
                    //     self.camera.offset_y +=
                    //         scroll.1 * self.vertical_scroll_multiplier_y;
                    // }
                    // // println!(
                    // //     "{} {}",
                    // //     self.camera.offset_x, self.camera.offset_y
                    // // );

                    // let size: (f32, f32) =
                    //     self.get_size_to_see_all_modules().try_tuple_into();

                    // let y_range: [f32; 2] = if self.allow_free_scroll {
                    //     [
                    //         self.height as f32
                    //             - size.1
                    //             - self.menu_height as f32,
                    //         0.0,
                    //     ]
                    // } else {
                    //     [
                    //         (self.height as f32
                    //             - size.1
                    //             - self.menu_height as f32)
                    //             .min(0.0),
                    //         0.0,
                    //     ]
                    // };
                    // self.camera.offset_y =
                    //     self.camera.offset_y.clamp(y_range[0], y_range[1]);

                    // let x_range: [f32; 2] = if self.allow_free_scroll {
                    //     [self.width as f32 - size.0, 0.0]
                    // } else {
                    //     [(self.width as f32 - size.0).min(0.0), 0.0]
                    // };
                    // self.camera.offset_x =
                    //     self.camera.offset_x.clamp(x_range[0], x_range[1]);

                    gui_in_focus = FocusTaken::FunctionallyTaken;
                }
            }
        }

        if self.collapsed {
            if cursor_style.is_none() {
                gui_in_focus = FocusTaken::FocusFree;
            } else {
                gui_in_focus = FocusTaken::VisuallyTaken;
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
    /// Set the initial state of the window to be closed
    #[must_use]
    pub const fn collapsed(mut self) -> Self {
        self.collapsed = true;
        self
    }
}

/// Get a set of default keybinds provided for the gui
#[must_use]
#[allow(clippy::too_many_lines)] // Defining all these keybinds takes some lines, okay?
pub fn get_default_keybinds() -> Vec<KeyBind<Actions>> {
    Vec::from([
        KeyBind::new(
            false,
            false,
            false,
            KeyCode::Tab.to_vec(),
            Actions::SelectNextModule {
                wrap: true,
                skips: 1,
            },
        ),
        KeyBind::new(
            true,
            false,
            false,
            KeyCode::Tab.to_vec(),
            Actions::SelectPreviousModule {
                wrap: true,
                skips: 1,
            },
        ),
        KeyBind::new(
            false,
            false,
            false,
            KeyCode::Escape.to_vec(),
            Actions::ClearModuleSelection,
        ),
        KeyBind::new(
            false,
            false,
            false,
            KeyCode::Home.to_vec(),
            Actions::GoToModule(0),
        ),
        KeyBind::new(
            false,
            false,
            false,
            KeyCode::End.to_vec(),
            Actions::SelectLastModule,
        ),
        KeyBind::new(
            false,
            false,
            true,
            KeyCode::UpArrow.to_vec(),
            Actions::ScrollToTop,
        ),
        KeyBind::new(
            false,
            false,
            true,
            KeyCode::DownArrow.to_vec(),
            Actions::ScrollToBottom,
        ),
        KeyBind::new(
            false,
            false,
            true,
            KeyCode::RightArrow.to_vec(),
            Actions::ScrollToRight,
        ),
        KeyBind::new(
            false,
            false,
            true,
            KeyCode::LeftArrow.to_vec(),
            Actions::ScrollToLeft,
        ),
        KeyBind::new(
            false,
            false,
            true,
            KeyCode::Insert.to_vec(),
            Actions::VirtualCursorToggle(None),
        ),
        KeyBind::new(
            false,
            false,
            false,
            KeyCode::UpArrow.to_vec(),
            Actions::ScrollVertically(-1.0),
        ),
        KeyBind::new(
            false,
            false,
            false,
            KeyCode::DownArrow.to_vec(),
            Actions::ScrollVertically(1.0),
        ),
        KeyBind::new(
            false,
            false,
            false,
            KeyCode::LeftArrow.to_vec(),
            Actions::ScrollHorizontally(-1.0),
        ),
        KeyBind::new(
            false,
            false,
            false,
            KeyCode::RightArrow.to_vec(),
            Actions::ScrollHorizontally(1.0),
        ),
        KeyBind::new(
            false,
            false,
            false,
            KeyCode::Space.to_vec(),
            Actions::VirtualCursorLeftMouseClick(0),
        ),
    ])
}

fn add(list: &mut Vec<u32>, name: u32, id: usize) {
    get_module_mut(name, |x| {
        x.added(id);
    });
    list.push(name);
}
#[allow(clippy::needless_pass_by_value)]
fn remove(list: &mut Vec<u32>, name: u32, id: usize) {
    let index = list.find(&name);
    if let Some(idx) = index {
        list.remove(idx);
    }
    get_module_mut(name, |x| {
        x.removed(id);
    });
}
