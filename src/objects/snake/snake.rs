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
    pub fn new(pos: Vector2i) -> Self {
        Self {
            parts: vec![SnakeCell::new(pos)],
            dir: Vector2i::new(0, 1)
        }
    }

    pub fn update(&mut self) {
        self.update_dir();

        let mut last_pos = self.parts[0].get_pos() - self.dir;
        for cell in self.parts.iter_mut() {
            let pos = cell.get_pos();

            cell.set_pos(last_pos);
            last_pos = pos;
        }
    }

    pub fn eat_apple(&mut self) {
        self.parts.push(SnakeCell::new(Vector2i::new(0, 0)));
    }

    pub fn check_head(&self) -> bool {
        let head = self.parts[0].get_pos();
        for cell in self.parts.iter().skip(1) {
            if cell.get_pos() == head {
                return true;
            }
        }

        false
    }

    pub fn check_collision(&self, pos: Vector2i) -> bool {
        for cell in self.parts.iter() {
            if pos == cell.get_pos() {
                return true;
            }
        }

        false
    }

    pub fn head_inside(&self, rect: Vector2u) -> bool {
        let pos = self.parts[0].get_pos();

        (0..rect.x).contains(&(pos.x as u32)) &&
            (0..rect.y).contains(&(pos.y as u32))
    }

    pub fn draw(&self, window: &mut RenderWindow) {
        for cell in self.parts.iter() {
            cell.draw(window);
        }
    }

    fn update_dir(&mut self) {
        let dir =
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

        if dir + self.dir != Vector2i::new(0, 0) {
            self.dir = dir;
        }
    }
}