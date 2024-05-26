use std::{error::Error, io::Read};

pub fn main() -> Result<(), Box<dyn Error>> {
    let mut res = reqwest::blocking::get("http://httpbin.org/get")?;
    let mut body = String::new();
    res.read_to_string(&mut body)?;

    println!("Status: {}", res.status());
    println!("Headers: {:#?}", res.headers());
    println!("Status: {}", body);
    Ok(())
}
