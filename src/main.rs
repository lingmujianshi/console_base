use std::env;
use std::process;

use console_base::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    let config = Config::new(&args).unwrap_or_else(|_err| {
        process::exit(1);
    });

    if let Err(_e) = console_base::run(config) {
        process::exit(1);
    }
}
