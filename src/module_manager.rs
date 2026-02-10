#![allow(clippy::significant_drop_tightening)]
use mirl::prelude::Buffer;

static CONTAINER_ID: std::sync::atomic::AtomicUsize =
    std::sync::atomic::AtomicUsize::new(1);

/// Get an ID that has not yet been used by any other container
pub fn get_available_id() -> usize {
    CONTAINER_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
}

use crate::{
    DearMirlGuiModule, Formatting, GuiReturnModuleError, ModulePath, WhatAmI,
    gui::ModuleContainer,
};

/// A global context that manages the existence of modules
pub static MODULES: ModuleManager =
    std::sync::LazyLock::new(|| std::sync::RwLock::new(Vec::new()));

/// Metric and image output cache so modules don't need to be redrawn every frame
pub static MODULES_IMAGE_CACHE: ModuleImageCache =
    std::sync::LazyLock::new(|| std::sync::RwLock::new(Vec::new()));
/// Convert from a generic id to a index
pub static MODULE_INDEX: std::sync::LazyLock<
    std::sync::RwLock<std::collections::HashMap<u32, usize>>,
> = std::sync::LazyLock::new(|| {
    std::sync::RwLock::new(std::collections::HashMap::new())
});
/// Convert from a generic id to a index
pub fn get_idx_of_id(name: u32) -> Option<usize> {
    MODULE_INDEX.read().ok()?.get(&name).copied()
}
type ModuleManager =
    std::sync::LazyLock<std::sync::RwLock<Vec<ModuleContainer>>>;

type ModuleImageCache =
    std::sync::LazyLock<std::sync::RwLock<Vec<Vec<BufferState>>>>;
/// The formatting to be globally used
pub static FORMATTING: std::sync::LazyLock<
    std::sync::RwLock<Option<std::sync::Arc<Formatting>>>,
> = std::sync::LazyLock::new(|| std::sync::RwLock::new(None));
/// Set the global formatting
pub fn set_formatting(formatting: Formatting) {
    if let Ok(mut yeah) = FORMATTING.write() {
        *yeah = Some(std::sync::Arc::new(formatting));
    }
}
#[allow(clippy::expect_used)]
/// Get the global formatting
///
/// # Panics
///
/// When the formatting is not set it will error, telling you how to set the formatting
pub fn get_formatting() -> std::sync::Arc<Formatting> {
    FORMATTING
        .read()
        .expect("Failed to acquire formatting lock")
        .as_ref()
        .expect("No formatting available!\
                \n> Please use `dear_mirl_gui::module_manager::set_formatting(dear_mirl_gui::Formatting::default(&font, {height}));`\
                \n> To get the font your os uses: `FileSystem::get_default_font()` (Required mirl flags: `font_support`, `std`, struct found in `mirl::prelude::FileSystem`)")
        .clone()
}
// type BufferAndOffset = std::sync::RwLock<(Buffer, (isize, isize))>;

/// Add a module to the global context
pub fn register_module<T: DearMirlGuiModule + 'static>(
    //  name: u32,
    module: T,
) -> ModulePath<T> {
    let path = ModulePath::new();
    let idx;
    let mut module = module;
    let formatting = get_formatting();
    module.apply_new_formatting(&formatting);

    if let Ok(mut list) = MODULES.write() {
        idx = list.len();
        list.push(ModuleContainer::new(module));
    } else {
        idx = 0;
    }

    if let Ok(mut cache) = MODULES_IMAGE_CACHE.write()
        && cache.len() <= idx
    {
        cache.resize_with(idx + 1, Vec::new);
    }

    if let Ok(mut index_map) = MODULE_INDEX.write() {
        index_map.insert(path.id, idx);
    }

    path
}
/// Remove a module from the global context
pub fn remove_module<T>(path: &ModulePath<T>) -> bool {
    let Some(idx) = get_idx_of_id(path.id) else {
        return false;
    };

    if let Ok(mut list) = MODULES.write()
        && idx < list.len()
    {
        list.remove(idx);
    }

    if let Ok(mut cache) = MODULES_IMAGE_CACHE.write()
        && idx < cache.len()
    {
        cache.remove(idx);
    }

    if let Ok(mut index_map) = MODULE_INDEX.write() {
        index_map.remove(&path.id);
        for (_, v) in index_map.iter_mut() {
            if *v > idx {
                *v -= 1;
            }
        }
    }

    true
}
/// Remove all caches images
pub fn reset_cache() {
    if let Ok(mut list) = MODULES_IMAGE_CACHE.write() {
        let length = list.len();
        let mut final_list = Vec::new();
        for _ in 0..length {
            final_list.push(Vec::new());
        }
        *list = final_list;
    }
}
// ///  Get the cached metric
// pub fn get_image_cache_by_id(id: &str) -> Option<BufferCacheInfo> {
//     let idx = get_idx_of_id(id)?;
//     let cache = MODULES_IMAGE_CACHE.read().ok()?;
//     cache.get(idx)?.clone()
// }

fn resolve_buffer_state(
    current: &Vec<BufferState>,
    id: usize,
) -> Option<std::sync::Arc<Buffer>> {
    match current.get(id)? {
        BufferState::Empty => None,
        BufferState::FoundAt(new_id) => resolve_buffer_state(current, *new_id),
        BufferState::Filled {
            image,
            ..
        } => Some(image.clone()),
    }
}
/// Get the cached image for the current module
pub fn get_image_cache(
    module_idx: usize,
    buffer_id: usize,
) -> Option<std::sync::Arc<Buffer>> {
    let cache = MODULES_IMAGE_CACHE.read().ok()?;
    let module_cache = cache.get(module_idx)?;
    resolve_buffer_state(module_cache, buffer_id)
}

/// Ensure a module's cache is properly initialized with the given capacity
pub fn ensure_module_cache_capacity(
    module_idx: usize,
    capacity: usize,
) -> Option<()> {
    let mut cache = MODULES_IMAGE_CACHE.write().ok()?;

    // Ensure module cache exists
    while cache.len() <= module_idx {
        cache.push(Vec::new());
    }

    let module_cache = &mut cache[module_idx];

    // Ensure buffer cache has the required capacity
    while module_cache.len() < capacity {
        module_cache.push(BufferState::Empty);
    }

    Some(())
}
fn replace_referenced(module: &mut [BufferState], idx: usize) {
    let (image, mut refs) =
        match std::mem::replace(&mut module[idx], BufferState::Empty) {
            BufferState::Filled {
                image,
                referenced_by,
            } if !referenced_by.is_empty() => (image, referenced_by),
            other => {
                module[idx] = other;
                return;
            }
        };

    let first = refs.remove(0);

    if !refs.is_empty() {
        for index in &refs {
            module[*index] = BufferState::FoundAt(idx);
        }
    }

    module[first] = BufferState::Filled {
        image,
        referenced_by: refs,
    };
}

/// Replace the cached image with another
pub fn insert_into_image_cache(
    idx: usize,
    id: usize,
    buffer: Buffer,
    insertion_mode: InsertionMode,
) -> Option<()> {
    //println!("Idx {} from {}", idx, id);
    let mut cache = MODULES_IMAGE_CACHE.write().ok()?;

    if cache.len() <= idx {
        cache.resize_with(idx + 1, Vec::new);
    }
    if cache[idx].len() <= id {
        cache[idx].resize_with(id + 1, || BufferState::Empty);
    }

    match insertion_mode {
        InsertionMode::Simple => {
            replace_referenced(&mut cache[idx], id);
            cache[idx][id] = BufferState::Filled {
                image: std::sync::Arc::new(buffer),
                referenced_by: Vec::new(),
            };
        }
        InsertionMode::SearchInSameModule => {
            let buffer_rc = std::sync::Arc::new(buffer);
            for (existing_id, existing_state) in cache[idx].iter().enumerate() {
                if let BufferState::Filled {
                    image,
                    ..
                } = existing_state
                    && std::sync::Arc::ptr_eq(image, &buffer_rc)
                {
                    cache[idx][id] = BufferState::FoundAt(existing_id);
                    if let BufferState::Filled {
                        referenced_by,
                        ..
                    } = &mut cache[idx][existing_id]
                    {
                        referenced_by.push(id);
                    }
                    return Some(());
                }
            }
            cache[idx][id] = BufferState::Filled {
                image: buffer_rc,
                referenced_by: Vec::new(),
            };
        }
        InsertionMode::ReplaceAll => {
            let cache_length = cache[idx].len();
            let references = if cache_length < 2 {
                Vec::new()
            } else {
                (1..cache_length).collect()
            };
            cache[idx][0] = BufferState::Filled {
                image: std::sync::Arc::new(buffer),
                referenced_by: references,
            };
            for existing_state in cache[idx].iter_mut().skip(1) {
                *existing_state = BufferState::FoundAt(0);
            }
        }
        InsertionMode::CloneAcrossIds(targets) => {
            if !targets.is_empty() {
                let first_id =
                    unsafe { *targets.iter().min().unwrap_unchecked() };
                if cache[idx].len() <= first_id {
                    cache[idx].resize_with(first_id + 1, || BufferState::Empty);
                }

                let buffer_rc = std::sync::Arc::new(buffer);

                cache[idx][first_id] = BufferState::Filled {
                    image: buffer_rc,
                    referenced_by: targets
                        .iter()
                        .copied()
                        .filter(|&id| id != first_id)
                        .collect(),
                };

                for &target_id in &targets {
                    if target_id == first_id {
                        continue;
                    }
                    if cache[idx].len() <= target_id {
                        cache[idx]
                            .resize_with(target_id + 1, || BufferState::Empty);
                    }
                    cache[idx][target_id] = BufferState::FoundAt(first_id);
                }
            }
        }
    }
    Some(())
}
#[derive(Debug, Clone)]
/// At what state a buffer can be in
pub enum BufferState {
    /// No buffer in buffer -> Create a new one
    Empty,
    /// Buffer! But it is potentially referenced by other slots
    Filled {
        /// The actual buffer
        image: std::sync::Arc<Buffer>,
        /// The slots that reference this one
        referenced_by: Vec<usize>,
    },
    /// Buffer! But somewhere else
    FoundAt(usize),
}
/// How a module image should be inserted into the cache
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InsertionMode {
    #[deprecated = "This causes visual desync, please use InsertionMode::ReplaceAll until it is fixed"]
    /// Insert without any duplicate checking
    Simple,
    /// See if any other buffer this module has produced is equivalent to this one
    SearchInSameModule,
    /// When the image should be synced across all gui instances
    ReplaceAll,
    /// Sync a single image across the specified container ids
    CloneAcrossIds(Vec<usize>),
}

/// Checks if any module needs to be redrawn
pub fn needs_redraw() -> bool {
    if let Ok(modules) = MODULES.read() {
        for module in modules.iter() {
            if module.need_redraw() {
                return true;
            }
        }
    }
    false
}
/// Get a module by name and execute a function on it (immutable access)
pub fn get_module_raw<R>(
    name: u32,
    f: impl FnOnce(&dyn DearMirlGuiModule) -> R,
) -> Option<R> {
    let idx = get_idx_of_id(name)?;
    let modules = MODULES.read().ok()?;
    let module_container = modules.get(idx)?;
    Some(module_container.with_ref(f))
}

/// Get a module by name and execute a function on it (mutable access)
pub fn get_module_raw_mut<R>(
    path: u32,
    f: impl FnOnce(&mut dyn DearMirlGuiModule) -> R,
) -> Option<R> {
    let idx = get_idx_of_id(path)?;
    let modules = MODULES.read().ok()?;
    let module_container = modules.get(idx)?;
    Some(module_container.with_ref_mut(f))
}

/// Get a module by its path and execute a function on it (immutable access)
///
/// Usage Example:
/// ```ignore
///                            // Module Type, Function Return, Module ID, Function
///                            // v                       v     v            v
/// let progress = get_module_as::<crate::modules::Slider, f32> (&module_path, |slider| slider.progress).unwrap();
/// ```
/// # Errors
/// When the given module cannot be found or the given module type is wrong
pub fn get_module_as<T: 'static + WhatAmI, R>(
    module_path: &ModulePath<T>,
    f: impl FnOnce(&T) -> R,
) -> Result<R, GuiReturnModuleError> {
    let module = get_module_raw(module_path.id, |module| {
        (
            module.as_any().downcast_ref::<T>().map(f),
            std::any::type_name::<T>(),
            module.what_am_i(),
        )
    });
    module.map_or_else(
        || {
            Err({
                GuiReturnModuleError::UnableToFindID(
                    module_path.id,
                    std::any::type_name::<T>().to_string(),
                )
            })
        },
        |value| {
            let (val, wrongly_used, correct) = value;
            val.map_or_else(
                || {
                    Err(GuiReturnModuleError::CastingAsWrongModule {
                        wrong: wrongly_used.to_string(),
                        correct: correct.to_string(),
                        id: module_path.id,
                    })
                },
                |output| Ok(output),
            )
        },
    )
}

/// Get a module by its path and execute a function on it (mutable access)
///
/// Usage Example:
/// ```ignore
///                               // Module Type, Function Return, Module ID, Function
///                               // v                       v     v           v
/// let result = get_module_as_mut::<crate::modules::Slider, ()>  (&slider_path, |slider| {
///     slider.progress += delta_time as f32 / 10.0;
///     slider.progress = slider.progress.clamp(0.0, 1.0);
///     if slider.progress == 1.0 {
///         slider.progress = 0.0
///     }
/// });
/// ```
/// # Errors
/// When the given module cannot be found or the given module type is wrong
pub fn get_module_as_mut<T: 'static, R>(
    module_path: &ModulePath<T>,
    f: impl FnOnce(&mut T) -> R,
) -> Result<R, GuiReturnModuleError> {
    let module = get_module_raw_mut(module_path.id, |module| {
        (
            module.as_any_mut().downcast_mut::<T>().map(f),
            std::any::type_name::<T>().to_string(),
            //module.what_am_i().to_string(),
            mirl::misc::type_name_of_val(&module).to_string(),
        )
    });
    module.map_or_else(
        || {
            Err({
                GuiReturnModuleError::UnableToFindID(
                    module_path.id,
                    std::any::type_name::<T>().to_string(),
                )
            })
        },
        |value| {
            let (val, wrongly_used, correct) = value;
            val.map_or_else(
                || {
                    Err(GuiReturnModuleError::CastingAsWrongModule {
                        wrong: wrongly_used,
                        correct,
                        id: module_path.id,
                    })
                },
                |output| Ok(output),
            )
        },
    )
}
/// Apply a new formatting to all modules
pub fn apply_formatting_to_modules(formatting: &Formatting) {
    if let Ok(mut modules) = MODULES.write() {
        for module in modules.iter_mut() {
            module.apply_new_formatting(formatting);
        }
    }
}

/// Apply the globally defined default formatting to all modules
pub fn apply_default_formatting_to_modules() {
    if let Ok(mut modules) = MODULES.write() {
        let formatting = get_formatting();

        for module in modules.iter_mut() {
            module.apply_new_formatting(&formatting);
        }
    }
}
