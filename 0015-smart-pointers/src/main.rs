use std::ops::Deref;
use List::{Cons, Nil};
use std::mem::drop;
use std::rc::Rc;

//use super::run;
mod lib;

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

fn main() {
    println!("Hello, world!");

    lib::run();

    let x = 5;
    let y = &x;
    let z = Box::new(x);
    assert_eq!(x, 5);
    assert_eq!(*y, x);
    assert_eq!(*z, x);

    let b = Box::new(5);
    println!("b = {}", b);

    let list = Cons(1, Rc::new(Cons(2, Rc::new(Cons(3, Rc::new(Nil))))));

    let k = MyBox::new(5);
    assert_eq!(*k, 5);
    let m = MyBox::new(String::from("Rust"));
    hello(&m);

    let c = CustomSmartPointer { data: String::from("test") };
    let c2 = CustomSmartPointer { data: String::from("ing") };
    drop(c2);
    println!("CustomSmartPointer Created");

    rc_stuff();
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

fn hello(name: &str) {
    println!("Hello {}!", name);
}

struct CustomSmartPointer {
    data: String
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`", self.data);
    }
}

enum RcList {
    Cons(i32, Rc<RcList>),
    Nil,
}

fn rc_stuff() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c expires = {}", Rc::strong_count(&a));
}

