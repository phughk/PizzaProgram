use std::io;
use std::string::String;
use std::io::Write; // Used for flush implicitly?

pub fn get_name(stdin: io::Stdin) -> String {
    print!("Enter name: ");
    io::stdout().flush();
    let mut name = String::new();
    stdin.read_line(&mut name);
    return name;
}
