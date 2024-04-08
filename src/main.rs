use std::{env, process};

use minigrep::Arguments;

fn main() {
    let args = env::args();

    let arguments = Arguments::new(args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1)
    });

    minigrep::run(arguments).unwrap_or_else(|err| {
        println!("Application error: {}", err);
        process::exit(1)
    });
}
