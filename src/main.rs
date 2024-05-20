mod csv_handler;
fn main() {
    if let Err(e) = csv_handler::read_from_file("./customers.csv") {
        eprintln!("{}", e);
    }
}
