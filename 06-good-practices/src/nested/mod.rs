//! This module is called `nested`, because the directory is called like that. The main file of
//! this module is `mod.rs`. `mod.rs` is for a module, what `lib.rs` is for a crate

// This won't be available in `lib.rs`
mod nested_one;

// Even though `nested_one` is private, we can still make some items inside of it public. This is
// called "reexporting".
pub use nested_one::quarx;

//This will be
pub mod nested_two;
