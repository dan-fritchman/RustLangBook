use std::fs::File;
use std::io;
use std::io::ErrorKind;
use std::io::Read;

fn read_some_file() -> Result<String, io::Error> {
    let mut f4 = File::open("hello.txt")?;
    println!("f4");
    return Ok(String::new());
}
fn main() {
    println!("Hello, world!");

    let f = File::open("hello.txt");
    let f = match f {
        Ok(file) => file,
        Err(ref err) if err.kind() == ErrorKind::NotFound => {
            match File::create("hello.txt"){
                Ok(fc) => fc,
                Err(e) => panic!(e)
            }
        },
        Err(error) => panic!(error)
    };
    let mut f2 = File::open("hello.txt").unwrap();
    let mut f3 = File::open("hello.txt")
        .expect("Error loading hello.txt");

    let mut s = String::new();
    f3.read_to_string(&mut s); // ?; // Question operator, not usable here
    let mut f4 = read_some_file(); // Usable in here 


}
