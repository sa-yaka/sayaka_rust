use std::{fmt::Display, ops::Deref};

fn main() {
    let name = MyBox::new("Rust".to_string());
    hello(&name);
    hello("Rust");
}

struct MyBox<T>(T);

impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

fn hello(name: &str) {
    println!("Hello {}", name);
}
