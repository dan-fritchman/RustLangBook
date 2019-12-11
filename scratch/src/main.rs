use libc::{c_char, c_int, gid_t, system};
use std::ffi::{OsStr,CString};


fn stuff(i: Option<i64>) -> i64 {
    if let Some(i) = i {
        println!("Got a number: {}", i);
        return i;
    }
    println!("Got an error");
    return -1;
}

fn matcher(x: i32) -> i32 {
    match x {
        x if x > 100 => -1,
        _ => 1
    }
}

fn main() {
    let i = stuff(Some(11));
    println!("{}", i);
    let k = stuff(None);
    println!("{}", k);

    println!("{}", matcher(101));
    println!("{}", matcher(99));

//    let output = Command::new("ls -alh").spawn();
    let arg = CString::new("ls").unwrap();
    let x = unsafe { system(arg.as_ptr()) };
}

