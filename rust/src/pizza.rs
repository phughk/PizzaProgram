use topping::Topping;
use pizza_base::Base;

pub struct Pizza<'a> {
    pub toppings: Vec<Topping<'a>>,
    pub base: Base<'a>
}
