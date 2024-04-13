use reqwest;
use serde::Deserialize;
use std::collections::HashMap;
use std::error::Error;
use std::io;

#[derive(Debug, Deserialize)]
pub struct ExchangeRates {
    pub conversion_rates: HashMap<String, f64>,
}

const EXCHANGE_BASE_URL: &str =
    "https://v6.exchangerate-api.com/v6/515adf0567a55d1970235d89/latest/";

pub async fn get_exchange_rates(
    from_currency: &str,
    to_currency: &str,
) -> Result<ExchangeRates, Box<dyn Error>> {
    let client = reqwest::Client::new();

    let exchange_api_url = format!("{}{}", EXCHANGE_BASE_URL, from_currency);

    let response = client.get(exchange_api_url).send().await?;
    let exchange_rates: ExchangeRates = response.json().await?;

    match exchange_rates.conversion_rates.get(to_currency) {
        Some(_) => Ok(exchange_rates),
        None => {
            let err_msg = format!(
                "Exchange rate from {} to {} not found.",
                from_currency, to_currency
            );
            return Err(Box::new(io::Error::new(io::ErrorKind::Other, err_msg)));
        }
    }
}
