use super::snake_cell::SnakeCell;
use sfml::{
    system::*,
    window::Key,
    graphics::RenderWindow
};
use crate::objects::cell::Cell;

pub struct Snake<'a> {
    parts: Vec<SnakeCell<'a>>,
    dir: Vector2i
}

impl<'a> Snake<'a> {
    pub fn new() -> Self {
        let pos = Vector2i::new(0, 0);

        Self {
            parts: vec![SnakeCell::new(pos)],
            dir: Vector2i::new(0, 1)
        }
    }

    pub fn update(&mut self, delta: f64) {
        self.update_dir();

        for cell in self.parts.iter_mut() {
            let mut pos = cell.get_pos();

            pos -= self.dir;

            cell.set_pos(pos);

            println!("Pos: {} - {}", pos.x, pos.y);
        }
    }

    pub fn eat_apple(&mut self) {

    }

    pub fn draw(&self, window: &mut RenderWindow) {
        for cell in self.parts.iter() {
            cell.draw(window);
        }
    }

    fn update_dir(&mut self) {
        self.dir =
            if Key::W.is_pressed() {
                Vector2i::new(0, 1)
            } else if Key::S.is_pressed() {
                Vector2i::new(0, -1)
            } else if Key::A.is_pressed() {
                Vector2i::new(1, 0)
            } else if Key::D.is_pressed() {
                Vector2i::new(-1, 0)
            } else {
                self.dir
            };

        println!("Dir: {} - {}", self.dir.x, self.dir.y);
    }
}