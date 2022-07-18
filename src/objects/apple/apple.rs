use crate::{objects, objects::cell::Cell};
use sfml::{graphics::*, system::*};

pub struct Apple<'a> {
    pos: Vector2i,
    sprite: RectangleShape<'a>,
}

impl<'a> Apple<'a> {
    pub fn new(pos: Vector2i) -> Self {
        let mut sprite = RectangleShape::new();

        sprite.set_fill_color(Color::RED);
        sprite.set_size(Vector2f::new(10.0, 10.0));
        sprite.set_position(objects::i_to_f(pos) * 10.0);

        Self { pos, sprite }
    }
}

impl Cell for Apple<'_> {
    fn set_pos(&mut self, pos: Vector2i) {
        self.pos = pos;

        self.sprite.set_position(objects::i_to_f(pos) * 10.0);
    }

    fn get_pos(&self) -> Vector2i {
        self.pos
    }

    fn draw(&self, window: &mut RenderWindow) {
        window.draw(&self.sprite);
    }
}
