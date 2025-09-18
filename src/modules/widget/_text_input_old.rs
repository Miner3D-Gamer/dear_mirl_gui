use mirl::extensions::{ListGetNewItems, ListHasDuplicates};
use mirl::platform::keycodes::StringToKeyCodes;
use mirl::{
    extensions::RemoveChar,
    platform::{CursorStyle, keycodes::KeyCode},
};

use crate::{
    Buffer,
    DearMirlGuiModule,
    gui::ModuleInputs,
    //modules::misc::merge_selections,
    render,
};
#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(unpredictable_function_pointer_comparisons)]
/// TODO:
/// - Caret blinking animation
/// - Text scrolling
/// - Text selection/highlighting
/// - ~~Better Carets (Multi caret support)~~
/// - ~~Multiline support~~
/// - ~~Automatic mouse position to caret position~~
/// - ~~Arrow Movement~~
/// - ~~Delete key Support~~
/// - ~~Undo/Redo System~~
/// - ~~Copy Support~~
/// - ~~Paste Support~~
/// - ~~Cut Support~~
///
/// Keybinds:
/// - Enter                 -> Creates a new line
/// - Tab                   -> Create X spaces until the caret is at a multiple of `tab_length`
/// - Backspace             -> Delete a single character to the left
/// - Backspace + Control   -> Deletes the character structure to the left
/// - Delete                -> Delete a single character to the right
/// - Delete + Shift        -> Delete the current line
/// - Delete + Control      -> Deletes the character structure to the right
/// - Z + Control           -> Undo
/// - Y + Control           -> Redo
/// - C + Control           -> Copy current line(s)
/// - X + Control           -> Cut current line(s)
/// - V + Control           -> Pase text at current line(s)
/// - Up                    -> Move caret(s) up one line
/// - Up + Alt              -> Swap current line and the line above
/// - Up + Alt + Shift      -> Duplicate line and don't move cursor
/// - Up + Alt + Control    -> Add caret in line above
/// - Down                  -> Move caret(s) down one line
/// - Down + Alt            -> Swap current line and the line above
/// - Down + Alt + Shift    -> Duplicate line and move cursor to new line
/// - Down + Alt + Control  -> Add caret in line below
/// - Left                  -> Move caret(s) left/to previous line
/// - Right                 -> Move caret(s) right/to next line
/// - Left + Control        -> Move caret(s) left while skipping the inners of a character structure/to previous line
/// - Right + Control       -> Move caret(s) right while skipping the inners of a character structure/to next line
///
pub struct TextInput {
    #[allow(missing_docs)]
    pub width: usize,
    #[allow(missing_docs)]
    pub text_height: usize,
    /// How many lines are allowed
    pub lines: usize,
    /// The text the input contains
    pub text: Vec<String>,
    #[allow(missing_docs)]
    pub needs_redraw: std::cell::Cell<bool>,
    /// If the button has been selected
    pub selected: bool,
    /// Used for detecting new key strokes
    pub last_keys_pressed: Vec<KeyCode>,
    /// The vertical, and horizontal position of the cursors. Yes for some reason this has multi cursor support
    pub caret_positions: Vec<(usize, usize)>,
    /// What text has been selected in the following format: Vec{Vertical}<Vec<(usize {horizontal}, usize {length})>> , once again with multi cursor support
    pub highlighted: Vec<Vec<(usize, usize)>>,

    /// When at the front of a string, should pressing backspace allow you to delete what is behind? The objective answer is no, the subjective answer is 'Let it be configurable'.
    pub remove_behind: bool,
    /// Last text states allowing for ctrl+z/ctrl+y
    pub last_states: Vec<Vec<String>>,
    /// The current state so the functions don't get confused what state is currently used
    pub current_state: usize,
    /// What text should be displayed when no text is written
    pub placeholder_text: String,
    /// If you can select the input field
    pub read_only: bool,
    /// How big a "tab" is
    pub tab_length: usize,
}
impl TextInput {
    #[allow(missing_docs)]
    #[must_use]
    pub fn new(
        text_height: usize,
        width: usize,
        lines: usize,
        text: Option<Vec<String>>,
        placeholder_text: Option<String>,
    ) -> Self {
        Self {
            width,
            text_height,
            lines,
            text: text.clone().unwrap_or_else(|| Vec::from([String::new()])),
            needs_redraw: true.into(),
            selected: false,
            last_keys_pressed: Vec::new(),
            // Yummy
            caret_positions: Vec::from([(
                text.clone().unwrap_or_default().len().saturating_sub(1),
                text.unwrap_or_default()
                    .last()
                    .map_or_default(|x| x.chars().count()),
            )]),
            highlighted: Vec::new(),
            remove_behind: false,
            last_states: Vec::new(),
            current_state: 0,
            read_only: false,
            placeholder_text: placeholder_text.unwrap_or_default(),
            tab_length: 4,
        }
    }
    fn is_something_selected(&self) -> bool {
        self.highlighted.iter().any(|x| !x.is_empty())
    }

    #[allow(clippy::too_many_lines)]
    #[allow(clippy::cognitive_complexity)] // Idc if it looks 'complex', it mostly works so I'm not changing it
    /// Takes in the keycodes and writes/deleted/moves cursor
    pub fn handle_keycodes(
        &mut self,
        new_keycodes: &[KeyCode],
        held_keycodes: &[KeyCode],
        info: &ModuleInputs,
    ) -> (bool, bool, Option<mirl::platform::file_system::FileData>) {
        let mut new_keycodes = new_keycodes.to_vec();
        let mut state_change = false;
        //let mut simplify_highlighted = false;
        let control_down = held_keycodes.contains(&KeyCode::LeftControl)
            || held_keycodes.contains(&KeyCode::RightControl);
        let alt_down = held_keycodes.contains(&KeyCode::LeftAlt)
            || held_keycodes.contains(&KeyCode::RightAlt);
        let shift_down = held_keycodes.contains(&KeyCode::LeftShift)
            || held_keycodes.contains(&KeyCode::RightShift);

        if control_down && new_keycodes.contains(&KeyCode::V) {
            return (false, true, None);
        }
        let mut new_clipboard_data = None;
        let c_pressed = new_keycodes.contains(&KeyCode::C);
        let x_pressed = new_keycodes.contains(&KeyCode::X);
        let copy = c_pressed || x_pressed;

        if control_down && copy {
            new_keycodes.retain(|x| *x != KeyCode::C);
            if self.caret_positions.len() == 1 {
                new_clipboard_data =
                    Some(mirl::platform::file_system::FileData::from_bytes(
                        self.text[self.caret_positions[0].0].clone().into(),
                        mirl::platform::file_system::DataType::Text,
                    ));
                if x_pressed {
                    new_keycodes.retain(|x| *x != KeyCode::X);
                    self.text.remove(self.caret_positions[0].0);
                    if self.text.is_empty() {
                        self.text.push(String::new());
                    }
                    for (vertical, horizontal) in &mut self.caret_positions {
                        if *vertical >= self.text.len() {
                            *vertical = self.text.len().saturating_sub(1);
                            *horizontal =
                                self.text[self.text.len() - 1].chars().count();
                        }
                    }
                }
            } else {
                let data = self.text.clone();
                new_clipboard_data =
                    Some(mirl::platform::file_system::FileData::from_bytes(
                        mirl::misc::strings_to_bytes(data),
                        mirl::platform::file_system::DataType::Text,
                    ));
            }
        }
        // else if new_keycodes.contains(&KeyCode::D)
        //     && self.caret_positions.len() == 1
        //     && self.text.len() < self.lines
        // {
        //     let v = self.caret_positions[0].0;
        //     let text = self.text[v].clone();
        //     self.text.insert(v, text);
        // }

        if let Some(clipboard_data) = info.clipboard_data
            && let Ok(text_data) = clipboard_data.as_string()
        {
            let line_amount =
                text_data.chars().filter(|x| x.to_string() == "\n").count();
            let all_on_different_lines = !self
                .caret_positions
                .iter()
                .map(|x: &(usize, usize)| x.0)
                .collect::<Vec<usize>>()
                .has_duplicates();

            if line_amount == self.caret_positions.len()
                && all_on_different_lines
            {
                let lines: Vec<&str> = text_data.split('\n').collect();
                for (idx, line_content) in
                    lines.iter().enumerate().take(line_amount)
                {
                    let after: String = self.text[self.caret_positions[idx].0]
                        .chars()
                        .skip(self.caret_positions[idx].1)
                        .collect();
                    let mut before: String = self.text
                        [self.caret_positions[idx].0]
                        .chars()
                        .take(self.caret_positions[idx].1)
                        .collect();
                    before.push_str(line_content);
                    before.push_str(&after);
                    self.text[idx] = before;
                    self.caret_positions[idx].1 += line_content.chars().count();
                }
            } else {
                self.handle_keycodes(
                    &text_data.to_keycodes(),
                    &Vec::new(),
                    info,
                );
            }
        }

        let action_on_highlighted = false && self.is_something_selected();
        if action_on_highlighted {
            // let mut destroy_highlight = false;
            // for keycode in new_keycodes {
            //     if let Some(value) = keycode.to_user_friendly_string() {
            //         destroy_highlight = true;
            //         self.caret_positions = Vec::new();
            //         for (vertical, highlights) in
            //             &mut self.highlighted.iter().enumerate()
            //         {
            //             for (pos, width) in highlights {
            //                 let before: String = self.text[vertical]
            //                     .chars()
            //                     .take(*pos)
            //                     .collect();
            //                 let after: String = self.text[vertical]
            //                     .chars()
            //                     .skip(*pos)
            //                     .collect();
            //                 if shift_down {
            //                     self.text[vertical] =
            //                         before + &value.to_uppercase() + &after;
            //                 } else {
            //                     self.text[vertical] =
            //                         before + &value.to_lowercase() + &after;
            //                 }
            //                 self.caret_positions
            //                     .push((vertical, pos + width + 1));
            //             }
            //         }
            //     } else {
            //         match keycode {
            //             KeyCode::Backspace => {}
            //             _ => {}
            //         }
            //     }
            // }
        } else {
            for keycode in &*new_keycodes {
                if let Some(value) = keycode.to_user_friendly_string() {
                    for (vertical, horizontal) in &mut self.caret_positions {
                        let before: String = self.text[*vertical]
                            .chars()
                            .take(*horizontal)
                            .collect();
                        let after: String = self.text[*vertical]
                            .chars()
                            .skip(*horizontal)
                            .collect();
                        if shift_down {
                            self.text[*vertical] =
                                before + &value.to_uppercase() + &after;
                        } else {
                            self.text[*vertical] =
                                before + &value.to_lowercase() + &after;
                        }
                        *horizontal += 1;
                    }
                } else {
                    match keycode {
                        KeyCode::Backspace => {
                            for (vertical, horizontal) in
                                &mut self.caret_positions
                            {
                                let previous = *horizontal;
                                *horizontal = horizontal.saturating_sub(1);
                                if *horizontal == 0
                                    && previous == 0
                                    && *vertical != 0
                                {
                                    let text = self.text.remove(*vertical);
                                    *vertical = vertical.saturating_sub(1);

                                    *horizontal =
                                        self.text[*vertical].chars().count();
                                    self.text[*vertical].push_str(&text);
                                } else if previous != 0 || self.remove_behind {
                                    self.text[*vertical]
                                        .remove_char_at(*horizontal);
                                }
                            }
                        }
                        KeyCode::Delete => {
                            for (vertical, horizontal) in
                                &mut self.caret_positions
                            {
                                if shift_down {
                                    if self.text.len() > 1 {
                                        self.text.remove(*vertical);
                                        *vertical = *vertical.min(
                                            &mut self
                                                .text
                                                .len()
                                                .saturating_sub(1),
                                        );
                                    } else {
                                        self.text[0] = String::new();
                                    }
                                } else if *horizontal
                                    == self.text[*vertical].chars().count()
                                    && *vertical != self.text.len() - 1
                                {
                                    let next_line =
                                        self.text.remove(*vertical + 1);
                                    self.text[*vertical].push_str(&next_line);
                                } else {
                                    self.text[*vertical]
                                        .remove_char_at(*horizontal);
                                    *horizontal = *horizontal.min(
                                        &mut self.text[*vertical]
                                            .chars()
                                            .count(),
                                    );
                                }
                            }
                        }
                        KeyCode::Left => {
                            for (vertical, horizontal) in
                                &mut self.caret_positions
                            {
                                if *horizontal == 0 && *vertical != 0 {
                                    *vertical = vertical.saturating_sub(1);

                                    *horizontal =
                                        self.text[*vertical].chars().count();
                                } else if control_down {
                                    *horizontal =
                                    mirl::misc::skipping_text_type::skip_char_type_backward(
                                        &self.text[*vertical],
                                        *horizontal,
                                    );
                                } else {
                                    *horizontal = horizontal.saturating_sub(1);
                                }
                            }
                        }
                        KeyCode::Right => {
                            for (vertical, horizontal) in
                                &mut self.caret_positions
                            {
                                let length =
                                    self.text[*vertical].chars().count();
                                if *horizontal == length
                                    && *vertical != self.text.len() - 1
                                {
                                    *vertical =
                                        self.text.len().min(*vertical + 1);
                                    *horizontal = 0;
                                } else if control_down {
                                    *horizontal =
                                        mirl::misc::skipping_text_type::skip_char_type(
                                            &self.text[*vertical],
                                            *horizontal,
                                        );
                                } else {
                                    *horizontal =
                                        horizontal.saturating_add(1).min(
                                            self.text[*vertical]
                                                .chars()
                                                .count(),
                                        );
                                }
                            }
                        }
                        KeyCode::Up => {
                            if self.caret_positions.len() == 1 && alt_down {
                                let current = self.caret_positions[0].0;
                                let new =
                                    self.caret_positions[0].0.saturating_sub(1);
                                if shift_down {
                                    if self.text.len() < self.lines {
                                        let text = self.text[current].clone();
                                        self.text.insert(current, text);
                                    }
                                } else {
                                    self.text.swap(current, new);
                                    self.caret_positions[0].0 = new;
                                }
                            } else {
                                for (vertical, horizontal) in
                                    &mut self.caret_positions
                                {
                                    if *vertical == 0 {
                                        *horizontal = 0;
                                    } else {
                                        *vertical -= 1;
                                        *horizontal = *horizontal.min(
                                            &mut self.text[*vertical]
                                                .chars()
                                                .count(),
                                        );
                                    }
                                }
                            }
                        }
                        KeyCode::Down => {
                            if self.caret_positions.len() == 1 && alt_down {
                                let current = self.caret_positions[0].0;
                                let new = (self.caret_positions[0].0 + 1)
                                    .min(self.text.len() - 1);
                                if shift_down {
                                    if self.text.len() < self.lines {
                                        let text = self.text[current].clone();
                                        self.text.insert(current, text);
                                        self.caret_positions[0].0 = current + 1;
                                    }
                                } else {
                                    self.text.swap(current, new);
                                    self.caret_positions[0].0 = new;
                                }
                            } else {
                                for (vertical, horizontal) in
                                    &mut self.caret_positions
                                {
                                    if *vertical == self.text.len() - 1 {
                                        *horizontal = self.text[*vertical]
                                            .chars()
                                            .count();
                                    } else {
                                        *vertical += 1;
                                        *horizontal = *horizontal.min(
                                            &mut self.text[*vertical]
                                                .chars()
                                                .count(),
                                        );
                                    }
                                }
                            }
                        }
                        KeyCode::Enter => {
                            for (vertical, horizontal) in
                                &mut self.caret_positions
                            {
                                if self.text.len() < self.lines {
                                    if control_down {
                                        self.text.insert(
                                            *vertical + 1,
                                            String::new(),
                                        );
                                    } else {
                                        // Take the text that is right to the caret and throw it on a new line
                                        let text_left = self.text[*vertical]
                                            .chars()
                                            .skip(*horizontal)
                                            .collect();
                                        self.text[*vertical] = self.text
                                            [*vertical]
                                            .chars()
                                            .take(*horizontal)
                                            .collect();
                                        self.text
                                            .insert(*vertical + 1, text_left);
                                    }
                                    if *vertical < self.text.len() {
                                        *vertical += 1;
                                    }
                                    *horizontal = 0;
                                }
                            }
                        }
                        KeyCode::Tab => {
                            for (vertical, horizontal) in
                                &mut self.caret_positions
                            {
                                let text_left: String = self.text[*vertical]
                                    .chars()
                                    .take(*horizontal)
                                    .collect();

                                let text_right: String = self.text[*vertical]
                                    .chars()
                                    .skip(*horizontal)
                                    .collect();

                                let length =
                                    text_left.chars().count() % self.tab_length;
                                let repeats = self.tab_length - length;
                                let insertion = " ".repeat(repeats);

                                let text = text_left + &insertion + &text_right;
                                *horizontal += repeats;
                                self.text[*vertical] = text;
                            }
                        }

                        _ => {}
                    }
                }
            }
        }

        if control_down {
            if new_keycodes.contains(&KeyCode::Z) {
                state_change = true;
                self.current_state = self.current_state.saturating_sub(1);
                self.text = self.last_states[self.current_state].clone();
            } else if new_keycodes.contains(&KeyCode::Y) {
                state_change = true;
                self.current_state = self
                    .last_states
                    .len()
                    .saturating_sub(1)
                    .min(self.current_state + 1);
                self.text = self.last_states[self.current_state].clone();
            }
        }
        // if simplify_highlighted {
        //     for x in &mut self.highlighted {
        //         *x = merge_selections(x);
        //     }
        // }
        (state_change, false, new_clipboard_data)
    }
}

impl DearMirlGuiModule for TextInput {
    #[allow(clippy::too_many_lines)]
    fn draw(&self, formatting: &crate::Formatting) -> Buffer {
        fn shimmer(buffer: &Buffer, x: usize, y: usize, new_color: u32) {
            let under = buffer.get_pixel((x, y));
            buffer.set_pixel_safe(
                x,
                y,
                mirl::graphics::interpolate_color_rgb_u32_f32(
                    under, new_color, 0.8,
                ),
            );
        }
        // Settings
        let text_color = formatting.text_color;
        let text_size_mul = 0.8;
        let background_color_change = -5.0;
        let background_color = formatting.secondary_color;
        let caret_color = formatting.text_color;
        let caret_width = self.text_height / 10;
        let placeholder_color_change = -30.0;

        // Code
        let mut color = background_color;

        color = mirl::graphics::adjust_brightness_hsl_of_rgb(
            color,
            background_color_change,
        );
        let buffer = Buffer::new_empty_with_color(
            self.width,
            self.get_height(formatting) as usize,
            color,
        );

        let x = 0;
        for (idx, text) in self.text.iter().enumerate() {
            render::draw_text_antialiased::<{ crate::DRAW_SAFE }>(
                &buffer,
                text,
                x as usize + formatting.horizontal_margin,
                idx * (self.text_height + formatting.vertical_margin)
                    + formatting.vertical_margin,
                text_color,
                self.text_height as f32 * text_size_mul,
                &formatting.font,
            );
            for (pos, width) in &self.highlighted[idx] {
                let text_until =
                    self.text[idx].chars().take(*pos).collect::<String>();
                let text_between = self.text[idx]
                    .chars()
                    .skip(*pos)
                    .take(*width)
                    .collect::<String>();
                let offset = render::get_text_width(
                    &text_until,
                    self.text_height as f32,
                    &formatting.font,
                );
                let width = render::get_text_width(
                    &text_between,
                    self.text_height as f32,
                    &formatting.font,
                );
                render::execute_at_rectangle::<true>(
                    &buffer,
                    (
                        offset as isize + formatting.horizontal_margin as isize,
                        (idx * (self.text_height + formatting.vertical_margin)
                            + formatting.vertical_margin)
                            as isize,
                    ),
                    (width as isize, self.text_height as isize),
                    mirl::graphics::color_presets::PURE_BLUE,
                    shimmer,
                );
            }
        }

        if self.selected {
            for (vertical, horizontal) in &self.caret_positions {
                let before = self.text[*vertical]
                    .chars()
                    .take(*horizontal)
                    .collect::<String>();

                let offset = render::get_text_width(
                    &before,
                    self.text_height as f32 * text_size_mul,
                    &formatting.font,
                );
                render::execute_at_rectangle::<true>(
                    &buffer,
                    (
                        x + offset as isize
                            + formatting.horizontal_margin as isize,
                        ((self.text_height + formatting.vertical_margin)
                            * vertical
                            + formatting.vertical_margin)
                            as isize,
                    ),
                    (caret_width as isize, self.text_height as isize),
                    caret_color,
                    mirl::misc::invert_color_if_same,
                );
            }
        }
        if self.text.len() == 1 && self.text[0].chars().count() == 0 {
            render::draw_text_antialiased::<{ crate::DRAW_SAFE }>(
                &buffer,
                &self.placeholder_text,
                formatting.horizontal_margin,
                formatting.vertical_margin,
                mirl::graphics::adjust_brightness_hsl_of_rgb(
                    text_color,
                    placeholder_color_change,
                ),
                self.text_height as f32 * text_size_mul,
                &formatting.font,
            );
        }

        // render::draw_text_antialiased::<{ crate::DRAW_SAFE }>(
        //     &buffer,
        //     &after,
        //     x as usize + offset as usize,
        //     0,
        //     text_color,
        //     self.height as f32 * text_size_mul,
        //     &formatting.font,
        // );

        buffer
    }
    fn get_height(&self, formatting: &crate::Formatting) -> isize {
        ((self.text_height + formatting.vertical_margin) * self.lines
            + formatting.vertical_margin * 2) as isize
    }
    fn get_width(&self, _formatting: &crate::Formatting) -> isize {
        self.width as isize
    }
    fn update(
        &mut self,
        info: &crate::ModuleInputs,
        formatting: &crate::Formatting,
    ) -> crate::GuiOutput {
        let mut cursor_style = None;
        let collision = mirl::math::collision::Rectangle::<_, false>::new(
            0,
            0,
            self.get_width(formatting),
            self.get_height(formatting),
        );
        if info.focus_taken {
            self.selected = false;
            self.highlighted = Vec::new();
            self.caret_positions = Vec::new();
        }

        if let Some(mouse_position) = info.mouse_pos {
            let collides = collision.does_area_contain_point(mouse_position);
            if collides {
                cursor_style = CursorStyle::Text.into();
            } else if info.mouse_info.left_clicked {
                self.selected = false;
            }
            if collides && info.mouse_info.left_clicked {
                self.selected = true;
                let vertical = (mouse_position.1 as usize / self.text_height)
                    .min(self.text.len() - 1);
                let horizontal = super::misc::get_closest_char_pos_to_mouse_pos(
                    &self.text[vertical],
                    self.text_height as f32,
                    &formatting.font,
                    mouse_position.0 as f32,
                );
                if info.pressed_keys.contains(&KeyCode::LeftAlt)
                    || info.pressed_keys.contains(&KeyCode::RightAlt)
                {
                    self.caret_positions.push((vertical, horizontal));
                } else {
                    self.caret_positions = Vec::from([(vertical, horizontal)]);
                }
            }
        }
        if self.read_only {
            self.selected = false;
            cursor_style = None;
        }
        let mut request_clipboard_data = false;
        let mut new_clipboard_data = None;
        if self.selected {
            self.needs_redraw.set(true);
            let new_keys: Vec<KeyCode> = self
                .last_keys_pressed
                .get_old_items(info.pressed_keys)
                .iter()
                .map(|x| **x)
                .collect();

            let state = self.text.clone();
            let (
                do_not_save_state,
                request_clipboard_data_local,
                new_clipboard_data_local,
            ) = self.handle_keycodes(&new_keys, info.pressed_keys, info);
            request_clipboard_data = request_clipboard_data_local;
            new_clipboard_data = new_clipboard_data_local;
            if !do_not_save_state && self.text != state {
                if self.current_state == self.last_states.len() {
                    self.last_states.push(Vec::new());
                }
                self.last_states[self.current_state] = state;
                self.current_state += 1;
                self.last_states.truncate(self.current_state);
            }

            self.last_keys_pressed.clone_from(info.pressed_keys);
        }
        while self.text.len() > self.highlighted.len() {
            self.highlighted.push(Vec::new());
        }
        crate::GuiOutput {
            new_clipboard_data,
            new_cursor_position: None,
            focus_taken: false,
            hide_cursor: false,
            text_input_selected: self.selected,
            new_cursor_style: cursor_style,
            request_clipboard_data,
        }
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
