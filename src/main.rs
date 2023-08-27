use std::env;
use std::io::{self};

use monkey::repl;

fn main() {
    let user = match env::var("USER") {
        Ok(user) => user,
        Err(_) => String::from("User"),
    };

    println!("Hello {}! This is the Monkey programming language!", user);
    println!("Feel free to type in commands");

    let stdin = io::stdin();
    let stdout = io::stdout();

    repl::start(stdin.lock(), stdout.lock());
}