use std::fmt::Display;

/// In Rust generics are type parameters for you structs, enums, types, impls, and functions. This is an
/// example for a generic parameter for a struct.
struct OptionalVector<T>(Option<Vec<T>>);

// A `impl` block defines all of its generic parameters directly after the `impl` keyword. This
// generic parameter can then be used througout the impl block, i.e. the type that should be
// implemented.
//
// Try to remove th `<T>` after `impl` and see what happens.
impl<T> OptionalVector<T> {
    fn new(x: T) -> Self {
        Self(Some(vec![x]))
    }
}

// You can also implement your generic struct for a specific type. This is especially useful for
// trait implementations, like the `From` trait. Be careful to not shadow functions in other impl
// blocks though!
impl OptionalVector<usize> {
    fn new_usize(x: usize) -> Self {
        Self(Some(vec![x]))
    }
}

/// You could also write this as a type definition. The difference is that you can't implement
/// functions on types
type OptionalVectorType<T> = Option<Vec<T>>;

// The prove that imple blocks are not possible for type definitions.
// impl<T> OptionalVectorType<T> {}

/// You can also define generic parametes on functions.
fn with_generic_parameter<T>(ovec: OptionalVectorType<T>) -> Vec<T> {
    // This can panic. Can you change this, so it will never panic.
    ovec.unwrap()
}

/// And you can restrict you generic parameters with traits they have to implement.
///
/// You can either restrict them directly where you define them (after the function name) or in the
/// `where` clause after the return type. If you only have few requirements for the generics you
/// should put this after the function name. If they get more complex, you should use the `where`
/// clause.
fn with_restricted_generics<A: Default, B>(a: Option<A>, b: B) -> A
where
    B: Display,
{
    // We guarantee that `B` implements `Display` and can therefore be printed.
    println!("{}", b);
    // We guarantee that `A` implements `Default`, so we can call `unwrap_or_default` on it, which
    // has this same requirement.
    a.unwrap_or_default()
}

/// If you want to store references in your structs, you have to deal with lifetimes. I do my best
/// to explain them, but don't worry if you don't get them right away. When beginning with Rust you
/// want to avoid storing references. This is a bit of a performance hit, but in the beginning it
/// is more important to get stuff to work. Perfomance can be tweaked later.
///
/// # Explanation
///
/// First you'll notice that a lifetime is defined exactly the same as a generic parameter. In fact
/// you will find that it is in practice just another type or better, type extension.
///
/// In this case the lifetime parameter `'a` binds the lifetime of `s` to the struct. When you
/// construct this struct anywhere, the Rust compiler will check, that `s` will live at least as
/// long as the struct. This is important, so that you can't drop `s` before dropping the struct,
/// which ultmately would lead to a SegFault in the best case or undefined behavior in the worst
/// case.
struct WithLifetime<'a> {
    s: &'a str,
}

// Good news everyone: Most of the time you don't have to specify the lifetime of the struct and
// can just use the annonymous lifetime `'_` and the compiler will infer an appropiate lifetime for
// you.
//
// Hint: When dealing with lifetimes, try `'_` first and hope that the compiler will do the right
// thing, usually it works.
impl WithLifetime<'_> {
    fn print_it(&self) {
        println!("{}", self.s);
    }
}

struct S;

// Let's look at lifetimes in functions next. What we haven't talked about is that every reference
// in Rust has a lifetime. Luckily you don't have to specify it most of the time. There are 3 rules
// how automatic lifetime assignment is performed. If your function meets one of those rules, you
// usually won't have to specify lifetimes.
//
// If you have lifetime problems, it often is helpful to remember these three rules and check if
// the automatic lifetime assignment failed.
//
// FYI: This is called "lifetime elision" and is also described here:
// https://doc.rust-lang.org/nomicon/lifetime-elision.html
#[rustfmt::skip]
impl S {
    /// Rule 1: Every arg with a reference type gets its own lifetime.
    fn foo(_s: &str, _t: &str) {}               //   |
//  fn foo<'a, 'b>(_s: &'a str, _t: &'b str) {} // <-- same as

    /// Rule 2: If there is *exactly* one arg with a reference type, the lifetime of this type gets
    /// assigned to all outputs.
    fn bar(_s: &str, _a: usize) -> (&str, &str) { ("", "") }              //   |
//  fn bar<'a>(_s: &'a str, _a: usize) -> (&'a str, &'a str) { ("", "") } // <-- same as

    /// Rule 3: If there is a `&self` argument this lifetime is used for all outputs, regardless of
    /// how many other args have a reference type.
    fn baz(&self, _s: &str) -> &str { "" }                  //   |
//  fn baz<'a, 'b>(&'a self, _s: &'b str) -> &'a str { "" } // <-- same as

    /// Is this legal? What lifetime does the return value have? And what _should_ it get?
    ///
    /// Remember: If two things have the same lifetime, they're tied together. So the return type
    /// should get the lifetime of the argument from which it is derived.
    fn quarx<'a>(s: &'a str, _t: &str) -> &'a str { s }

    /// Let's make it a little bit more complicated. We can have multiple lifetimes in only one
    /// argument. Remember, that we bound the lifetime of `WithLifetime`'s field to the struct? Now
    /// we have to decide, which lifetime our return value should get. Look at what the compiler
    /// tells you to do here. But remember: lifetimes are complicated and the compiler might not
    /// always be 100% correct, because it doesn't know the intention of the programmer.
    ///
    /// (What the compiler suggests might work in this case, but in more complicated cases you
    /// should be careful where lifetimes are coming from)
    fn quarz<'a>(s: &WithLifetime<'a>) -> &'a str {
        s.s
    }
}

fn main() {
    with_restricted_generics(Some(""), "");

    with_generic_parameter(Some(vec![42]));

    OptionalVector::new("");
    OptionalVector::new_usize(10);

    let wl = WithLifetime { s: "" };
    wl.print_it();

    S::foo("", "");
    S::bar("", 0);
    let s = S;
    s.baz("");
    S::quarx("", "");
    S::quarz(&wl);
}
