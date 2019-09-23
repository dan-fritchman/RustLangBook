use std::slice;

#[test]
fn t1() {
    println!("Running test1");
}

#[test]
fn f1() {
    let mut num = 5;
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;
    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }

    let mut v = vec![1, 2, 3, 4, 5, 6];
    let r = &mut v[..];
    let (a, b) = r.split_at_mut(3);
    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);
}

extern "C" {
    fn abs(input: i32) -> i32;
}

#[test]
fn f2() {
    unsafe {
        println!("Abs from C: {}", abs(-3));
    }
}

#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Called Rust from C!");
}

static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    unsafe { COUNTER += inc; }
}

#[test]
fn f3() {
    add_to_count(3);
    add_to_count(4);
    unsafe {
        println!("COUNTER: {}", COUNTER);
    }
}

// Traits & Associated Types

struct Counter { c: u32 }

pub trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}

impl Iterator for Counter {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        return Some(3);
    }
}

use std::ops::Add;

#[derive(Debug, PartialEq)]
struct Point { x: i32, y: i32 }

impl Add for Point {
    type Output = Point;
    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

#[test]
fn t2() {
    assert_eq!(Point { x: 1, y: 2 } + Point { x: 5, y: 7 },
               Point { x: 6, y: 9 });
}

// Name Disambiguation

struct Human {}

trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

impl Pilot for Human {
    fn fly(&self) {
        println!("Pilot!");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Wizard!");
    }
}

impl Human {
    fn fly(&self) {
        println!("Human!");
    }
}

#[test]
fn test_flying() {
    let person = Human {};
    person.fly();
    Pilot::fly(&person);
    Wizard::fly(&person);
}

trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("Puppy")
    }
}

#[test]
fn test_animals() {
    assert_eq!(Dog::baby_name(), String::from("Spot"));
    assert_eq!(<Dog as Animal>::baby_name(), String::from("Puppy"));
}

use std::fmt;

trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl OutlinePrint for Point {}

#[test]
fn test_outline_print() {
    let p = Point { x: 1, y: 2 };
    p.outline_print();
}

// newtype pattern

struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

#[test]
fn test_wrapper() {
    let w = Wrapper(vec![String::from("xyz")]);
    println!("{}", w);
}

fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

#[test]
fn test_do_twice() {
    assert_eq!(do_twice(add_one, 1), 4);
}

fn returns_closure() -> Box<Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}

#[test]
fn test_return_closure() {
    let x = returns_closure();
}