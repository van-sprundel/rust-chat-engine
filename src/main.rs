mod client;
mod server;

fn main() {
    let mut args = std::env::args();
    match (args.nth(1).as_ref().map(String::as_str), args.next()) {
        (Some("client"), None) => client::main(),
        (Some("server"), None) => server::main(),
        _ => println!("Usage: cargo run [client|server]"),
    }
}