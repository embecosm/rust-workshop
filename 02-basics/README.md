# Basics

This project shows you the basic things in Rust. The code in here currently does
not compile. Your task is to make it compile.

## Compiling Rust

To compile a Rust crate, you want to use `cargo`. `cargo` is the package manager
and build tool of Rust. It will automatically download and compile all the
dependencies in `Cargo.toml` and check and compile your crate.

To run `cargo` you have different options. The commands relevant for this are:

- `cargo check`: Only verifies your code, does not produce a binary (fast)
- `cargo build`: Compiles your code and produces a binary
- `cargo run`: Compiles your code, produces a binary and executes the binary

During development you want to use `cargo check`, because it won't invoke the
compiler backend (LLVM).

If `cargo check` passes (without warnings!), you want to run `cargo build` or
`cargo run` to build (and run) the binary.

## Bonus

After `cargo check` passes, you might also want to run `cargo clippy`. This will
give you some more warnings regarding idiomatic code style, likely bugs or
possible performance problems.

_Note:_ You may have to run `cargo clean` before `cargo clippy` for Clippy to
produce output. A fix for this bug is in the pipeline.
