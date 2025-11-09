use mirl::Buffer;
use mirl::extensions::*;
use mirl::render;

use crate::ModuleDrawInfo;
use crate::ModuleUpdateInfo;
use crate::{AnyCasting, AnyCloning, Formatting};

/// Support builtin types
pub mod primitives;

/// Define what struct should be used instead of the "real" struct -> bool cannot store data
pub trait InspectableType {
    /// The target type for getting/setting values
    type Inspectable: Inspectable;
    /// Create an inspectable type from the raw value
    fn new_from_value(value: Self) -> Self::Inspectable;
}
// default impl<I: Inspectable + std::default::Default> InspectableType for I {
//     type Inspectable = I;
//     fn new_from_value(_: Self) -> Self::Inspectable {
//         std::default::Default::default()
//     }
// }

/// Get/Set struct values programmatically
pub trait Inspectable:
    std::fmt::Debug
    + Send
    + Sync
    + AnyCasting
    + AnyCloning
    + crate::DearMirlGuiModule
    + mirl::misc::Comparable
{
    /// Get all sub values
    fn get_fields_mut(&mut self) -> Vec<(&'static str, DynSyncInspectable)>;
    // #[allow(unused_variables)]
    // /// Draw the gui of the current thing
    // fn draw_ui(
    //     &mut self,
    //     formatting: &Formatting,
    //     info: &ModuleDrawInfo,
    // ) -> Buffer {
    //     Buffer::generate_fallback(self.get_size(formatting).tuple_into(), 2)
    // }
    // /// Get the size of the current thing
    // fn get_size(&mut self, _formatting: &Formatting) -> (isize, isize) {
    //     (50, 50)
    // }
    /// Sync the current value with a new one
    ///
    /// # Errors
    /// Usually when unable to lock or type casting went wrong
    fn sync(
        &mut self,
        new: &DynSyncInspectable,
    ) -> Result<(), Box<dyn std::error::Error>>;
    /// ENUM ONLY gets the possible enum value
    fn as_enum(&mut self) -> Option<&mut dyn InspectEnum> {
        None
    }
    /// Get the name of the current thing: MyStruct {} -> MyStruct
    fn get_name(&mut self) -> &'static str;
}

/// Get/Set enum values programmatically
pub trait InspectEnum {
    /// Get the name of each possible variant
    fn variants(&self) -> &'static [&'static str];
    /// Get the variant name index based on the given variant list
    fn current_index(&self) -> usize;
    /// Set the current variant
    fn set_variant(&mut self, index: usize);
    /// Get the fields/values the current variant may hold
    fn current_variant_fields(
        &mut self,
    ) -> Vec<(&'static str, DynSyncInspectable)>;
}

/// A reuseable helper for defining that an item is any Inspectable (thread safe)
pub type DynSyncInspectable =
    std::sync::Arc<std::sync::Mutex<Box<dyn Inspectable>>>;

/// Draws an inspectable onto a buffer
///
/// # Errors
/// When the lock has been poisoned
pub fn draw_inspectable(
    r: &DynSyncInspectable,
    depth: usize,
    formatting: &Formatting,
    known_size: Option<(isize, isize)>,
    name: Option<&str>,
    module_draw_info: &ModuleDrawInfo,
) -> Result<Buffer, Box<dyn std::error::Error>> {
    let size =
        known_size.unwrap_or(get_size_of_inspectable(r, formatting, name)?);
    let mut thing =
        r.lock().map_err(|e| format!("Failed to lock mutex: {e}",))?;

    let fields = thing.get_fields_mut();

    let mut buffer = Buffer::new_empty_with_color(
        size.tuple_into(),
        mirl::graphics::adjust_brightness_hsl_of_rgb(
            formatting.background_color,
            (5 * (1 + depth)) as f32,
        ),
    );
    let mut offset = (
        formatting.horizontal_margin as isize,
        formatting.vertical_margin as isize,
    );
    let type_name = thing.get_name();
    let name = name.map_or_else(
        || format!("{type_name}:"),
        |x| format!("{x} ({type_name}):"),
    );

    // Draw name
    render::draw_text_antialiased_isize::<true>(
        &mut buffer,
        &name,
        offset,
        formatting.text_color,
        formatting.height as f32,
        &formatting.font,
    );

    let title_height = render::get_text_height(
        &name,
        formatting.height as f32,
        &formatting.font,
    ) as isize;

    offset = offset.add((0, title_height));

    if fields.is_empty() {
        // It's a leaf node (primitive), draw it
        offset = offset.add((0, formatting.vertical_margin as isize));

        let img = thing.draw(formatting, module_draw_info).0;
        render::draw_buffer_on_buffer_1_to_1::<true, false, false, false>(
            &mut buffer,
            &img,
            offset,
        );
    } else {
        // It's a struct, traverse its fields
        drop(thing); // Release lock before recursing

        for (name, field) in fields {
            offset = offset.add((0, formatting.vertical_margin as isize));

            let img = draw_inspectable(
                &field,
                depth + 1,
                formatting,
                None,
                Some(name),
                module_draw_info,
            )?;
            render::draw_buffer_on_buffer_1_to_1::<true, false, false, false>(
                &mut buffer,
                &img,
                offset,
            );
            offset = offset.add((0, img.height as isize));
        }
    }

    Ok(buffer)
}
/// Precalculates the size of the inspectable before drawing it
///
/// # Errors
/// When the lock has been poisoned
pub fn get_size_of_inspectable(
    r: &DynSyncInspectable,
    formatting: &Formatting,
    name: Option<&str>,
) -> Result<(isize, isize), Box<dyn std::error::Error>> {
    let mut thing =
        r.lock().map_err(|e| format!("Failed to lock mutex: {e}"))?;

    let type_name = thing.get_name();
    let name = name.map_or_else(
        || format!("{type_name}:"),
        |x| format!("{x} ({type_name}):"),
    );
    let fields = thing.get_fields_mut(); // Use immutable version if available

    // Calculate title size
    let title_width = render::get_text_width(
        &name,
        formatting.height as f32,
        &formatting.font,
    ) as isize;
    let title_height = render::get_text_height(
        &name,
        formatting.height as f32,
        &formatting.font,
    ) as isize;

    let mut max_width = title_width;
    let mut total_height = title_height;

    if fields.is_empty() {
        // Leaf node - use custom size
        let (w, h) =
            (thing.get_width(formatting), thing.get_height(formatting))
                .tuple_into();
        max_width = max_width.max(w);
        total_height += formatting.vertical_margin as isize + h;
    } else {
        // Struct with fields - recurse
        drop(thing); // Release lock before recursing

        for (name, field) in fields {
            let (w, h) =
                get_size_of_inspectable(&field, formatting, Some(name))?;
            max_width = max_width.max(w);
            total_height += formatting.vertical_margin as isize + h;
        }
    }

    // Add outer margins
    Ok((
        max_width + (formatting.horizontal_margin * 2) as isize,
        total_height + (formatting.vertical_margin * 2) as isize,
    ))
}

/// Updates all inspectable with new user data
///
/// # Errors
/// When the lock has been poisoned
pub fn update_inspectable(
    r: &DynSyncInspectable,
    depth: usize,
    formatting: &Formatting,
    known_size: Option<(isize, isize)>,
    name: Option<&str>,
    module_update_info: &ModuleUpdateInfo,
) -> Result<crate::GuiOutput, Box<dyn std::error::Error>> {
    let mut module_update_info = *module_update_info;
    let size =
        known_size.unwrap_or(get_size_of_inspectable(r, formatting, name)?);
    let mut thing =
        r.lock().map_err(|e| format!("Failed to lock mutex: {e}",))?;

    let fields = thing.get_fields_mut();

    let mut buffer = Buffer::new_empty_with_color(
        size.tuple_into(),
        mirl::graphics::adjust_brightness_hsl_of_rgb(
            formatting.background_color,
            (5 * (1 + depth)) as f32,
        ),
    );
    let mut offset = (
        formatting.horizontal_margin as isize,
        formatting.vertical_margin as isize,
    );
    let type_name = thing.get_name();
    let name = name.map_or_else(
        || format!("{type_name}:"),
        |x| format!("{x} ({type_name}):"),
    );

    // Draw name
    render::draw_text_antialiased_isize::<true>(
        &mut buffer,
        &name,
        offset,
        formatting.text_color,
        formatting.height as f32,
        &formatting.font,
    );

    let title_height = render::get_text_height(
        &name,
        formatting.height as f32,
        &formatting.font,
    ) as isize;

    offset = offset.add((0, title_height));

    let mut output = crate::GuiOutput::empty();

    if fields.is_empty() {
        // It's a leaf node (primitive), draw it
        offset = offset.add((0, formatting.vertical_margin as isize));
        if let Some(pos) = module_update_info.mouse_pos {
            module_update_info.mouse_pos = Some(pos.sub(offset.tuple_into()));
        }

        output |= thing.update(&module_update_info);
    } else {
        // It's a struct, traverse its fields
        drop(thing); // Release lock before recursing

        for (name, field) in fields {
            offset = offset.add((0, formatting.vertical_margin as isize));

            output |= update_inspectable(
                &field,
                depth + 1,
                formatting,
                None,
                Some(name),
                &module_update_info,
            )?;
            let height;
            match field.lock() {
                Ok(mut val) => {
                    height = val.get_height(formatting);
                    drop(val);
                }
                Err(err) => {
                    return Err(format!("Failed to lock mutex: {err}",).into());
                }
            }

            offset = offset.add((0, height as isize));
        }
    }
    //println!("{output:?}");
    Ok(output)
}

// /// Updates the internal value with a newer version
// ///
// /// # Errors
// /// When the lock has been poisoned
// pub fn sync_inspectable(
//     storage: &DynSyncInspectable,
//     newer: &DynSyncInspectable,
//     depth: usize,
//     formatting: &Formatting,
//     name: Option<&str>,
// ) -> Result<(), Box<dyn std::error::Error>> {
//     let mut thing =
//         storage.lock().map_err(|e| format!("Failed to lock mutex: {e}",))?;

//     let fields = thing.get_fields_mut();

//     let type_name = thing.get_name();
//     let name = name.map_or_else(
//         || format!("{type_name}:"),
//         |x| format!("{x} ({type_name}):"),
//     );

//     // Draw name

//     if fields.is_empty() {
//     } else {
//         // It's a struct, traverse its fields
//         drop(thing); // Release lock before recursing

//         for (name, field) in fields {}
//     }

//     Ok(())
// }

// pub trait SyncWithSelf {
//     fn sync_with_self(&mut self, other: &Self);
// }
