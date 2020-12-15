mod animal;
mod cage;
mod error;

use std::cmp::Reverse;

pub use animal::Animal;
pub use cage::Cage;
pub use error::Result;

/// `animal_groups` will take an iterator an group animals by species into cages.
///
/// # Hints
///
/// - To group things, a `HashMap<String, Vec<Animal>>` is really useful
/// - Take a look at the `Entry` API of the `hash_map` module
/// - To get a vector of cages, you can use the `Cage::from` function on the
///   `Vec`s in the `HashMap`
///
/// # What's this weird argument type?
///
/// Don't worry about this for now, we'll talk about it after the break. It
/// basically means that this function takes some `Iterator` which you can then
/// use in e.g. a `for`-loop:
fn animal_groups<'a>(animals: impl Iterator<Item = &'a Animal>) -> Vec<Cage> {
    let _ = animals; // suppress warning
    vec![]
}

/// `sorted_animal_groups` does the same as (and can reuse) `animal_groups`.
///
/// # Hint
///
/// To sort a vector with a key, take a look at `Vec::sort_by_key`.
fn sorted_animal_groups<'a, K, F>(animals: impl Iterator<Item = &'a Animal>, key: F) -> Vec<Cage>
where
    F: Fn(&Cage) -> K,
    K: Ord,
{
    let _ = animals; // suppress warning
    let _ = key; // suppress warning
    vec![]
}

/// `fitting_cage` tries to `find` a fitting cage for the passed herbivore.
///
/// # Hint
///
/// With the use of iterators, this is a one-liner
fn fitting_cage<'a>(cages: &'a mut [Cage], herbivore: &Animal) -> Option<&'a mut Cage> {
    assert!(!herbivore.carnivore);
    let _ = cages; // suppress warning
    None
}

/// BONUS: `extract_food` tries to extract the food animals from the passed animals.
///
/// If `food` is `None` just return the passed animals.
///
/// If `Some` `food` is passed, check if the animals has the specified species
/// and return an `Err`, if not.
///
/// # Errors
///
/// This function errors if there are no `animals` of the `food` species.
/// (If a function returns `Result`, you should put an `# Errors` section in
/// the documentation)
fn extract_food(
    animals: Vec<Animal>,
    food: Option<&str>,
) -> Result<(Option<Vec<Animal>>, Vec<Animal>)> {
    let _ = food; // suppress warning
    Ok((None, animals))
}

// You won't have to modify the following functions. They should Just Work™.

/// Nothing to change here.
///
/// `cage_em_all` contains the algorithm for the caging.
///
/// But you'll have to fill out the other function stubs in this files and the
/// other modules for this to work.
pub fn cage_em_all(animals: Vec<Animal>, food: Option<&str>) -> Result<Vec<Cage>> {
    let (food_animals, filtered_animals) = extract_food(animals, food)?;

    let mut carnivore_groups = sorted_animal_groups(
        filtered_animals.iter().filter(|animal| animal.carnivore),
        |cage| {
            Reverse(
                cage.strongest()
                    .expect("contains at least one carnivore")
                    .clone(),
            )
        },
    );

    let herbivore_groups =
        animal_groups(filtered_animals.iter().filter(|animal| !animal.carnivore));

    if let Some(food_animals) = food_animals {
        carnivore_groups[0].deliver_food(Cage::from(food_animals))?;
    }

    Ok(relocate_animals(carnivore_groups, herbivore_groups))
}

/// Nothing to change here.
///
/// `relocate_animals` takes the split up carnivores and herbivores and puts
/// them in the correct cages.
fn relocate_animals(mut carnivore_cages: Vec<Cage>, herbivore_cages: Vec<Cage>) -> Vec<Cage> {
    let mut weak_herbivores = Cage::default();

    for herbivores in herbivore_cages {
        let weakest_herbivore = herbivores
            .weakest()
            .expect("contains at least one herbivore");

        if let Some(cage) = fitting_cage(&mut carnivore_cages, weakest_herbivore) {
            cage.move_from(herbivores);
        } else {
            weak_herbivores.move_from(herbivores);
        }
    }

    carnivore_cages.push(weak_herbivores);

    carnivore_cages
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn animal_grouping() {
        let a1 = Animal {
            name: String::from("A1"),
            species: String::from("S1"),
            strength: 10,
            carnivore: false,
        };
        let a2 = Animal {
            name: String::from("A2"),
            species: String::from("S1"),
            strength: 11,
            carnivore: false,
        };
        let b1 = Animal {
            name: String::from("B1"),
            species: String::from("S2"),
            strength: 5,
            carnivore: true,
        };
        let b2 = Animal {
            name: String::from("B2"),
            species: String::from("S2"),
            strength: 6,
            carnivore: true,
        };

        let animals = vec![a1.clone(), a2.clone(), b1.clone(), b2.clone()];

        let grouping = animal_groups(animals.iter());

        let cage1 = Cage {
            animals: vec![a1, a2],
        };
        let cage2 = Cage {
            animals: vec![b1, b2],
        };

        assert_eq!(grouping, vec![cage1, cage2]);
    }

    #[test]
    fn sorted_animal_grouping() {
        let a1 = Animal {
            name: String::from("A1"),
            species: String::from("S1"),
            strength: 9,
            carnivore: true,
        };
        let a2 = Animal {
            name: String::from("A2"),
            species: String::from("S1"),
            strength: 11,
            carnivore: true,
        };
        let b1 = Animal {
            name: String::from("B1"),
            species: String::from("S2"),
            strength: 10,
            carnivore: true,
        };
        let b2 = Animal {
            name: String::from("B2"),
            species: String::from("S2"),
            strength: 8,
            carnivore: true,
        };

        let animals = vec![a1.clone(), a2.clone(), b1.clone(), b2.clone()];

        // Sort from weak to strong
        let grouping =
            sorted_animal_groups(animals.iter(), |cage| cage.strongest().unwrap().clone());

        let cage1 = Cage {
            animals: vec![a1, a2],
        };
        let cage2 = Cage {
            animals: vec![b1, b2],
        };

        assert_eq!(grouping, vec![cage2, cage1]);
    }

    #[test]
    fn fits_cage() {
        let a1 = Animal {
            name: String::from("A1"),
            species: String::from("S1"),
            strength: 9,
            carnivore: true,
        };
        let a2 = Animal {
            name: String::from("A2"),
            species: String::from("S1"),
            strength: 11,
            carnivore: true,
        };
        let b1 = Animal {
            name: String::from("B1"),
            species: String::from("S2"),
            strength: 10,
            carnivore: false,
        };
        let b2 = Animal {
            name: String::from("B2"),
            species: String::from("S2"),
            strength: 11,
            carnivore: false,
        };
        let b3 = Animal {
            name: String::from("B3"),
            species: String::from("S2"),
            strength: 12,
            carnivore: false,
        };

        let cage1 = Cage {
            animals: vec![a1, a2],
        };

        assert!(fitting_cage(&mut [cage1.clone()], &b1).is_none());
        assert!(fitting_cage(&mut [cage1.clone()], &b2).is_none());
        assert!(fitting_cage(&mut [cage1], &b3).is_some());
    }

    #[test]
    fn food_extraction() {
        let a1 = Animal {
            name: String::from("A1"),
            species: String::from("S1"),
            strength: 10,
            carnivore: true,
        };
        let b1 = Animal {
            name: String::from("B1"),
            species: String::from("S2"),
            strength: 9,
            carnivore: false,
        };

        assert_eq!(
            extract_food(vec![a1.clone()], None).unwrap(),
            (None, vec![a1.clone()])
        );
        assert_eq!(
            extract_food(vec![a1.clone(), b1.clone()], Some("S2")).unwrap(),
            (Some(vec![b1.clone()]), vec![a1.clone()])
        );
        assert!(extract_food(vec![a1], Some("S2")).is_err());
    }

    #[test]
    fn food_delivery() {
        let a1 = Animal {
            name: String::from("A1"),
            species: String::from("S1"),
            strength: 10,
            carnivore: true,
        };
        let b1 = Animal {
            name: String::from("B1"),
            species: String::from("S2"),
            strength: 9,
            carnivore: false,
        };
        let b2 = Animal {
            name: String::from("B2"),
            species: String::from("S2"),
            strength: 10,
            carnivore: false,
        };
        let b3 = Animal {
            name: String::from("B3"),
            species: String::from("S2"),
            strength: 11,
            carnivore: false,
        };

        let mut cage1 = Cage { animals: vec![a1] };
        let food_cage1 = Cage { animals: vec![b1] };
        let food_cage2 = Cage { animals: vec![b2] };
        let food_cage3 = Cage { animals: vec![b3] };

        assert!(cage1.deliver_food(food_cage1).is_ok());
        assert!(cage1.deliver_food(food_cage2).is_err());
        assert!(cage1.deliver_food(food_cage3).is_err());
    }
}
