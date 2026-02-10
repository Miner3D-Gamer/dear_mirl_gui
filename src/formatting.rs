#[derive(Debug, Clone, Hash)]
/// The current formatting for a window/its modules
pub struct Formatting {
    /// The font in use
    pub font: mirl::dependencies::fontdue::Font,
    /// The default height/size for texts
    pub height: usize,
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
    pub const DEFAULT_TEXT_COLOR: u32 = mirl::graphics::colors::WHITE;
    /// The margin between individual modules and the edge
    pub const DEFAULT_HORIZONTAL_MARGIN: usize = 5;
    /// The margin between individual modules and the edge
    pub const DEFAULT_VERTICAL_MARGIN: usize = 5;
    /// Create a formatting instance and fill in all empty values with defaults
    #[must_use]
    pub fn configured(
        font: &mirl::dependencies::fontdue::Font,
        height: usize,
        menu_text_color: Option<u32>,
        main_color: Option<u32>,
        secondary_color: Option<u32>,
        horizontal_margin: Option<usize>,
        vertical_margin: Option<usize>,
    ) -> Self {
        Self {
            height,
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
    pub fn default(
        font: &mirl::dependencies::fontdue::Font,
        height: usize,
    ) -> Self {
        Self {
            height,
            font: font.clone(),
            background_color: Self::DEFAULT_BACKGROUND_COLOR,
            foreground_color: Self::DEFAULT_STUFF_COLOR,
            text_color: Self::DEFAULT_TEXT_COLOR,
            misc_ui_color: 0,
            horizontal_margin: Self::DEFAULT_HORIZONTAL_MARGIN / 2,
            vertical_margin: Self::DEFAULT_VERTICAL_MARGIN / 2,
        }
    }
    #[must_use]
    /// Set the font
    pub fn font(mut self, font: mirl::dependencies::fontdue::Font) -> Self {
        self.font = font;
        self
    }

    #[must_use]
    /// Set the background color
    pub const fn background_color(mut self, color: u32) -> Self {
        self.background_color = color;
        self
    }

    #[must_use]
    /// Set the foreground color
    pub const fn foreground_color(mut self, color: u32) -> Self {
        self.foreground_color = color;
        self
    }

    #[must_use]
    /// Set the text color
    pub const fn text_color(mut self, color: u32) -> Self {
        self.text_color = color;
        self
    }

    #[must_use]
    /// Set the miscellaneous UI color
    pub const fn misc_ui_color(mut self, color: u32) -> Self {
        self.misc_ui_color = color;
        self
    }

    #[must_use]
    /// Set the horizontal margin
    pub const fn horizontal_margin(mut self, margin: usize) -> Self {
        self.horizontal_margin = margin;
        self
    }

    #[must_use]
    /// Set the vertical margin
    pub const fn vertical_margin(mut self, margin: usize) -> Self {
        self.vertical_margin = margin;
        self
    }
}
