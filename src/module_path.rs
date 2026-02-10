#[cfg(feature = "module_path_naming")]
/// When no name has been set
pub const NO_NAME: [char; 8] = ['N', 'o', ' ', 'n', 'a', 'm', 'e', '!'];
// const INVALID_NAME: [char; 8] = ['I', 'n', 'v', 'a', 'l', 'i', 'd', '!'];

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// The id used to get set module
pub struct ModulePath<T> {
    /// The path
    pub id: u32,
    /// So the struct saves the type
    pub phantom: std::marker::PhantomData<T>,
    #[cfg(feature = "module_path_naming")]
    /// An optional name
    pub name: [char; 8],
}
impl<T: Clone> Copy for ModulePath<T> {}

impl<T> ModulePath<T> {
    #[must_use]
    /// Create a new module path
    pub fn new() -> Self {
        Self {
            id: mirl::graphics::generate_random_color((
                (0, 0, 0),
                (255, 255, 255),
            )),
            phantom: std::marker::PhantomData,
            #[cfg(feature = "module_path_naming")]
            name: NO_NAME,
        }
    }
    #[must_use]
    /// Create a new module path
    pub const fn const_new(id: u32) -> Self {
        Self {
            id,
            phantom: std::marker::PhantomData,
            #[cfg(feature = "module_path_naming")]
            name: NO_NAME,
        }
    }
    #[must_use]
    /// Get the path of the module
    pub const fn id(&self) -> u32 {
        self.id
    }
    #[must_use]
    #[cfg(feature = "module_path_naming")]
    /// Give the path a name
    ///
    /// The name must be 8 characters or less, remaining characters will be ignored
    pub fn with_name(mut self, name: &str) -> Self {
        let mut chars = [' '; 8];

        for (i, c) in name.chars().take(8).enumerate() {
            chars[i] = c;
        }

        self.name = chars;
        self
    }
}
impl<T> Default for ModulePath<T> {
    fn default() -> Self {
        Self::new()
    }
}

// impl<T> std::ops::Deref for ModulePath<T> {
//     type Target = String;
//     fn deref(&self) -> &Self::Target {
//         &self.id
//     }
// }
