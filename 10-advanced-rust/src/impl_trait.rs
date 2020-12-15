trait AnimalBehavior {
    fn annoy(&mut self, _: &mut impl AnimalBehavior) {}
}

struct Cat;
struct Dog;

impl AnimalBehavior for Cat {}
impl AnimalBehavior for Dog {}

fn animal_things(a1: &mut impl AnimalBehavior, a2: &mut impl AnimalBehavior) {
    a1.annoy(a2);
}

fn evens(v: &[u32]) -> impl Iterator<Item = &u32> {
    v.iter().filter(|&x| x % 2 == 0)
}

pub fn run() {
    let mut cat = Cat;
    let mut dog = Dog;
    animal_things(&mut cat, &mut dog);

    let v = vec![1, 2, 3];
    assert_eq!(evens(&v).collect::<Vec<_>>(), vec![&2]);
}
