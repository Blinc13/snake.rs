use sfml::{
    graphics::*,
    system::*
};
use crate::objects::cell::Cell;

pub struct Apple<'a> {
    pos: Vector2i,
    sprite: RectangleShape<'a>
}

impl<'a> Apple<'a> {
    pub fn new(pos: Vector2i) -> Self {
        let mut sprite = RectangleShape::new();

        sprite.set_fill_color(Color::RED);
        sprite.set_size(Vector2f::new(10.0, 10.0));

        Self {
            pos,
            sprite
        }
    }
}

impl Cell for Apple<'_> {
    fn set_pos(&mut self, pos: Vector2i) {
        self.pos = pos;

        let pos = Vector2f::new(pos.x as f32, pos.y as f32);

        self.sprite.set_position(pos*10);
    }

    fn get_pos(&self) -> Vector2i {
        self.pos
    }

    fn draw(&self, window: &mut RenderWindow) {
        window.draw(&self.sprite);
    }
}