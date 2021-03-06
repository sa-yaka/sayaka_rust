use crate::List::*;
use std::rc::Rc;

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));

    let b = Cons(3, Rc::clone(&a));
    let c = Cons(4, Rc::clone(&a));
    println!("Rc count {:?}, {:?}", Rc::strong_count(&a), a);
}
#[derive(Debug)]
enum List {
    Cons(i32, Rc<List>),
    Nil,
}
