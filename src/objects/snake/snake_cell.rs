use crate::{objects, objects::cell::Cell};
use sfml::{graphics::*, system::*};

pub struct SnakeCell<'a> {
    pos: Vector2i,
    sprite: RectangleShape<'a>,
    size: f32
}

impl<'a> SnakeCell<'a> {
    pub fn new(pos: Vector2i, size: u32) -> Self {
        let size = size as f32;
        let mut sprite = RectangleShape::new();

        sprite.set_fill_color(Color::GREEN);
        sprite.set_size(Vector2f::new(size, size));

        Self { pos, sprite, size }
    }
}

impl Cell for SnakeCell<'_> {
    fn draw(&self, window: &mut RenderWindow) {
        window.draw(&self.sprite);
    }
    fn set_pos(&mut self, pos: Vector2i) {
        self.pos = pos;

        self.sprite.set_position(objects::i_to_f(pos) * self.size);
    }
    fn get_pos(&self) -> Vector2i {
        self.pos
    }
}
