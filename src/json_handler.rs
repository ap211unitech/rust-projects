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

#[derive(Serialize, Deserialize)]
struct Article {
    name: String,
    summary: String,
}

#[derive(Serialize, Deserialize)]
struct Author {
    name: String,
    articles: Vec<Article>,
    is_famous: bool,
}

pub fn write_json() -> Result<(), Box<dyn Error>> {
    let articles = vec![
        Article {
            name: String::from("Learn Rust"),
            summary: String::from("In this video, we will be handling different files with Rust."),
        },
        Article {
            name: String::from("Learn Python"),
            summary: String::from(
                "In this video, we will be covering basics of python programming language.",
            ),
        },
    ];

    let author = Author {
        articles,
        name: String::from("Arjun Porwal"),
        is_famous: true,
    };
    let json = serde_json::to_string(&author).unwrap();
    println!("{}", json);
    Ok(())
}
