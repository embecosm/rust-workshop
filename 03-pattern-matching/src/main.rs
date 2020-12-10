/// `S` is a struct with 3 fields. The syntax for fields is similar to the syntax for function
/// arguments: `name: T`
///
/// The `derive` attribute provides some functionality to the struct. In this case, those derives
/// allows comparisons of instances of `S` and the cloning of `S`. "Things" can be derived, when all
/// fields also derive or implement these "things".
#[derive(Eq, PartialEq, Clone, Debug)]
struct S {
    a: u32,
    b: String,
    c: Vec<bool>,
}

// This is an `impl` block for the struct `S` you can implement functions and methods for `S` here.
// Functions in impl blocks are also called associated functions, also known as static functions in
// other programming languages. Methods take `self` as their first argument.
impl S {
    // Constructors in Rust usually start with `new`. Because there is no overloading, you have to
    // use a different name for each constructor.
    fn new(a: u32, b: String) -> Self {
        Self { a, b, c: vec![] }
    }

    // This is a method. The first argument is a reference to `self`. But is this correct?
    fn push_bool(&mut self, b: bool) {
        self.c.push(b);
    }
}

// `E` is an enum with 4 variants. Unlike in C variants are not just integers, but can hold
// additional informations. For example, `E::B` is a "tuple-struct variant", which means that
// `E::B` has two annonymous fields. `E::C` is a "struct variant", which is different to `E::B`,
// because it's fields are named. (FYI: Both struct variants also exist outside of an enum
// context).
//
// `E` tries to derive `Debug`. Can you find out what you need to change so that `Debug` is
// derivable? Remember: The compiler is your friend!
#[derive(Eq, PartialEq, Clone, Debug)]
enum E {
    A,
    B(u32, bool),
    C { a: usize, b: String },
    D(S),
}

// Also enums can have associated functions and methods.
impl E {
    // Let's be creative and print some things depending on the different variants. Don't worry
    // about forgetting a variant, the compiler has your back!
    //
    // If you want to be more creative in the match expression, take a look at
    // https://doc.rust-lang.org/rust-by-example/flow_control/match.html
    fn creative_printing(&self) {
        match self {
            Self::A => println!("A is boring"),
            Self::B(x, _) if *x == 42 => println!("This seems to be the answer"),
            Self::B(_, b) => println!("To be or not to be? The answer is: {}", b),
            Self::C { .. } => println!("C is always cool"),
            Self::D(s) => println!("S says: {:?}", s),
        }
    }

    // In Rust you won't need things like `std::pair`, because this is encoded in the type system
    // as a tuple. Tuples can contain as many elements as you like.
    //
    // Did you notice, that `self` is moved _into_ this method?
    fn into_tuple(self) -> (u32, bool) {
        match self {
            Self::A => (0, false),
            Self::B(a, b) => (a, b),
            Self::C { a, b } => (a as u32, b.is_empty()),
            // Add the `E::D` case by destructuring `S`
            Self::D(S { a, c, .. }) => (a, c.get(0).copied().unwrap_or_default()),
        }
    }
}

fn main() {
    let e = Some(E::A);

    // Not only `match` expressions perform pattern matching, but also the `if let` syntax.
    if let Some(e) = &e {
        e.creative_printing();
    } else {
        let mut my_s = S::new(42, "Hello, Rustacean!".to_string());
        my_s.push_bool(true);
        match my_s {
            S { a, c, .. } if a > 10 => println!("{:?}", c.get(0)),
            S { a: x, b: y, c: z } => println!("{:?}, {:?}, {:?}", x, y, z),
        }
    }

    // Because we derived `Eq, PartialEq` for `E` we can also directly compare with `==` and don't
    // have to use the pattern matching syntax with `if let`.
    //
    // FYI: In Rust you don't need parenthesis around the condition. The compiler will even
    // complain if you do this. Want to try it out?
    if e == Some(E::B(10, true)) {
        println!("we're inside the if")
    }

    // The result of the `if` expression is assigned to `e2`. Rust does not have a ternary
    // (`b ? x : y`) operator.
    let e2 = if e.is_none() {
        E::C {
            a: 0,
            b: String::new(),
        }
    } else {
        E::D(S::new(0, String::new()))
    };

    println!("{:?}", e2.into_tuple());
}
