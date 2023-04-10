enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::rc::Rc;

fn main() {

    // Unsure of the lifetimes, so we should use a Rc<T> instead
    // Use RefCount when we don't know the order of the reference lifetimes at compile-time
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    // clone() only increments refcount
    let b = Cons(3, Rc::clone(&a));
    let c = Cons(4, Rc::clone(&a));
}
