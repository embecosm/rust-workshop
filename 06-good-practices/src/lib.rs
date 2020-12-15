//! The `lib.rs` file is the main file of your library. If you want to make
//! functions/modules/structs public and available to other crates, you have to declare them as
//! `pub` in this file

#![warn(missing_docs)]

// This will make the `pub_one` module, defined in `pub_one.rs`, available to crates using this
// library
pub mod pub_one;

// This module won't be available to crates using this library, but can be accessed through
// `crate::private_two::*` from other modules in this crate. See `pub_one.rs`.
mod private_two;

pub mod nested;

// `quarx` was defined in a private submodule of `nested`, but publicly reexported in `nested`, so
// we can `use` it here.
use nested::quarx;

pub use private_two::foo;

/// Careful, this function might panic or return an error.
///
/// We should document this:
///
/// # Panics
///
/// Panics if `a == 0`.
///
/// This section is to document if and when your function can panic
///
/// # Errors
///
/// Errors if `a == 1`.
///
/// This section is to document if and when your function can panic
///
/// # Examples
///
/// ```rust
/// use good_practices::panic_result_example;
///
/// assert!(panic_result_example(1).is_err());
/// assert!(panic_result_example(2).is_ok());
/// ```
///
/// ```rust,should_panic
/// use good_practices::panic_result_example;
///
/// let _ = panic_result_example(0);
/// ```
///
/// This section provides examples for your function. You should include `assert!`s  into the
/// examples, because `cargo test` will automatically check those examples, as if they're unit
/// tests. This prevents your documentation from getting bit rotten.
pub fn panic_result_example(a: usize) -> Result<(), ()> {
    if a == 0 {
        panic!()
    } else if a == 1 {
        Err(())
    } else {
        Ok(())
    }
}

/// `run` runs the code of the library
///
/// BTW: This is a doc comment. These comments will be processed when running `cargo doc` and
/// converted to the documentation style you've seen for example in the `std` documentation. Every
/// public item should have a doc comment. The `#![warn(missing_docs)]` attribute at the top will
/// ensure, that you won't forget it.
pub fn run() {
    // You can either use the public functions in a module by specifying the whole path
    pub_one::bar();

    // or with the `use` statement above, we can import functions and use them directly
    quarx();
}

// This is how to write unit tests in Rust.
//
// The `cfg` attribute tells the compiler that it ignore all of this code, if no tests are run.
#[cfg(test)]
mod test {
    use super::*;

    // A test is marked with the `test` attribute.
    #[test]
    fn panic_result_example_works() {
        assert!(panic_result_example(1).is_err());
        assert!(panic_result_example(2).is_ok());
    }

    #[test]
    #[should_panic]
    fn panic_result_example_panics() {
        let _ = panic_result_example(0);
    }
}
