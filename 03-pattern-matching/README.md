# Pattern Matching

After getting to know the basics of Rust, this task will show you the power of
the Rust type system. In particular how to work with algebraic data types.

## Task

As in the first task, try to get the code to compile. After it compiles, use
`cargo clippy` to find more things you can improve.

## Bonus

Pattern matching in Rust is even more powerful than shown in this task. Take a
look at https://doc.rust-lang.org/rust-by-example/flow_control/match.html and
find out what's possible.

For example, destructuring of structs/enums/tuples is not only possible in
`match`es, but also in `let` bindings or in argument position of functions:

```rust
struct Vec2D { a: u32, b: u32 }

fn norm(Vec2D { a, b }: Vec2D) -> f32 {
    ((a * a + b * b) as f32).sqrt()
}
```
