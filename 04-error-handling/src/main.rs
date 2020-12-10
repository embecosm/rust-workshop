// This function returns an `Option`. An `Option` is a type, that is either a value or nothing. Or
// to be more precise, that is either `Some(T)` or `None`.
//
// You can see the `None` part of `Option` as the `null` pointer in Rust. But different from `null`
// pointers in other languages, Rust forces you to deal with this value and makes it impossible to
// ignore it. So no more null pointer exceptions!
fn small_multiplication(a: u8, b: u8) -> Option<u8> {
    if a > 10 || b > 10 {
        return None;
    }

    Some(a * b)
}

// Besides `Option`, another very important type is `Result`. `Result` is just like `Option`, just
// that it allows to pass a value in the `Err` case.
fn checked_division(a: u8, b: u8) -> Result<u8, &'static str> {
    if b == 0 {
        return Err("Division by zero!");
    }

    Ok(a / b)
}

#[allow(unused_variables)]
fn main() {
    let mult_some: Option<u8> = small_multiplication(3, 4);
    let mult_none: Option<u8> = small_multiplication(3, 11);
    let div_ok: Result<u8, &'static str> = checked_division(8, 4);
    let div_err: Result<u8, &'static str> = checked_division(8, 0);

    // Sledgehammer approach
    // You can just unwrap `Option`s or `Result`s. But this should only be done if you are
    // **absolutely** sure that you have the `Some`/`Ok` variant!
    let good: u8 = mult_some.unwrap();
    let good: u8 = div_ok.unwrap();
    // These will panic and your program will abort, so use them cautiously!
    let bad = mult_none.unwrap();
    // Using `expect` is a bit better style, because it makes debugging easier, when your
    // expectation isn't met.
    let bad = div_err.expect("this should've been successful");

    // Safe unwrapping
    // You can unwrap things and simultaniously provide default values. Take a look at the `Option`
    // documentation and see what functions are there.
    let good: u8 = mult_some.unwrap_or(100);
    let good: u8 = mult_some.unwrap_or(100);
    let good: u8 = div_ok.unwrap_or(0);
    let good: u8 = div_err.unwrap_or_default();

    // Really dealing with the value:
    match mult_some {
        Some(val) => println!("result = {}", val),
        None => unreachable!(
            "You shouldn't always panic here. Just doing nothing in this case is often times also correct."
        ),
    }

    // Or you can use `if let` here. This is especially useful if you can just ignore if an `Err`
    // is returned.
    if let Ok(val) = div_ok {
        println!("result = {}", val);
    }

    // And now you: Try to use functions like `map` or `and_then` to transform the results.
    let float_mult: Option<f32> = mult_some.map(|x| x as f32);

    let still_optional_div: Option<f64> = div_err
        .and_then(|x| {
            if x > 10 {
                Ok(x as f64)
            } else {
                Err("too small")
            }
        })
        .ok();

    // The stage is yours. Try out different methods from the `Option` and `Result` documentation
    // and familiarize yourself with them.
    let res = mult_some
        .ok_or("not a small multiplication")
        .map(|x| x * 2)
        .map_err(|err| {
            eprintln!("{}", err);
            err.len()
        })
        .map_or_else(|e| e < 10, |x| x < 10);

    println!("{}", res);
}
