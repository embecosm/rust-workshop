fn foo<'a, 'b: 'a>(s1: &'b str) -> &'a str {
    s1
}

pub fn run() {
    assert_eq!(foo("Id"), "Id");
}
