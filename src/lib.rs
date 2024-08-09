mod random;

pub use random::{ConstOr, Switch, UniformCollection, UniformRange};

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
