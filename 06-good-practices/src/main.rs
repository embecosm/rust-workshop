//! This is the file producing the binary for the crate.
//!
//! There are two types of crates in Rust. Until now we've only seen the `bin` type. But there is
//! also the `lib` type. The difference is that a `bin` crate produces a binary, the a `lib` crate
//! only a `dll` or object file.
//!
//! If you want to write a binary crate, you ususally split it up into the `lib` part where all the
//! logic is implemented and a `main.rs` which only contains boilerplate for setup, if necessary.
//!
//! Fun fact: The implementation of the `rustc` binary looks exactly like this file.
//!
//! BTW: This is a doc comment for this module.

fn main() {
    // The name of the `lib` part is defined in the `Cargo.toml`. In this case it is
    // `good_practices`. (The binary will be called the same)
    good_practices::run();
}
