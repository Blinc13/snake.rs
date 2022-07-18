use sfml::{graphics::*, system::*};

pub trait Cell {
    fn draw(&self, window: &mut RenderWindow);
    fn set_pos(&mut self, pos: Vector2i);
    fn get_pos(&self) -> Vector2i;
}
