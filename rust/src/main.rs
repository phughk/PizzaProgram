use std::io;
mod helpers;
mod pizza_base;
mod topping;
mod pizza;

fn main() {
    let stdin = io::stdin();
    let name = helpers::read_line(&stdin, "Enter name: ");
    println!("Hello {}", name);

    //let bases = vec!(red, white); // We can't own in a vector and then pass Box owenership down
    
    let picked_base = pick_base(); // bases[0];

    let picked_toppings = pick_toppings();

    let pizza = pizza::Pizza {
        toppings: picked_toppings,
        base: picked_base,
    };

    println!("Pizza is {}", pizza);
}

fn pick_base() -> Box<pizza_base::Base> {
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
    let mut choices = vec!(red, white);
    return Box::new(choices.remove(0));
}


fn pick_toppings() -> Vec<Box<topping::Topping>> {
    let salami = topping::protein("Salami", 0.8);
    let tomatoe = topping::vegeterian("Tomatoe", 0.1);
    let mut toppings = vec![salami, tomatoe];

    return vec![Box::new(toppings.remove(0))];
}
