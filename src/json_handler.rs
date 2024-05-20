use std::error::Error;

use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Serialize, Deserialize)]
struct User {
    name: String,
}

pub fn read_from_file() -> Result<(), Box<dyn Error>> {
    let json = r#"{"name": "Arjun Porwal"}"#;
    let parsed: User = serde_json::from_str(&json)?;
    println!("{}", parsed.name);
    Ok(())
}
