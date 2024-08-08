use crate::{Constant, Generator, Range, Resource};
use rand::{distributions::uniform::SampleUniform, rngs::ThreadRng, Rng};

/// A uniform distribution range generator.
///
/// This structure represents a range with a close lower bound (`lb`) and an
/// open upper bound (`ub`), from which it generates a random value uniformly.
pub struct UniformRange<T> {
    lb: T,
    ub: T,
    rng: ThreadRng,
}

impl<T> UniformRange<T>
where
    T: Clone + PartialOrd,
{
    /// Creates a new `UniformSampleRange` with the specified bounds.
    pub fn new(lb: T, ub: T) -> Self {
        Self {
            lb,
            ub,
            rng: rand::thread_rng(),
        }
    }
}

impl<T> Generator<T> for UniformRange<T>
where
    T: Clone + PartialOrd + SampleUniform,
{
    /// Generates a random sample within the specified bounds.
    fn generate(&mut self) -> T {
        self.rng.gen_range(self.lb.clone()..self.ub.clone())
    }
}

impl<T> Range<T> for UniformRange<T>
where
    T: Clone + PartialOrd + SampleUniform,
{
    fn set_range(&mut self, lb: T, rb: T) {
        self.lb = lb;
        self.ub = rb;
    }
}

/// A resource generator that randomly samples from a collection of values.
///
/// This structure maintains a collection of values and provides methods
/// for adding, consuming, and sampling from these values.
pub struct UniformResource<T> {
    values: Vec<T>,
    rng: ThreadRng,
}

impl<T> UniformResource<T> {
    /// Creates a new `RandomSampleResource` with the given initial values.
    pub fn new(values: Vec<T>) -> Self {
        Self {
            values,
            rng: rand::thread_rng(),
        }
    }
}

impl<T> Generator<T> for UniformResource<T>
where
    T: Clone,
{
    /// Generates a random sample from the resource.
    fn generate(&mut self) -> T {
        let index = self.rng.gen_range(0..self.values.len());
        self.values[index].clone()
    }
}

impl<T> Resource<T> for UniformResource<T>
where
    T: Clone + PartialEq,
{
    /// Check if the resource is empty.
    fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    /// Adds a value to the resource.
    fn add(&mut self, value: T) {
        self.values.push(value);
    }

    /// Consumes a value from the resource.
    fn consume(&mut self, value: &T) {
        if let Some(index) = self.values.iter().position(|v| v == value) {
            self.values.remove(index);
        }
    }
}

/// A switch generator that randomly selects between two generators.
pub struct Switch<G1, G2> {
    gen1: G1,
    gen2: G2,
    prob: f64,
    rng: ThreadRng,
}

impl<G1, G2> Switch<G1, G2> {
    /// Creates a new `Switch` with the specified generators.
    pub fn new(gen1: G1, gen2: G2) -> Self {
        Self {
            gen1,
            gen2,
            prob: 0.5,
            rng: rand::thread_rng(),
        }
    }
    /// Set probability of selecting the first generator.
    pub fn set_g1_prob(&mut self, prob: f64) {
        if prob < 0.0 {
            self.prob = 0.0;
        } else if prob > 1.0 {
            self.prob = 1.0;
        } else {
            self.prob = prob;
        }
    }
}

impl<T, G1, G2> Generator<T> for Switch<G1, G2>
where
    G1: Generator<T>,
    G2: Generator<T>,
{
    /// Generates a random sample from one of the generators.
    fn generate(&mut self) -> T {
        if self.rng.gen_bool(self.prob) {
            self.gen1.generate()
        } else {
            self.gen2.generate()
        }
    }
}

/// A generator that switches between a constant value and another generator.
pub type ConstOr<T, G> = Switch<Constant<T>, G>;