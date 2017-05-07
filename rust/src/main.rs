use std::io;
use std::string::String;
use std::io::Write; // Used for flush implicitly?
mod helpers;
mod pizza_base;
mod topping;

struct Pizza<'a> {
    toppings: Vec<topping::Topping<'a>>,
    base: pizza_base::Base<'a>
}

impl<'a> Pizza<'a> {
    pub fn to_string(&self) -> String {
        let total: f32 = self.toppings
            .iter()
            .map(|topping| topping.price)
            .fold(0f32, |left, right| { left + right });
        let topping_list = self.toppings
            .iter()
            .map(|topping| topping.name)
            .collect::<Vec<&str>>()
            .join(", ");
        return format!("Pizza with {} has price {} and is {}vegetarian", topping_list, total, if self.is_vegetarian() {""} else {"not "});
    }

    pub fn is_vegetarian(&self) -> bool {
        let veg_toppings = self.toppings
            .iter()
            .map(|topping| topping.vegetarian)
            .fold(false, |left, right| {left || right});
        return veg_toppings || self.base.vegetarian;
    }
}

fn main() {
    let stdin = io::stdin();
    let name = helpers::get_name(stdin);
    println!("Hello {}", name);
    str_topping(get_topping());
    println!("Pizza is {}", get_pizza().to_string());
}

fn get_pizza<'a>() -> Pizza<'a> {
    return Pizza { toppings: vec!(get_topping(), get_topping()), base: pizza_base::Base{ vegetarian: false, name: "Test base", height: 12.5f32, thickness: 113f32}};
}

fn get_topping<'a>() -> topping::Topping<'a> {
    return topping::Topping { name: "Salami", price: 0.7, vegetarian: false}
}

fn str_topping(topping: topping::Topping) {
    println!("{} - {}", topping.name, topping.price)
}
