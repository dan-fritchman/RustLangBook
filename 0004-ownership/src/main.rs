fn main() {
    test1();
    test2();
    test3();
    test4();
    test5();
    test6();
}
fn test6(){
    // String Slicing
    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];
    println!("hello: {}", hello);
    println!("world: {}", world);

    let hw = String::from("hello world");
    let fw = first_word(&hw);
    println!("First word: {}", fw);

    let fw2 = first_word(&fw);
    println!("First word: {}", fw2);

    let a = [1,2,3,4,5];
    let slice = &a[1..3];
    println!("First Slice Element: {}", slice[0]);
}
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' { return &s[0..i]; }
    }
    &s[..]
}
fn test5(){
    // let s = dangle(); 
    let s = no_dangle(); 
    println!("s: {}", s);
}
// fn dangle() -> &String {
//     let s = String::from("miami");
//     &s //Fails to compile! Dangling 
// }
fn no_dangle() -> String {
    String::from("beach")
}
fn test4() {
    // Mutable String borrowing & editing 
    let mut s = String::from("Hello");
    edit(&mut s);
    println!("s: {}", s); // Ours
}
fn edit (s: &mut String) {
    s.push_str(" World!");
}
fn test3() {
    let s = String::from("hello!");
    let len = calc_len(&s);
    println!("len: {}", len); 
}
fn calc_len(s: &String) -> usize{
    s.len()
}
fn test2(){
    let s1 = gives_ownership();
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);
    println!("s1: {}", s1); // Ours
    // println!("s2: {}", s2); // Not ours!
    println!("s3: {}", s3); // Ours
}
fn gives_ownership()-> String{
    let s = String::from("world");
    s
}
fn takes_and_gives_back(s:String ) -> String {
    s
}
fn test1(){
    let s = String::from("hello");
    takes_ownership(s);
    // -> s no longer owned by main()
    // println!("s: {}", s);
    let x = 5;
    makes_copy(x);
    println!("x: {}", x);
}
fn takes_ownership(s:String){
    println!("s: {}", s);
}
fn makes_copy(i:i32){
    println!("i: {}", i);
}