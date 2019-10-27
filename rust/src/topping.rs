
pub struct Topping {
    pub name: String,
    pub vegeterian: bool,
    pub price: f32,
}

pub fn vegeterian(name: &str, price: f32) -> Topping {
    Topping {
        name: String::from(name),
        price: price,
        vegeterian: true,
    }
}

pub fn protein(name: &str, price: f32) -> Topping {
    Topping {
        name: String::from(name),
        price: price,
        vegeterian: false,
    }
}


#[cfg(test)]
mod tests {
    use topping::*;

    #[test]
    fn creates_vegeterian() {
        let veg = vegeterian("Vegetable", 0.5);
        assert_eq!(veg.vegeterian, true);
    }

    #[test]
    fn create_protein() {
        let prot = protein("Meat", 0.4);
        assert_eq!(prot.vegeterian, false);
    }
}
