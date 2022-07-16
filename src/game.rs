use crate::objects::cell::Cell;
use sfml::system::Vector2u;

pub struct Game {
    cells: Vec<Box<dyn Cell>>,
    field_size: Vector2u
}

impl Game {
    fn new(width: u32, height: u32) -> Self {
        Self {
            cells: vec![],
            field_size: Vector2u::new(width, height)
        }
    }
}