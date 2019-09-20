extern crate minigrep;

use std::env;
use std::process;

use minigrep::Config;


fn main() {
    // Collect & print argv
//    let args: Vec<String> = env::args().collect();
    // Create a Config
    let config = Config::new(env::args())
        .unwrap_or_else(|err| {
            eprintln!("Problem parsing arguments: {}", err);
            process::exit(1);
        });
    // And run with it
    if let Err(e) = minigrep::run(config) {
        eprintln!("App error: {}", e);
        process::exit(1);
    };
}
