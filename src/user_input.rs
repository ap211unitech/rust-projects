use std::env;

#[derive(Debug)]
pub struct UserInput {
    pub amount: f64,
    pub from_currency: String,
    pub to_currency: String,
}

impl UserInput {
    pub fn new(args: &mut env::Args) -> Result<UserInput, &str> {
        args.next();

        if args.len() != 3 {
            return Err("Usage: <amount> <from_currency> <to_currency>");
        }

        let amount = match args.next() {
            Some(x) => match x.parse::<u64>() {
                Ok(amt) => amt as f64,
                _ => return Err("Error is parsing amount.."),
            },
            None => return Err("<amount> not given.."),
        };

        let from_currency = match args.next() {
            Some(x) => x.to_uppercase(),
            None => return Err("<from_currency> not given.."),
        };

        let to_currency = match args.next() {
            Some(x) => x.to_uppercase(),
            None => return Err("<to_currency> not given.."),
        };

        Ok(UserInput {
            amount,
            from_currency,
            to_currency,
        })
    }
}
