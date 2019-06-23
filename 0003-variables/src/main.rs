
const A_CONST: u32 = 5;
fn sub(){
    let x = A_CONST;
    let x = x + 1;
    let x = x * 2;
    println!("Value of x: {}", x);
}
fn sub2() {
    let mut x = 5;
    println!("Value of x: {}", x);
    x = 6;
    println!("Value of x: {}", x);
}
fn sub3(){
    let _a: u32 = 0xFF;
    let _b: i32 = -11;
    let _c: isize = 2^62;

    // let sum = _a + _b;

    let _t = true;
    let _f: bool = false;

    let _c1 = 'c';
    let _c2: char = 'd';

    // Built-in collections: tuple and array 

    // Tuples 
    let _t1 = (500, 501, 502);
    let _t2 = ('a', 2, 3.0);
    let (_t, _tt, _ttt) = _t2;
    println!("First tuple element: {}", _t);
    println!("Second tuple element: {}", _t2.1);

    // Fixed size arrays, ala C 
    let _arr = [1,2,3,4,5];
    let months = ["Jan", "Feb", "Mar"];
    println!("First month: {}", months[0]);
}
fn sub4(x:i32, y:i32) {
    println!("Value of x: {}", x);
    // An expression assignment 
    let z = {
        let a = 2;
        // Note no `return`, and no semicolon! 
        x + y + a
    };
    println!("Value of z: {}", z);
}
fn five() -> i32 {
    // Last expression is the return value 
    5
    // Same as return 5;
}
fn control(x:i32){
    if x < 11 {
        println!("True!");
    } else {
        println!("False!");
    }
}
fn control_expr(x:i32){
    let y = if x==11 { 
        3
    } else {
        4
    };
    println!("Value of y: {}", y);
}
fn loops(){
    let mut x = 3;
    while x > 0 {
        println!("Value of x: {}", x);
        x -= 1;
    }
    let arr = [1,2,3,4,5];
    for e in arr.iter() {
        println!("Value of e: {}", e);
    }
}
fn main() {
    sub();
    sub2();
    sub3();
    sub4(11, 12);
    let f = five();
    println!("Value of f: {}", f);
    control(10);
    control(12);
    control_expr(11);
    control_expr(12);
    loops();
}
