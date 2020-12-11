#![warn(clippy::filter_map)]

// Let's start with closures, a powerful expression type in Rust.
fn closure(a: u32, s: String) {
    // You can define closures with the syntax `|<args>| { <code> }`.
    //
    // You can look at closures as unnamed inner functions, that have access to the variables
    // around it.
    let check_string_shorter = |l: usize| -> bool {
        // But be careful, if you transfer ownership of a outer variable inside a closure, it will
        // no longer be available in the code afterwards. It doesn't matter where you use the
        // closure, but where you define it.
        //
        // You can say that "the closure takes ownership of the outer variable". Remove the
        // following line to fix this function.
        s.len() < l
    };

    let check_string_longer = |l| s.len() > l;

    if a < 10 {
        check_string_shorter(10);
    } else {
        check_string_longer(10);
    }
}

// You can also take a closure as a argument to a function. Those arguments are usually defined
// with a generic parameter. We'll look at generics later. For now, you have to know that the
// `where` clause means, that `F` is some function, that takes something of type `usize` and
// returns a bool. One example for this is the `for_closure_fn`.
fn take_closure<F>(f: F) -> bool
where
    F: Fn(usize) -> bool,
{
    f(10)
}

// This can be passed to `take_closure`
fn for_closure_fn(a: usize) -> bool {
    a == 0
}

fn call_take_closure() {
    // You can either pass the name of a fitting function...
    println!("{}", take_closure(for_closure_fn));

    // ...or a closure. Most of the time you'll pass a closure.
    println!("{}", take_closure(|a| a > 0));
}

// fn take(s: String) {
//     println!("{}", s);
// }

fn for_loops(vec: Vec<usize>) {
    // In Rust classic for-loops with a counter are discuraged. You will find that you almost never
    // need such a loop. Instead you have iterators.
    //
    // (You'll begin to wonder why iterators are not the default in other languages)
    for elem in vec.iter() {
        println!("{}", elem);
    }

    // Because Rust is a convenient language you can also leave out the `.iter()` call.
    for elem in &vec {
        println!("{}", elem);
    }

    // But be careful to not accidentally move the vector into the loop. As always: ask the
    // compiler what to do instead. We're not dealing with lifetimes yet, so it should still be
    // your friend.
    println!("{:?}", vec);
}

// Now we start looking at common iterator functions, that you'll use every day with Rust. It
// starts slow here. In `iterators_on_steroids` you'll see the full power of iterators.
fn iterators(vec: Vec<usize>) {
    // The `into_iter` method will move the underlying structure into the iterator. This is
    // equivalent to `for elem in vec`. The difference to `.iter()` is that `iter()` gives you
    // references to the elements in the vector, while `into_iter()` gives you the pure elements.
    //
    // But see yourself. Change `into_iter()` to `iter()` and compare the compiler output by
    // uncommenting the `let` statement in the loop.
    for elem in vec.iter() {
        // This is a hack to let the compiler show you the type of `elem`. `printf`-debugging
        // during compile time.
        // let _: () = elem;
        println!("{}", elem);
    }

    // If you need the element and also an index to the element, you can use the `enumerate`
    // method:
    for (i, elem) in vec.iter().enumerate() {
        println!("vec[{}] = {}", i, elem)
    }

    // Before running the for loop you may want to modify the iterator, by for example filtering
    // out elements. See `iterators_on_steroids` for more useful methods of iterators.
    for elem in vec.iter().filter(|x| *x % 2 == 0) {
        println!("Only even: {}", elem);
    }
}

// You'll find that you often don't even need a for loop, but can do things purely with iterators
// and its methods.
fn iterators_on_steroids(vec: Vec<usize>) {
    let greater_three_except_first = vec
        // First get an iterator
        .iter()
        // Next only find the elements that are at least 3
        .filter(|x| *x >= &3)
        // Skip the fist element, because for some reason we don't care about this
        .skip(1)
        // Convert/Map the element to a `String`
        .map(|x| x.to_string())
        // Collect all of the elements in a vector
        .collect::<Vec<_>>();

    println!("{:?}", greater_three_except_first);

    // And now you. Go look at the `Iterator` documentation and and try to get the assert_eq below
    // to pass.
    let first_two_even_floats = vec
        .iter()
        .filter_map(|x| if *x % 2 == 0 { Some(*x as f32) } else { None })
        .take(2)
        .collect::<Vec<_>>();

    assert_eq!(first_two_even_floats, vec![2f32, 4f32]);
}

fn main() {
    closure(10, String::new());
    call_take_closure();
    for_loops(vec![1, 2, 3, 4, 5]);
    iterators(vec![1, 2, 3, 4, 5]);
    iterators_on_steroids(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
}
