use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

/// Nothing to change here.
///
/// `Animal` represents an animal in the JSON file.
///
/// By deriving `Serialize` and `Deserialize`, `serde` can use this struct for
/// (de)serialization.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Animal {
    pub name: String,
    pub species: String,
    pub strength: usize,
    pub carnivore: bool,
}

// You can implement traits like `PartialEq` for your types.
//
// With this it is possible to compare animals with `==` and `!=`. In this small
// project, we only care about the `species` when comparing animals.
impl PartialEq for Animal {
    fn eq(&self, other: &Self) -> bool {
        let _ = other; // suppress warning
        false
    }
}

// We order `Animal`s only by their strength. This should be a one-liner. See
// the `Ord` documentation for ideas how to implement this.
impl Ord for Animal {
    fn cmp(&self, other: &Self) -> Ordering {
        let _ = other; // suppress warning
        Ordering::Equal
    }
}

// Nothing to change here.
//
// The `Eq` trait is a marker trait, that tells users of the type that `x == x`
// always evaluates to `true`. This is not checked by the compiler, but by
// implementing this trait the programmer guarantees that the `PartialEq`
// implementation meets this condition.
//
// An example for a type, that doesn't implement `Eq` is `f32`/`f64`, because
// `f32::NAN != f32::NAN` by definition.
impl Eq for Animal {}

// Nothing to change here.
//
// See the `Ord` documentation, why this is implemented like this
impl PartialOrd for Animal {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
