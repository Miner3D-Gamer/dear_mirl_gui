use mirl::{
    misc::EasyUnwrapUnchecked,
    render::{self, Buffer, BufferGetPixel, BufferSetPixel},
};

/// Considering the distance the mouse moved and the width of the container, adjust the progress
pub fn adjust_progress_by_mouse<
    T: core::ops::Sub<Output = T>
        + core::ops::Div<Output = T>
        + core::ops::Add<Output = T>
        + Copy,
>(
    current_progress: T,
    mouse_x: T,
    width: T,
) -> T {
    let mouse_progress = mouse_x / width;
    mouse_progress + current_progress
}

/// A simple function to draw a diagonal cross
#[must_use]
pub fn draw_cross(size: usize, thickness: isize) -> Buffer {
    let mut buffer = Buffer::new_empty((size, size));
    render::draw_line::<true>(
        &mut buffer,
        (0, 0),
        (size, size),
        mirl::graphics::colors::WHITE,
        thickness,
    );
    render::draw_line::<true>(
        &mut buffer,
        (0, size),
        (size, 0),
        mirl::graphics::colors::WHITE,
        thickness,
    );

    buffer
}
/// A simple function to draw a solid block
#[must_use]
pub fn draw_blocking(size: usize, color: u32) -> Buffer {
    Buffer::new_empty_with_color((size, size), color)
}

/// Get the closest position between 2 characters to the target X
#[must_use]
pub fn get_closest_char_pos_to_mouse_pos(
    text: &str,
    height: f32,
    font: &mirl::dependencies::fontdue::Font,
    x: f32,
) -> usize {
    if x <= 0.0 {
        return 0;
    }
    // Why do I gotta divide the width by 1.25 to line it up properly?
    // In what context would that ever make sense? Idk how I even got this number in the first space
    let the_great_divider: f32 = 1.25;
    let char_count = text.chars().count();

    // Check the middle point of each character
    for i in 0..char_count {
        // Get width up to this character and up to the next character
        let width_before = if i == 0 {
            0.0
        } else {
            render::get_text_width(
                &text.chars().take(i).collect::<String>(),
                height,
                font,
            ) / the_great_divider
        };

        let width_after = render::get_text_width(
            &text.chars().take(i + 1).collect::<String>(),
            height,
            font,
        ) / the_great_divider;

        // The middle of this character
        let char_middle = f32::midpoint(width_before, width_after);

        // If we're before the middle of this character, cursor goes before it
        if x < char_middle {
            return i;
        }
    }

    // If we got here, cursor goes at the end
    char_count
}

/// It is a waste to store overlapping regions
#[must_use]
pub fn merge_selections(regions: &[(usize, usize)]) -> Vec<(usize, usize)> {
    let mut intervals: Vec<(usize, usize)> =
        regions.iter().map(|&(pos, width)| (pos, pos + width)).collect();

    intervals.sort_by_key(|x| x.0);

    let mut merged: Vec<(usize, usize)> = Vec::new();
    for (start, end) in intervals {
        if let Some(last) = merged.last_mut()
            && start <= last.1
        {
            last.1 = last.1.max(end);
            continue;
        }
        merged.push((start, end));
    }

    merged.into_iter().map(|(s, e)| (s, e - s)).collect()
}
#[must_use]
#[allow(clippy::needless_pass_by_value)]
/// When helper function when a module doesn't need to look differently on different guis
pub fn determine_need_redraw(list: Vec<(usize, bool)>) -> bool {
    list.iter().any(|x| x.1)
}
#[must_use]
/// Generate a fallback buffer containing an error message
pub fn new_buffer_error(string: &str) -> (Buffer, crate::module_manager::InsertionMode) {
    let formatting = crate::module_manager::get_formatting();
    (
        Buffer::new_with_text(
            string,
            formatting.height,
            &formatting.font,
            formatting.text_color,
            formatting.background_color,
            None,
        )
        .easy_unwrap_unchecked(), // We will never use a text size so big it'll actually crash, right?
        crate::module_manager::InsertionMode::ReplaceAll,
    )
}
/// A "shimmer" where pixels are simply interpolated between the old and new color
///
/// Used for the text highlighting in this lib
pub const fn shimmer(
    buffer: &mut (impl [const] BufferGetPixel + [const] BufferSetPixel),
    xy: (usize, usize),
    new_color: u32,
) {
    let under = buffer.get_pixel_unsafe(xy);
    buffer.set_pixel_safe(
        xy,
        mirl::graphics::interpolate_color_rgb_u32_f32(under, new_color, 0.5),
    );
}
