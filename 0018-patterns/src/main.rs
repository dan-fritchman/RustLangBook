fn print_coords(&(x, y): &(i32, i32)) {
    println!("current location: ({}, {})", x, y);
}

fn main() {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using your favorite color, {}, as the background", color);
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using orange as the background color");
        } else {
            println!("Using blue as the background color");
        }
    }

    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }

    let v = vec!['a', 'b', 'c'];
    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }

    print_coords(&(5, 11));

//    if let Some(x) = some_option {
//        println!("{}", x);
//    }

    let x = 1;
    match x {
        1 => println!("one"),
        2 => println!("two"),
        _ => println!("anything")
    }
    match x {
        1 | 2 => println!("one or two"),
        3...5 => println!("three to five"),
        _ => println!("something else")
    }

    point_stuff();
    ignore_stuff();
}

struct Point {
    x: i32,
    y: i32,
}

fn point_stuff() {
    //! De-structuring
    let p = Point { x: 0, y: 7 };
    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);
    let Point { x, y } = p;
    assert_eq!(0, x);
    assert_eq!(7, y);

    match p {
        Point { x, y: 0 } => println!("On the x-axis at {}", y),
        Point { x: 0, y } => println!("On the y-axis at {}", x),
        Point { x, y } => println!("On neither axis at {}, {}", x, y),
    }

    let points = vec![
        Point { x: 0, y: 0 },
        Point { x: 1, y: 5 },
        Point { x: 10, y: -3 },
    ];
    let sum_of_squares: i32 = points.iter()
        .map(|&Point { x, y }| x * x + y * y)
        .sum();
    println!("Sum of squares: {}", sum_of_squares);

    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });
}

fn foo(_: i32, y: i32) {
    println!("This code only uses y: {}", y);
}

struct Point3 { x: i32, y: i32, z: i32 }

fn ignore_stuff() {
    foo(1, 2);

    let mut setting_value = Some(5);
    let new_setting_value = Some(10);
    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite existing value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }
    println!("setting is {:?}", setting_value);

    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (first, _, third, _, fifth) => {
            println!("Some numbers: {}, {}, {}", first, third, fifth);
        }
    }

    let origin = Point3 { x: 0, y: 0, z: 0 };
    match origin {
        Point3 { x, .. } => println!("x is {}", x),
    }

    let n = (1, 2, 3, 4, 5);
    match n {
        (first, .., last) => {
            println!("Some nums: {}, {}", first, last);
        }
    }

    let robot_name = Some(String::from("Bors"));
    match robot_name {
        Some(ref name) => println!("Found a name: {}", name),
        None => (),
    }
    println!("robot_name is: {:?}", robot_name);

    let mut r2_name = Some(String::from("Bors2"));
    match r2_name {
        Some(ref mut name2) => *name2 = String::from("Another name"),
        None => (),
    }
    println!("r2_name is: {:?}", r2_name);

    let num = Some(4);
    match num {
        Some(x) if x < 5 => println!("less than five: {}", x),
        Some(x) => println!("{}", x),
        None => (),
    }

    let a = Some(5);
    let b = 10;
    match a {
        Some(50) => println!("Got 50"),
        Some(n) if n == b => println!("Matched, n = {:?}", n),
        _ => println!("Default case, a = {:?}", a),
    }
    println!("at the end: a={:?}, b={:?}", a, b);

    enum Message { Hello { id: i32 } }
    let msg = Message::Hello { id: 5 };
    match msg {
        Message::Hello { id: id_var @ 3...7 } => {
            println!("Found and id in range: {}", id_var);
        }
        Message::Hello { id: 10...12 } => {
            println!("Found another in range!");
        }
        Message::Hello { id } => {
            println!("Found some other id: {}", id);
        }
    }
}

