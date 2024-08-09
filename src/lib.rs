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
pub struct Constant<T>(pub T);

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
pub struct DefaultOr<T, G>(pub G, pub T);

impl<T, G> Generator<T> for DefaultOr<T, G>
where
    G: Generator<T>,
    T: Clone,
{
    fn try_generate(&mut self) -> Option<T> {
        self.0.try_generate().or(Some(self.1.clone()))
    }
}

pub use random::{Switch, UniformCollection, UniformRange};
