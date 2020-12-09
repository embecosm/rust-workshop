use std::fmt::{Debug, Display, Formatter};

use crate::Animal;

/// `Error` is the error type of this crate.
///
/// Usually crates define their own Error type which then can be used in other
/// crates for error handling.
///
/// Add more variants here, as you see fit.
pub enum Error {
    IOError(std::io::Error),
    Serialization(serde_json::Error),
    // Add custom variants here
    NoFood(String),
    FoodTooStrong(Animal, Animal),
}

// You should implement the `Display` trait for your error, so that crates can
// print your errors in a human readable way.
//
// If you add more variants to `Error`, you also have to deal with them here.
// But don't worry about forgetting to do this. The compiler will tell you :)
impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IOError(err) => write!(f, "{}", err),
            Self::Serialization(err) => write!(f, "{}", err),
            // Deal with more variants here
            Self::NoFood(food) => write!(f, "There is no food species with the name {}!", food),
            Self::FoodTooStrong(food, weak) => write!(
                f,
                "`{food_name}` of species `{food_species}` is to strong for `{weak_name}` of species `{weak_species}`.",
                food_name=food.name,
                food_species=food.species,
                weak_name=weak.name,
                weak_species=weak.species
            ),
        }
    }
}

/// Nothing to change here.
///
/// `Result` is the result type of a struct.
///
/// If you have an `Error` type, you should also have your own `Result` type
pub type Result<T> = std::result::Result<T, Error>;


// Nothing to change here.
//
// If your crates doesn't deal with errors from other crates, they can be
// collected in the Error type of the crate. The most comfortable way to do this
// is to implement the `From` trait for these errors.
impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Self::IOError(err)
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Self::Serialization(err)
    }
}

// Nothing to change here.
//
// This is kind of an antipattern for errors, but it simplifies tings for this
// example program.
impl Debug for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
