use std::fmt::Debug;

use crate::{Constant, Generator};
use bitflags::{Bits, Flags};
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
    fn try_generate(&mut self) -> Option<T> {
        if self.lb < self.ub {
            Some(self.rng.gen_range(self.lb.clone()..self.ub.clone()))
        } else {
            None
        }
    }
}

/// A generator that randomly samples from a collection of values.
pub struct UniformCollection<T> {
    values: Vec<T>,
    rng: ThreadRng,
}

impl<T> UniformCollection<T> {
    /// Creates a new `UniformCollection` with the given initial values.
    pub fn new(values: Vec<T>) -> Self {
        Self {
            values,
            rng: rand::thread_rng(),
        }
    }

    /// Check if the collection is empty.
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
}

impl<T> Generator<T> for UniformCollection<T>
where
    T: Clone,
{
    /// Generates a random sample from the resource.
    fn try_generate(&mut self) -> Option<T> {
        if self.values.is_empty() {
            None
        } else {
            let index = self.rng.gen_range(0..self.values.len());
            Some(self.values[index].clone())
        }
    }
}

/// A switch generator that randomly selects between two generators.
pub struct RandomSwitch<G1, G2> {
    gen1: G1,
    gen2: G2,
    prob: f64,
    rng: ThreadRng,
}

impl<G1, G2> RandomSwitch<G1, G2> {
    /// Creates a new `RandomSwitch` with the specified generators.
    pub fn new(gen1: G1, gen2: G2, prob: f64) -> Self {
        let mut g = Self {
            gen1,
            gen2,
            prob,
            rng: rand::thread_rng(),
        };
        g.set_g1_prob(prob);
        g
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

impl<T, G1, G2> Generator<T> for RandomSwitch<G1, G2>
where
    G1: Generator<T>,
    G2: Generator<T>,
{
    /// Generates a random sample from one of the generators.
    fn try_generate(&mut self) -> Option<T> {
        if self.rng.gen_bool(self.prob) {
            self.gen1.try_generate()
        } else {
            self.gen2.try_generate()
        }
    }
}

/// A generator that randomly chooses between a constant value and another generator.
pub type SwitchConstant<T, G> = RandomSwitch<Constant<T>, G>;

/// A generator that randomly generates a "flags" type value.
///
/// The generator will randomly select a flag from the flag set with a probability of `prob`.
pub struct RandomFlags<T> {
    rng: ThreadRng,
    prob: f64,
    inclusion: T,
    exclusion: T,
    constraints: Vec<(T, T)>,
}

impl<T> RandomFlags<T>
where
    T: Flags,
{
    /// Creates a new `RandomFlag` with the specific flag type.
    pub fn new(prob: f64) -> Self {
        Self {
            rng: rand::thread_rng(),
            prob,
            inclusion: T::empty(),
            exclusion: T::empty(),
            constraints: Vec::new(),
        }
    }

    /// Set probability of selecting a flag.
    pub fn set_prob(&mut self, prob: f64) {
        if prob < 0.0 {
            self.prob = 0.0;
        } else if prob > 1.0 {
            self.prob = 1.0;
        } else {
            self.prob = prob;
        }
    }

    /// Include some flags in the generator. Value generated will always include these flags.
    pub fn include(&mut self, flags: T) {
        self.inclusion = T::from_bits_truncate(self.inclusion.bits() | flags.bits());
    }

    /// Exclude some flags from the generator. Value generated will never include these flags.
    pub fn exclude(&mut self, flags: T) {
        self.exclusion = T::from_bits_truncate(self.exclusion.bits() | flags.bits());
    }

    /// Add a constraint to the generator.
    ///
    /// If `flag1` is selected, then `flag2` must also be selected.
    pub fn constraint(&mut self, flag1: T, flag2: T) {
        self.constraints.push((flag1, flag2));
    }
}

impl<T> Generator<T> for RandomFlags<T>
where
    T: Flags + Debug,
{
    /// Generates a random flag value.
    fn try_generate(&mut self) -> Option<T> {
        let mut value = T::Bits::EMPTY;
        for flag in T::FLAGS.iter() {
            if self.rng.gen_bool(self.prob) {
                value = value | flag.value().bits();
            }
        }
        // Check constraints
        for (flag1, flag2) in self.constraints.iter() {
            if (value | flag1.bits()) == value {
                value = value | flag2.bits();
            }
        }
        // Check exclusions
        value = value & !self.exclusion.bits();
        // Check inclusions
        value = value | self.inclusion.bits();
        Some(T::from_bits_truncate(value))
    }
}
