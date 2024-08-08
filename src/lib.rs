mod random;

pub use random::{ConstOr, Switch, UniformRange, UniformResource};

/// A generic value generator trait.
pub trait Generator<T> {
    /// Generates a value of type `T`.
    fn generate(&mut self) -> T;
}

/// Trait representing a range type generator.
///
/// The `Range` trait defines an interface for generating a random sample
/// from a specified range of values.
pub trait Range<T>: Generator<T> {
    /// Sets the range of values to generate.
    fn set_range(&mut self, lb: T, rb: T);
}

/// Trait representing a resource type generator.
///
/// The `Resource` trait defines an interface for managing a collection
/// of values and generating random samples from this collection.
pub trait Resource<T>: Generator<T> {
    /// Checks if the resource is empty.
    fn is_empty(&self) -> bool;

    /// Adds a value to the resource.
    fn add(&mut self, value: T);

    /// Consumes a value from the resource.
    fn consume(&mut self, value: &T);
}

/// Constant generator.
pub struct Constant<T>(pub T);

impl<T> Generator<T> for Constant<T>
where
    T: Clone,
{
    fn generate(&mut self) -> T {
        self.0.clone()
    }
}
