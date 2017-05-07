use std::io;
use std::string::String;
use std::io::Write; // Used for flush implicitly
use topping::Topping;

pub fn read_line(stdin: io::Stdin, prompt: &str) -> String {
    print!("{}", prompt);
    let _ = io::stdout().flush();
    let mut result = String::new();
    let _ = stdin.read_line(&mut result);
    return result;
}

pub fn handle_topping<'a>(stdin: io::Stdin) -> Topping<'a>{
    let name = read_line(stdin, "Topping name: ");
    let price = read_line(stdin, "Price: ");
    return Topping {name: &name, price: 0.7, vegetarian: false};
}
