use std::{env, error::Error, fs};

pub struct Arguments {
    query: String,
    filename: String,
    case_senstive: bool,
}

impl Arguments {
    pub fn new(mut args: env::Args) -> Result<Arguments, &'static str> {
        args.next();

        let query = match args.next() {
            Some(x) => x,
            None => return Err("Didn't get a query string"),
        };

        let filename = match args.next() {
            Some(x) => x,
            None => return Err("Didn't get a filename"),
        };

        let case_senstive = env::var("CASE_SENSTIVE").is_err();
        Ok(Arguments {
            query,
            filename,
            case_senstive,
        })
    }
}

pub fn run(args: Arguments) -> Result<(), Box<dyn Error>> {
    println!("Query for => {}", args.query);
    println!("In file => {}", args.filename);

    let file_content = fs::read_to_string(args.filename)?;

    let results = if args.case_senstive {
        search(&args.query, &file_content)
    } else {
        search_insenstive(&args.query, &file_content)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_insenstive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query.to_lowercase()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_senstive() {
        let query = "arjun";
        let contents = "Arjunkwwabc\nMy name is\narjun porwal";
        assert_eq!(search(query, contents), vec!["arjun porwal"]);
    }
    #[test]
    fn case_insenstive() {
        let query = "arjun";
        let contents = "Arjunkwwabc\nMy name is\narjun porwal";
        assert_eq!(
            search_insenstive(query, contents),
            vec!["Arjunkwwabc", "arjun porwal"]
        );
    }
}
