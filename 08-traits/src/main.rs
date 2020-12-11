// Traits give Rust a bit of an object oriented character. Traits can be seen as interfaces in
// e.g. Java. You can implement functions in traits directly or leaf them open to the implementor.
// The later are called "required functions" in a trait, that have to be implemented.
//
// For example the `Iterator` trait has exactly one required function: `next`. Every other method
// is implmented by using the `next` method.
trait AnimalBehavior {
    // A function without a body (`{ ... }`) is a "required function"
    fn sound(&self) -> String;

    // The keyword `Self` (uppercase) describes the implementing type. This is especially useful
    // for traits.
    fn walk<A: AnimalBehavior>(&mut self, other: &A);

    fn be_annoyed(&mut self);

    fn location(&self) -> (u32, u32);

    // This is an optional function, because it already has a default implementation.
    fn annoy<A: AnimalBehavior>(&mut self, other: &mut A) {
        self.walk(other);

        println!("{}", self.sound().to_uppercase());
        println!("{}", self.sound().to_uppercase());
        println!("{}", self.sound().to_uppercase());
        println!("{}", self.sound().to_uppercase());
        println!("{}", self.sound().to_uppercase());

        other.be_annoyed();
    }
}

#[derive(Debug)]
struct Cat {
    x: u32,
    y: u32,
    annoyed: bool,
}

#[derive(Debug, Copy, Clone)]
struct Dog {
    x: u32,
    y: u32,
    annoyed: bool,
}

// This doesn't implement all methods yet. Add those methods to make the compiler pass
impl AnimalBehavior for Cat {
    fn sound(&self) -> String {
        String::from("miaow")
    }

    fn walk<A: AnimalBehavior>(&mut self, other: &A) {
        let (x, y) = other.location();
        self.x = x + 2;
        self.y = y + 2;
    }

    fn be_annoyed(&mut self) {
        self.annoyed = true;
    }

    fn location(&self) -> (u32, u32) {
        (self.x, self.y)
    }
}

impl AnimalBehavior for Dog {
    fn sound(&self) -> String {
        String::from("woof")
    }

    fn walk<A: AnimalBehavior>(&mut self, other: &A) {
        let (x, y) = other.location();
        self.x = x;
        self.y = y + 1;
    }

    fn be_annoyed(&mut self) {
        self.annoyed = true;
    }

    fn location(&self) -> (u32, u32) {
        (self.x, self.y)
    }

    // Dogs are never annoying, so we reimplement this method, so that it just does nothing.
    fn annoy<A: AnimalBehavior>(&mut self, _other: &mut A) {}
}

fn animals_do_animal_things<A, B>(animal_1: &mut A, animal_2: &mut B)
where
    A: AnimalBehavior,
    B: AnimalBehavior,
{
    animal_1.annoy(animal_2);
}

// You can also implement traits for your types from other crates/libraries, like `std`. For
// example the `Add` trait allows the usage of the `+` operator with `Dog`s.
impl std::ops::Add for Dog {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            annoyed: false,
        }
    }
}

// What you *can't* do is implement a foreign trait for a foreign type. One of the two have to be
// defined in your own crate.
// impl std::ops::Add for Vec<usize> {
//     type Output = usize;
//
//     fn add(self, rhs: Vec<usize>) -> Self::Output {
//         self.iter().zip(&rhs).fold(0, |acc, (x, y)| acc + x + y)
//     }
// }

// To do this, you usually implement a wrapper type in combination with the `Deref` trait.
struct USizeContainer(Vec<usize>);

/// The `Deref` trait implements the `*T` unary operator. This is especially useful because in Rust
/// calling methods automatically dereferences the input until this method is found.
///
/// ```rust
/// let x: &f32 = &1.0;
///
/// (*x).sqrt();
/// // is equivalent to
/// x.sqrt();
/// // both call the `sqrt` method of the `f32` type, not the `&f32` type.
/// ```
impl std::ops::Deref for USizeContainer {
    type Target = Vec<usize>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::Add for USizeContainer {
    type Output = usize;

    fn add(self, rhs: USizeContainer) -> Self::Output {
        // Because we implemented `Deref` we can call `iter` directly on `self`, even though we
        // never implemented this method on `USizeContainer`.
        self.iter().zip(&*rhs).fold(0, |acc, (x, y)| acc + x + y)
    }
}

fn main() {
    let mut annoying_cat = Cat {
        x: 10,
        y: 5,
        annoyed: false,
    };
    let mut cat = Cat {
        x: 20,
        y: 15,
        annoyed: false,
    };
    let mut dog = Dog {
        x: 0,
        y: 12,
        annoyed: false,
    };
    println!("Cat1: {:?}; Cat2: {:?}", annoying_cat, cat);

    animals_do_animal_things(&mut annoying_cat, &mut cat);

    println!("Cat1: {:?}; Cat2: {:?}", annoying_cat, cat);
    println!("Dog: {:?}", dog);

    // Let the cat annoy the dog. Why doen't this work?
    // Hint: Try to use some more generics.
    animals_do_animal_things(&mut annoying_cat, &mut dog);

    println!("Cat: {:?}; Dog: {:?}", annoying_cat, dog);

    println!("2*Dog: {:?}", dog + dog);
}
