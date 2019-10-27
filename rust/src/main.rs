use std::io;
mod helpers;
mod pizza_base;
mod topping;
mod pizza;

fn main() {
    let stdin = io::stdin();
    let name = helpers::read_line(&stdin, "Enter name: ");
    println!("Hello {}", name);
    let bases = all_bases();
    let picked_base = pick_base(&bases);

    let toppings = all_toppings();
    let picked_toppings = pick_toppings(&toppings);

    let pizza = pizza::Pizza {
        toppings: picked_toppings,
        base: &picked_base,
    };

    println!("Pizza is {}", pizza);
}

fn pick_base(bases: &[pizza_base::Base]) -> &pizza_base::Base {
    return &bases[0];
}

fn pick_toppings(toppings: &[topping::Topping]) -> Vec<&topping::Topping> {
    return vec![&toppings[0]];
}

fn all_bases() -> Vec<pizza_base::Base> {
    let red = pizza_base::Base {
        vegeterian: true,
        name: String::from("Normal Tomatoe Base"),
        diameter: 12.0f32,
        thickness: 0.3f32,
    };

    let white = pizza_base::Base {
        vegeterian: false,
        name: String::from("Thin Bianca Base"),
        diameter: 12.0f32,
        thickness: 0.1f32,
    };
    return vec![red, white];
}

fn all_toppings() -> Vec<topping::Topping> {
    let salami = topping::protein("Salami", 0.8);
    let tomatoe = topping::vegeterian("Tomatoe", 0.1);
    return vec![salami, tomatoe];
}
