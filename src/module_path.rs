#[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// The id used to get set module
pub struct ModulePath<T> {
    /// The path
    pub id: u32,
    /// So the struct saves the type
    pub phantom: std::marker::PhantomData<T>,
    /// An optional name
    pub name: String,
}
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
            name: String::new(),
        }
    }
    #[must_use]
    /// Create a new module path
    pub const fn const_new(id: u32) -> Self {
        Self {
            id,
            phantom: std::marker::PhantomData,
            name: String::new(),
        }
    }
    #[must_use]
    /// Get the path of the module
    pub const fn id(&self) -> u32 {
        self.id
    }
    #[must_use]
    /// Give the path a name
    pub fn with_name(mut self, name: String) -> Self {
        self.name = name;
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
