# Build your own Zoo

As your first Rust project, you should build your own Zoo. Some things are
already prepared in this crate, you should fill the gaps.


## Objectives

- Your animals shouldn't eat each other. Exotic animals are expensive and dogs
  are cute. Cats exist (see [below](#Bonus)).
- Use as few cages as possible.


## Rules

1. Every animal has these attributes:
    - `name` (string)
    - `species` (string)
    - `strength` (int)
    - `carnivore` (bool)
2. Animals with the same species go into the same cage
3. Carnivores can't be in the same cage as another carnivore
4. Animal `A` eats animal `B` if:
    - `A` is a carnivore
    - `A` is stronger than `B` (`A::strength > B::strength`)
    - `A` and `B` have different species
5. If two carnivores of different species are equally strong they won't eat each
   other
6. If a herbivore can go in multiple cages, it will be put in the cage with the
   strongest `carnivore` (for intimidation)


## Task

There are multiple functions and methods in the files

- [`src/lib.rs`](src/lib.rs)
- [`src/animal.rs`](src/animal.rs)
- [`src/cage.rs`](src/cage.rs)
- [`src/error.rs`](src/error.rs)

that currently are only stubs. Use what you learned until now combined with the
[`std` documentation](https://doc.rust-lang.org/std/) to fill out those methods.

If you filled out every method, run `cargo test` to see if they implement the
intended behavior.

If the tests are passing, you can run your program with

```bash
# The whitespace between `--` is intended
cargo run -- animals.json
```

_NOTE:_ If you see a comment `Nothing to change here.` you should not have to
change that part of the program (`struct`, `fn`, `enum`, ...) for the program to
work.


## Bonus

Add a flag `--food SPECIES`, that will put all animals of the specified
`SPECIES` in the same cage as the strongest carnivore.

Your program should error if:

- There's an animal of `SPECIES` that is at least as strong as the strongest
  carnivore
- There is no animal of `SPECIES` in the JSON file

So for example `--food Cat` should put all cats in the same cage as the T-Rex.
But `--food T-Rex` should emit an error, because T-Rex is the strongest
carnivore, with a strength of over 9000!
