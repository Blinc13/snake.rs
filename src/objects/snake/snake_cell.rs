use crate::{
    objects,
    objects::cell::Cell
};
use sfml::{
    graphics::*,
    system::*
};

pub struct SnakeCell<'a> {
    pos: Vector2i,
    sprite: RectangleShape<'a>
}

impl<'a> SnakeCell<'a> {
    pub fn new(pos: Vector2i) -> Self {
        let mut sprite = RectangleShape::new();

        sprite.set_fill_color(Color::GREEN);
        sprite.set_size(Vector2f::new(10.0, 10.0));

        Self {
            pos,
            sprite
        }
    }
}

impl Cell for SnakeCell<'_> {
    fn draw(&self, window: &mut RenderWindow) {
        window.draw(&self.sprite);
    }
    fn set_pos(&mut self, mut pos: Vector2i) {
        self.pos = pos;

        self.sprite.set_position(objects::i_to_f(pos)*10.0);
    }
    fn get_pos(&self) -> Vector2i {
       self.pos
    }
}