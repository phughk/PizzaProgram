use std::io;
use std::string::String;
use std::io::Write; // Used for flush implicitly?
use std::str;

struct Topping<'a> {
    name: &'a str,
    price: f32
}

struct Pizza<'a> {
//    toppings: Vec<Box<Topping<'a>>>
    toppings: Vec<Topping<'a>>
}

impl <'a> Pizza<'a> {
    pub fn to_string(&self) -> String {
        return format!("Pizza with {} has price {}", self.toppings[0].name, self.toppings[0].price);
    }
}

fn main() {
    let stdin = io::stdin();
    let name  = get_name(stdin);
    println!("Hello {}", name);
    str_topping(get_topping());
    println!("Pizza is {}", get_pizza().to_string());
}

fn get_pizza<'a>() -> Pizza<'a> {
    return Pizza{toppings:vec!(get_topping())}
}

fn get_topping<'a>() -> Topping<'a> {
    return Topping {name: "Salami", price: 0.7}
}

fn str_topping(topping: Topping) {
    println!("{} - {}", topping.name, topping.price)
}

fn get_name(stdin: io::Stdin) -> String {
    print!("Enter name: ");
    io::stdout().flush();
    let mut name = String::new();
    stdin.read_line(&mut name);
    return name;
}
