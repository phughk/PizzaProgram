use std::str;

pub struct Base<'a> {
    pub name: &'a str,
    pub height: f32,
    pub thickness: f32,
    pub vegetarian: bool,
}
