// As you can see, in Rust function definitions don't start with the return type, but with the
// keyword `fn`. The return type is defined at the end of the function header, with the `-> T`
// syntax.
//
// The same goes for arguments: first you have to write the name, then the type: `name: T`.
fn foo(a: bool) -> bool {
    !a
}

fn bar() {
    // In Rust you don't have to specify a type for everything, so most of the time you'll just use
    // `let name = expr;`
    let x = 42;
    // But you can specify a type and Rust will check if you assigned the correct type
    let y: usize = 0usize;
    println!("x = {}; y = {}", x, y);
    // There's also variable shadowing in Rust, where you can redefine the variable with a
    // completely new type
    let y: &str = "Hello, World!";

    println!("y = {}", y);
}

fn quurx() {
    // By default everything is immutable
    let mut x = 42;
    x += 1;

    // To make it mutable, you have to add the keyword `mut`
    let mut y = 10;
    y += 1; // BTW: there are no pre/post-increment operators in Rust

    println!("x = {}; y = {}", x, y);
}

fn baz() {
    // Type inference doesn't always work though. A prominent example is the `collect` method on
    // iterators.
    let mut x: Vec<_> = std::iter::empty().collect();
    x.push(42);
    // Let me introduce the turbofish to you: `::<>`
    let mut y = std::iter::empty().collect::<Vec<_>>();
    y.push(42);
}

fn quarx(s: &str) {
    println!("{}", s);
}

fn quarz(s: &mut String) {
    s.push_str(" WORLD!!!");
}

// You might notice, that we now pass a `&str` instead of a `String`. The main difference between
// the two are that `&str` lives on the stack and `String` lives on the heap.
//
// Note: If you want to pass a reference to a `String`, so `&String`, you usually want to have the
// type `&str` instead of `&String`. We'll get to the "Why?" later. (Clippy will tell you about
// this)
fn macro_mania(a: u32, b: bool, s: &str) {
    // With the `format!` macro it is possible to format your types to `String`s.
    let x = format!("a = {}", a);
    println!("{}", x);

    // `println!` use `format!` internally, so you don't need to `format!` things before printing
    // them.
    println!("b = {}", b);

    // Rust brings `printf`-debugging to the next level with the `dbg!` macro. This macro will
    // print the name, location and value of the passed variables.
    dbg!(a, b, s);

    // Rust also provides macros to abort your program:
    panic!("PANIC: {}", s);
}

fn main() {
    foo(true);
    bar();
    baz();
    quurx();

    // `String` is a owned type. This means that there is exactly one owner at all times. Here
    // we're passing it to `quarx` by value. In Rust we call this "moving the value" or "transfer
    // ownership" to `quarx`.
    //
    // Try to fix this, by passing `s` to `quarx` by reference. Or in Rust speak: make `quarx`
    // borrow `s`.
    let mut s = String::from("I love Rust already!");
    quarx(&s);
    println!("{}", s);

    // When borrowing `s` as mutable, you won't be able to borrow it immutable. Try to fix this
    // somehow. You have to make sure, that no shared references to `s` exist anymore, when you
    // mutably borrow `s`.
    macro_mania(42, false, &s);
    let x = &mut s;
    quarz(x);
    println!("{}", s);

}
