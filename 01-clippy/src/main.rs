// You can specify lint levels with the following attributes:
// - `allow`: suppress the emission of a lint
// - `warn`: Only emit a warning, when the specified lint is triggered
// - `deny`: Emit an error, when the specified lint is triggered
// - `forbid`: rarely used - like `deny`, but trying to `allow` this lint in some places will error.
#![deny(clippy::all)]

fn main() {
    let x = true;

    // This allows the `bool_comparison` lint just for this expression...
    #[allow(clippy::bool_comparison)]
    if x == true {
        println!("Yay");
    }

    // ...but not for following expressions
    if x == false {
        println!("Nay");
    }

    float_comparison(f32::NAN);
    manual_memcpy();
}

const ALMOST_PI: f64 = 3.141562;

fn float_comparison(x: f32) {
    if ALMOST_PI != std::f64::consts::PI {
        println!("So close!");
    }

    if f32::NAN == x {
        println!("We got NAN!");
    }
}

fn manual_memcpy() {
    let src = vec![42; 28];
    let mut dst = vec![0; 28 + 64];

    for i in 0..src.len() {
        dst[i + 64] = src[i];
    }

    println!("{:?}", dst);
}

// And over 400 more lints: https://rust-lang.github.io/rust-clippy/stable/index.html
