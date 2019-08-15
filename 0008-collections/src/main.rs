
fn test1() {
    let v: Vec<i32> = Vec::new();
    let v2 = vec![1,2,3];
    
    let mut v3 = Vec::new();
    v3.push(5);
    v3.push(6);
    v3.push(7);

    let mut v4 = vec![1,2,3,4,5];
    let third: &i32 = &v4[2];
    let third2: Option<&i32> = v4.get(2);

    // FIXME: book seems to think this should fail 
    let first = &v4[0];
    v4.push(7);

    for i in &v4 {
        println!("{}", i);
    }

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String)
    }
    let row = vec![
        SpreadsheetCell::Int(333),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(3.14159),
    ];
    for r in &row {
        match r {
            SpreadsheetCell::Int(k) => println!("{}", k),
            SpreadsheetCell::Text(k) => println!("{}", k),
            SpreadsheetCell::Float(k) => println!("{}", k),
        }
    }
}
fn test2(){
    let mut s1 = String::new();
    let data = "initial contents";
    let s2 = data.to_string();
    let s3 = "initial contents".to_string();
    let s4 = String::from("initial contents");
    let mut s5 = String::from("foo");
    s5.push_str("bar");
    let s6 = "start";
    s5.push_str(s6);
    println!("s6 is {}", s6);
    s5.push('a'); // Single char only
    s5.push('b');

    let s11 = String::from("Hello ");
    let s12 = String::from("World! ");
    let s3 = s1 + &s2;

    let s21 = String::from("tic");
    let s22 = String::from("tac");
    let s23 = String::from("toe");
    let s24 = format!("{}-{}-{}", s21, s22, s23);
    println!("{}", s24);

    for c in "µ∫√".chars() {
        println!("{}", c);
    }
    for b in "µ∫√".chars() {
        println!("{}", b);
    }
}
fn test3(){
    use std::collections::HashMap;
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let teams = vec![
        String::from("Blue"),
        String::from("Yellow"),
    ];
    let initial_scores = vec![10,50];
    let scores2: HashMap<_,_> = teams.iter().zip(
        initial_scores.iter()
    ).collect();

    let field_name = String::from("Favorite");
    let field_value = String::from("Blue");
    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // println!("{}", field_name);  // <- Borrow failure
    
    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    for (key, value) in &scores {
        println!("{}: {}", key,value);
    }
    // Update the blue team score
    scores.insert(String::from("Blue"), 100);
    for (key, value) in &scores {
        println!("{}: {}", key,value);
    }
    scores.entry(String::from("Yellow")).or_insert(150);
    for (key, value) in &scores {
        println!("{}: {}", key,value);
    }

    let text = "hello world wonderful world";
    let mut m = HashMap::new();
    for word in text.split_whitespace() {
        let count = m.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", m);
}
fn main() {
    println!("Hello, world!");
    test1();
    test2();
    test3();
}
