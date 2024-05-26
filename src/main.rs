mod blocking_request;

fn main() {
    if let Err(e) = blocking_request::main() {
        eprintln!("{}", e);
    }
}
