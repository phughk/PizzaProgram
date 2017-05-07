use std::io;
use std::string::String;
use std::io::Write; // Used for flush implicitly?

pub fn get_name(stdin: io::Stdin) -> String {
    print!("Enter name: ");
    let _ = io::stdout().flush();
    let mut name = String::new();
    let _ = stdin.read_line(&mut name);
    return name;
}
