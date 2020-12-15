use std::rc::Rc;

unsafe fn deref(x: *const u32) -> u32 {
    *x
}

static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

struct S(Rc<u32>);

unsafe impl Send for S {}
unsafe impl Sync for S {}

union MyUnion {
    f1: u32,
    f2: f32,
}

pub fn run() {
    let x = 42u32;
    let ptr_x = &x as *const u32;
    assert_eq!(unsafe { deref(ptr_x) }, 42);

    add_to_count(3);
    assert_eq!(unsafe { COUNTER }, 3);

    let u = MyUnion { f1: 1 };
    let f = unsafe { u.f2 };
    println!("{}", f);
}
