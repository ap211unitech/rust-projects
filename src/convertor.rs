use crate::{fetcher::ExchangeRates, user_input::UserInput};

pub struct Convertor {
    exchange_rates: ExchangeRates,
}

impl Convertor {
    pub fn new(exchange_rates: ExchangeRates) -> Self {
        Convertor { exchange_rates }
    }

    pub fn convert(&self, user_input: &UserInput) -> String {
        let exchange_rate = self
            .exchange_rates
            .conversion_rates
            .get(&user_input.to_currency)
            .unwrap();

        let converted_value = exchange_rate * user_input.amount;

        format!("{:.2} {}", converted_value, user_input.to_currency)
    }
}
