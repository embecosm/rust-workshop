#[derive(Default)]
struct VecWrap(Vec<Box<dyn T>>);

impl std::ops::Deref for VecWrap {
    type Target = Vec<Box<dyn T>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for VecWrap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

struct S1;
struct S2;

trait T {
    fn foo(&self) -> u32;
}

impl T for S1 {
    fn foo(&self) -> u32 {
        1
    }
}

impl T for S2 {
    fn foo(&self) -> u32 {
        2
    }
}

fn bar(v: Vec<Box<dyn T>>) -> u32 {
    v.iter().fold(0, |acc, x| acc + x.foo())
}

pub fn run() {
    let mut v = VecWrap::default();
    v.push(Box::new(S1));
    v.push(Box::new(S2));
    assert_eq!(bar(v.0), 3);
}
