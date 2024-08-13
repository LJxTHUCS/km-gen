/// Random-based generators.
mod random;

/// A generic value generator trait.
pub trait Generator<T> {
    /// Try generates a value of type `T`, returning `None` if it fails.
    fn try_generate(&mut self) -> Option<T>;

    /// Generates a value of type `T`, may panic if it fails.
    fn generate(&mut self) -> T {
        self.try_generate().expect("Failed to generate value")
    }
}

/// Constant generator.
pub struct Constant<T>(T);

impl<T> Constant<T> {
    /// Create a new constant generator.
    pub fn new(value: T) -> Self {
        Self(value)
    }
    /// Set the value of the generator.
    pub fn set(&mut self, value: T) {
        self.0 = value;
    }
}

impl<T> Generator<T> for Constant<T>
where
    T: Clone,
{
    fn try_generate(&mut self) -> Option<T> {
        Some(self.0.clone())
    }
}

/// Default-or generator.
///
/// Generates the default value of type `T` if the wrapped generator fails.
pub struct DefaultOr<T, G>(T, G);

impl<T, G> DefaultOr<T, G> {
    /// Create a new default-or generator.
    pub fn new(default: T, generator: G) -> Self {
        Self(default, generator)
    }
    /// Set the default value of the generator.
    pub fn set_default(&mut self, default: T) {
        self.0 = default;
    }
}

impl<T, G> Generator<T> for DefaultOr<T, G>
where
    G: Generator<T>,
    T: Clone,
{
    fn try_generate(&mut self) -> Option<T> {
        self.1.try_generate().or(Some(self.0.clone()))
    }
}

pub use random::{RandomFlags, RandomSwitch, SwitchConstant, UniformCollection, UniformRange};
