use std::io;
use std::string::String;
use std::io::Write; // Used for flush implicitly?

fn main() {
    let name  = get_name();
    println!("Hello {}", name);
}

fn get_name() -> String {
    print!("Enter name: ");
    io::stdout().flush();
    let stdin = io::stdin();
    let mut name = String::new();
    stdin.read_line(&mut name);
    return name;
}
