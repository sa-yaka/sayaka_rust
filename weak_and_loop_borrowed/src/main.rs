use std::{cell::RefCell, rc::Rc};

fn main() {
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));
    println!("a 的初始化 rc 计数 = {}", Rc::strong_count(&a));
    println!("a 指向的节点 = {:?}", a.tail());

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));
    println!("b 创建之后 a 的 rc 计数 = {}", Rc::strong_count(&a));
    println!("b 初始化 rc 计数 = {}", Rc::strong_count(&b));
    println!("b 指向的节点 = {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }
    println!("在更改 a 后 b 的 rc 计数 = {}", Rc::strong_count(&b));
    println!("在更改 a 后 a 的 rc 计数 = {}", Rc::strong_count(&a));
}

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

use crate::List::*;
impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}
