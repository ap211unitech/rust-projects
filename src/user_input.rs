use std::env;

const EXCHANGE_API_URL: &str =
    "https://v6.exchangerate-api.com/v6/515adf0567a55d1970235d89/latest/USD";

#[derive(Debug)]
pub struct UserInput {
    amount: u64,
    from_currency: String,
    to_currency: String,
}

impl UserInput {
    pub fn new(args: &mut env::Args) -> Result<UserInput, &str> {
        args.next();

        if args.len() != 3 {
            return Err("Usage: <amount> <from_currency> <to_currency>");
        }

        let amount = match args.next() {
            Some(x) => match x.parse::<u64>() {
                Ok(amt) => amt,
                _ => return Err("Error is parsing amount.."),
            },
            None => return Err("amount not given.."),
        };

        let from_currency = match args.next() {
            Some(x) => x.to_uppercase(),
            None => return Err("from_currency not given.."),
        };

        let to_currency = match args.next() {
            Some(x) => x.to_uppercase(),
            None => return Err("to_currency not given.."),
        };

        Ok(UserInput {
            amount,
            from_currency,
            to_currency,
        })
    }
}
