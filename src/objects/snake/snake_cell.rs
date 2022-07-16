use crate::objects::cell::Cell;
use sfml::{
    graphics::*,
    system::*
};
use sfml::system::Vector2i;

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

        pos*=10;
        let pos = Vector2f::new(pos.x as f32, pos.y as f32);

        self.sprite.set_position(pos);
    }
    fn get_pos(&self) -> Vector2i {
       self.pos
    }
}