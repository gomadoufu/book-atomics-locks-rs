use std::rc::Rc;

fn main() {
    let a: Rc<[i32; 3]> = Rc::new([1, 2, 3]);
    let b: Rc<[i32; 3]> = a.clone();

    assert_eq!(a.as_ptr(), b.as_ptr());
}
