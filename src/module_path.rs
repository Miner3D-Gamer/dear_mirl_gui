#[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// The id used to get set module
pub struct ModulePath<T> {
    /// The path
    pub id: String,
    /// So the struct saves the type
    pub phantom: std::marker::PhantomData<T>,
}
impl<T> ModulePath<T> {
    #[must_use]
    /// Create a new module path
    pub fn new(id: &str) -> Self {
        Self {
            id: id.to_string(),
            phantom: std::marker::PhantomData,
        }
    }
    #[must_use]
    /// Create a new module path
    pub const fn const_new(id: String) -> Self {
        Self {
            id,
            phantom: std::marker::PhantomData,
        }
    }
    #[must_use]
    /// Get the path of the module
    pub fn id(&self) -> String {
        self.id.clone()
    }
}

// impl<T> std::ops::Deref for ModulePath<T> {
//     type Target = String;
//     fn deref(&self) -> &Self::Target {
//         &self.id
//     }
// }
