mod csv_handler;
mod json_handler;

fn main() {
    // CSV File Read
    if let Err(e) = csv_handler::read_from_file("./customers.csv") {
        eprintln!("{}", e);
    }

    // JSON File Read
    if let Err(e) = json_handler::read_from_file() {
        eprintln!("{}", e);
    }
}
