mod user_input;

use std::{env, process};

use user_input::UserInput;

fn main() {
    /****************  Take User Input ****************/
    let mut args = env::args();

    let user_input = UserInput::new(&mut args).unwrap_or_else(|e| {
        println!("{}", e);
        process::exit(1)
    });

    println!("{:?}", user_input);

    /****************  Validate User Input ****************/

    /****************  Convert currency ****************/
}
