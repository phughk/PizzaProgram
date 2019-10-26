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
    return pizza::Pizza {
        toppings: vec![salami_topping(), tomatoe_topping()],
        base: pizza_base::Base {
            vegeterian: false,
            name: String::from("Test base"),
            diameter: 12.5f32,
            thickness: 113f32,
        },
    };
}

fn salami_topping() -> topping::Topping {
    return topping::Topping {
        name: String::from("Salami"),
        price: 0.7,
        vegeterian: false,
    };
}

fn tomatoe_topping() -> topping::Topping {
    return topping::Topping {
        name: String::from("Tomatoes"),
        price: 0.2,
        vegeterian: true,
    };
}
