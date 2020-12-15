fn foo<F>(f: F) -> i32
where
    F: for<'a> Fn(&'a i32, &i32) -> &'a i32,
{
    let (x, y) = (0, 1);
    *f(&x, &y)
}

pub fn run() {
    assert_eq!(foo(|x, _| x), 0);
}
