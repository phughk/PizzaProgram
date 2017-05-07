use topping::Topping;
use pizza_base::Base;
use std::fmt;

pub struct Pizza<'a> {
    pub toppings: Vec<Topping<'a>>,
    pub base: Base<'a>,
}

impl<'a> fmt::Display for Pizza<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let total: f32 = self.toppings
            .iter()
            .map(|topping| topping.price)
            .fold(0f32, |left, right| left + right);
        let topping_list = self.toppings
            .iter()
            .map(|topping| topping.name)
            .collect::<Vec<&str>>()
            .join(", ");
        write!(f,
               "Pizza with {} has price {} and is {}vegetarian",
               topping_list,
               total,
               if self.is_vegetarian() { "" } else { "not " })
    }
}

impl<'a> Pizza<'a> {
    pub fn is_vegetarian(&self) -> bool {
        let veg_toppings = self.toppings
            .iter()
            .map(|topping| topping.vegetarian)
            .fold(false, |left, right| left || right);
        return veg_toppings || self.base.vegetarian;
    }
}
