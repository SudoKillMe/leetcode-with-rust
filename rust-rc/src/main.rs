use std::rc::Rc;
use std::cell::RefCell;
fn main() {
//    let x = Rc::new(45);
//    let y1 = x.clone();
//    let y2 = x.clone();
//
//    println!("{:?}", Rc::strong_count(&x));
//
//    let w = Rc::downgrade(&x);
//    println!("{:?}", Rc::weak_count(&x));
//
//    let y3 = &*x;
//    println!("{}", 100 - *x);

    let x = RefCell::new(vec![1,2,3]);
    let t = x.borrow();
    println!("{:?}", x.borrow());
    x.borrow_mut().push(5);
    println!("{:?}", x.borrow());
}
