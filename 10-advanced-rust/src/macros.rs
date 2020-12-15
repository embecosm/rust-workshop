macro_rules! my_macro {
  ($e:expr, [$($v:ident),*]) => {
    $e;
    let v = vec![$(stringify!($v)),*];
    println!("{:?}", v);
  }
}

#[allow(unused_must_use)]
pub fn run() {
    my_macro!(2 + 3, [a, b, c]);
}
