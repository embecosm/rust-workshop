use crate::{Animal, Error, Result};
use serde::{Deserialize, Serialize};

/// Nothing to change here
///
/// `Cage` is mostly a wrapper for the type `Vec<Animal>`, where we can
/// implement methods and traits on.
///
/// For example the derived `Serialize` and `Deserialize` allows us to
/// serialize this struct with `serde`.
#[derive(Serialize, Deserialize, Clone, Default, Debug, Eq, PartialEq)]
pub struct Cage {
    pub animals: Vec<Animal>,
}

// Nothing to change here
//
// For wrapper types like this, it's recommended to implement the `From` trait
// for the wrapped type. This will give you an "constructor"
// `Cage::from(Vec<Animal>)` and in addition the method `into()` for free. With
// this you can for example write `let x: Cage = animal_vec.into()`.
impl From<Vec<Animal>> for Cage {
    fn from(animals: Vec<Animal>) -> Self {
        Self { animals }
    }
}

impl Cage {
    /// `move_from` will move the animals from the passed `Cage` to `self`.
    ///
    /// As you can see by the absent `&` before `Cage`, this method literally
    /// moves the `Cage` into the method and takes ownership of the animals.
    ///
    /// # Hint
    ///
    /// This method should be a one-liner. Take a look at the `Vec`
    /// documentation to find out how to `extend` one vector with another.
    pub fn move_from(&mut self, cage: Cage) {
        // let _ = cage; // suppress warning
        self.animals.extend(cage.animals);
    }

    /// `weakest` will return the weakest animal in `self`.
    ///
    /// # Hint
    ///
    /// Since you have implemented `Ord` on `Animal`, you can use the `min()`
    /// method from the `Iterator` trait.
    pub fn weakest(&self) -> Option<&Animal> {
        // None
        self.animals.iter().min()
    }

    /// `strongest` will return the strongest animal in `self`.
    ///
    /// # Hint
    ///
    /// Since you have implemented `Ord` on `Animal`, you can use the `max()`
    /// method from the `Iterator` trait.
    pub fn strongest(&self) -> Option<&Animal> {
        // None
        self.animals.iter().max()
    }

    /// `fits` will determine if the passed herbivore can be put in `self`.
    ///
    /// A herbivore can be placed into a cage, if the strongest animal in the
    /// cage is weaker (`<`) than the herbivore.
    ///
    /// # Hint
    ///
    /// `Option` can be seen as an `Iterator` with either 1 or 0 elements. This
    /// means you can call `Iterator` methods, like `map` on an `Option`, which
    /// converts the type in the option to any type you want.
    pub fn fits(&self, herbivore: &Animal) -> bool {
        assert!(!herbivore.carnivore);
        // false
        self.strongest()
            .map(|strongest_animal| strongest_animal < herbivore)
            .unwrap_or(true)
    }

    /// BONUS: `deliver_food` will move the food from `food_cage` to `self`.
    ///
    /// # Errors
    ///
    /// This method errors, if an animal in the `food_cage` is stronger than
    /// the weakest animal in `self`.
    pub fn deliver_food(&mut self, food_cage: Cage) -> Result<()> {
        // Ok(())
        if let (Some(strongest_food), Some(weakest_animal))= (food_cage.strongest(), self.weakest()) {
            if food_cage.strongest() >= self.weakest() {
                return Err(Error::FoodTooStrong(
                    strongest_food.clone(),
                    weakest_animal.clone(),
                ));
            }
        }

        self.move_from(food_cage);

        Ok(())
    }
}
