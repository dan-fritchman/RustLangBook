struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool
}

fn build_user(email:String, username:String)-> User{
    return User {
        email,
        username,
        active:true,
        sign_in_count:1,
    }
}

fn test1(){
    let mut user1 = User {
        email: String::from("a@b.c"),
        username: String::from("abc"),
        active: true,
        sign_in_count:1,
    };
    user1.email = String::from("x@y.z");
    let user2 = User {
        email: String::from("b@b.c"),
        username: String::from("b"),
        ..user1
    };
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn test2(){
    let black = Color(0,0,0);
    let origin = Point(0,0,0);
}

#[derive(Debug)]
struct Rect{
    width:u32,
    height:u32,
}
impl Rect {
    fn area(&self) -> u32 {
        return self.width * self.height;
    }
    fn can_hold(&self, other: &Rect) -> bool {
        return self.width > other.width &&
            self.height > other.height;
    }
    fn square(size: u32) -> Rect {
        return Rect{width:size, height:size};
    }
}
// struct Circle {
//     radius: u32
// }
// fn area(c:&Circle) -> u32 {
//     return 3.14159 * c.radius * c.radius;
// }
fn test3(){
    let rect1 = Rect{width:30, height:50};
    println!("rect1 is {:#?}", rect1);

    let a = rect1.area();
    println!("{}", a);

    let rect2 = Rect{width:100, height:100};
    let b = rect2.can_hold(&rect1);
    println!("{}", b);

    let s1 = Rect::square(2);
    let b2 = rect2.can_hold(&s1);
    println!("{}", b2);
}
fn main() {
    println!("Hello, world!");
    test1();
    test2();
    test3();
}
