mod convertor;
mod fetcher;
mod user_input;

use std::{env, process};

use convertor::Convertor;
use fetcher::get_exchange_rates;
use user_input::UserInput;

#[tokio::main]
async fn main() {
    /****************  Take User Input ****************/
    let mut args = env::args();

    let user_input = UserInput::new(&mut args).unwrap_or_else(|e| {
        println!("{}", e);
        process::exit(1)
    });

    println!("{:?}", user_input);

    /**************** Fetch exchange rates ****************/
    let exchange_rates = get_exchange_rates(&user_input.from_currency, &user_input.to_currency)
        .await
        .unwrap_or_else(|e| {
            println!("{}", e);
            process::exit(1)
        });

    /**************** Convert the currency ****************/
    let convertor = Convertor::new(exchange_rates);
    let response = convertor.convert(&user_input);
    println!("Converted amount => {}", response);
}
