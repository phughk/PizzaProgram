use std::str;

pub struct Base<'a> {
    name: &'a str,
    height: f32,
    thickness: f32,
    vegetarian: bool
}
