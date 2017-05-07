use std::io;
use std::string::String;
use std::io::Write; // Used for flush implicitly?
mod helpers;
mod pizza_base;
mod topping;
mod pizza;

fn main() {
    let stdin = io::stdin();
    let name = helpers::get_name(stdin);
    println!("Hello {}", name);
    str_topping(get_topping());
    println!("Pizza is {}", get_pizza());
}

fn get_pizza<'a>() -> pizza::Pizza<'a> {
    return pizza::Pizza {
               toppings: vec![get_topping(), get_topping()],
               base: pizza_base::Base {
                   vegetarian: false,
                   name: "Test base",
                   height: 12.5f32,
                   thickness: 113f32,
               },
           };
}

fn get_topping<'a>() -> topping::Topping<'a> {
    return topping::Topping {
               name: "Salami",
               price: 0.7,
               vegetarian: false,
           };
}

fn str_topping(topping: topping::Topping) {
    println!("{} - {}", topping.name, topping.price)
}
