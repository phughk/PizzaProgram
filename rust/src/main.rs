use std::io;
mod helpers;
mod pizza_base;
mod topping;
mod pizza;

fn main() {
    let stdin = io::stdin();
    let name = helpers::read_line(&stdin, "Enter name: ");
    println!("Hello {}", name);
    println!("Pizza is {}", get_pizza());
}

fn get_pizza() -> pizza::Pizza {
    let salami = topping::protein("Salami", 0.8);
    let tomatoe = topping::vegeterian("Tomatoe", 0.1);
    return pizza::Pizza {
        toppings: vec![salami, tomatoe],
        base: pizza_base::Base {
            vegeterian: false,
            name: String::from("Test base"),
            diameter: 12.5f32,
            thickness: 113f32,
        },
    };
}
