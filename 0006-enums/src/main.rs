enum IpAddr {
    V4(u8,u8,u8,u8), 
    V6(String)
}

fn test1(){
    let v4 = IpAddr::V4(127,0,0,1);
    let v6 = IpAddr::V6(String::from("::1"));
}

fn test2(){
    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;    
}

enum Coin {
    Penny, Nickel, Dime, Quarter
}
fn value_in_cents(coin:Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25
    }
}
fn test3(){
    let p = Coin::Penny;
    println!("{}", value_in_cents(p));
}

fn plus_one(x: Option<i32>) -> Option<i32>{
    match x {
        None => None,
        Some(i) => Some(i+1),
    }
}
fn test4(){
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}

fn main() {
    println!("Hello, world!");
    test1();
    test2();
    test3();
    test4();
}
