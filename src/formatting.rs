

#[derive(Debug, Clone, Hash)]
/// The current formatting for a window/its modules
pub struct Formatting {
    /// The font in use
    pub font: fontdue::Font,
    /// The color of the background
    pub background_color: u32,
    /// The color of the rest of the elements
    pub foreground_color: u32,
    /// The color of the text
    pub text_color: u32,
    /// Currently unused
    pub misc_ui_color: u32,
    /// How many pixels there should be between each element of a module horizontally
    pub horizontal_margin: usize,
    /// How many pixels there should be between each element of a module vertically
    pub vertical_margin: usize,
}
impl Formatting {
    /// The color of the background
    pub const DEFAULT_BACKGROUND_COLOR: u32 =
        mirl::graphics::rgba_to_u32(10, 5, 20, 255);
    /// The color of the stuff that neither the background nor text
    pub const DEFAULT_STUFF_COLOR: u32 =
        mirl::graphics::rgba_to_u32(40, 30, 100, 255);
    /// The color of the text
    pub const DEFAULT_TEXT_COLOR: u32 = mirl::graphics::color_presets::WHITE;
    /// The margin between individual modules and the edge
    pub const DEFAULT_HORIZONTAL_MARGIN: usize = 5;
    /// The margin between individual modules and the edge
    pub const DEFAULT_VERTICAL_MARGIN: usize = 5;
    /// Create a formatting instance and fill in all empty values with defaults
    #[must_use]
    pub fn configured(
        font: &fontdue::Font,
        menu_text_color: Option<u32>,
        main_color: Option<u32>,
        secondary_color: Option<u32>,
        horizontal_margin: Option<usize>,
        vertical_margin: Option<usize>,
    ) -> Self {
        Self {
            font: font.clone(),
            background_color: main_color
                .unwrap_or(Self::DEFAULT_BACKGROUND_COLOR),
            foreground_color: secondary_color
                .unwrap_or(Self::DEFAULT_STUFF_COLOR),
            text_color: menu_text_color.unwrap_or(Self::DEFAULT_TEXT_COLOR),
            misc_ui_color: 0,
            horizontal_margin: horizontal_margin
                .unwrap_or(Self::DEFAULT_HORIZONTAL_MARGIN)
                / 2,
            vertical_margin: vertical_margin
                .unwrap_or(Self::DEFAULT_VERTICAL_MARGIN)
                / 2,
        }
    }
    /// Create a formatting instance and fill in all empty values with defaults
    #[must_use]
    pub fn default(font: &fontdue::Font) -> Self {
        Self {
            font: font.clone(),
            background_color: Self::DEFAULT_BACKGROUND_COLOR,
            foreground_color: Self::DEFAULT_STUFF_COLOR,
            text_color: Self::DEFAULT_TEXT_COLOR,
            misc_ui_color: 0,
            horizontal_margin: Self::DEFAULT_HORIZONTAL_MARGIN / 2,
            vertical_margin: Self::DEFAULT_VERTICAL_MARGIN / 2,
        }
    }
}