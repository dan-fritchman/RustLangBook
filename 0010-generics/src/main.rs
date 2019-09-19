
use std::fmt::Display;


fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    return largest;
}

struct Point<T,U> {
    x: T,
    y: U
}
impl<T,U> Point<T,U> {
    fn mixup<V,W> (self, other:Point<V,W>) -> Point<T,W> {
        return Point {x:self.x, y:other.y};
    }
}

fn points () {
    let pi = Point {x:5, y:7};
    let pf = Point {x:1.0, y:4.0};
}

pub trait Summary {
    fn summarize(&self) -> String {
        return String::from("(Read more...)")
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String
}

impl Summary for NewsArticle {
    // fn summarize(&self) -> String {
    //     return format!("{}, by {} ({})", self.headline, self.author, self.location);
    // }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        return format!("{}: {}", self.username, self.content);
    }
}

pub fn notify<T: Summary>(item:T) {
    println!("Breaking news! {}", item.summarize());
}

struct Pair<T>{
    x:T, y:T
}

impl<T> Pair<T>{
    fn new(x:T,y:T) -> Self {
        return Self {x,y}
    }
}

impl<T:Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { return x; }
    else { return y; }
}

fn longest_with_announcement<'a, T>(x: &'a str, y: &'a str, ann:T) -> &'a str 
    where T:Display {
        println!("Announcement! {}", ann);
        if x.len() > y.len() { return x; }
        else { return y; }
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("Result of call to `largest`: {}", result);

    points();
    let p = Point{x:1,y:2};
    println!("p.x = {}", p.x);

    let p1 = Point{x:5, y:10.4};
    let p2 = Point{x:"hello", y:'c'};
    let p3 = p1.mixup(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("????"),
        reply: false, 
        retweet: false
    };
    println!("1 new tweet: {}", tweet.summarize());
    let article = NewsArticle {
        headline: String::from("hdr"),
        location: String::from("loc"),
        author: String::from("joe"),
        content: String::from("cnt")
    };
    println!("New article: {}", article.summarize());
    notify(article);
    notify(tweet);

    let string1 = String::from("abcd");
    let string2 = "xyz";
    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);

    let str1 = String::from("long string is long");
    {
        let str2 = String::from("xyz");
        let rs = longest(str1.as_str(), str2.as_str());
        println!("The longest str is {}", rs);
    }

}
