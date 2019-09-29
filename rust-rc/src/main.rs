use std::rc::Rc;
use std::cell::RefCell;
fn main() {
    let x = Rc::new(3);
    let y = Rc::clone(&x);

    println!("{:?}", Rc::try_unwrap(x));
    println!("{:?}", Rc::try_unwrap(y));
}
