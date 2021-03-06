use crate::objects::cell::Cell;
use crate::objects::{Apple, Snake};
use rand::Rng;
use sfml::{
    graphics::{Color, RenderTarget, RenderWindow},
    system::*,
    window::{ContextSettings, Event, Style, VideoMode},
};

pub struct Game<'a> {
    snake: Snake<'a>,
    field_size: Vector2u,
    window: RenderWindow,
    pixel_size: u32
}

impl<'a> Game<'a> {
    pub fn new(width: u32, height: u32, pixel_size: u32) -> Self {
        let mut window = RenderWindow::new(
            VideoMode::new(width * pixel_size,
                           height * pixel_size, 8),
            "Snake",
            Style::default(),
            &ContextSettings::default(),
        );
        window.set_framerate_limit(10);

        Self {
            snake: Snake::new(Vector2i::new((width / 2) as i32, (height / 2) as i32), pixel_size),
            field_size: Vector2u::new(width, height),
            window,
            pixel_size
        }
    }

    pub fn start(mut self) {
        let mut apple = Apple::new(self.get_random_position(), self.pixel_size);

        while self.window.is_open() {
            if let Some(Event::Closed) = self.window.poll_event() {
                self.window.close();
            }

            if self.snake.check_collision(apple.get_pos()) {
                apple.set_pos(self.get_random_position());
                self.snake.eat_apple();
            }
            if self.snake.check_head() || !self.snake.head_inside(self.field_size) {
                self.window.close();
            }

            self.snake.update();

            self.snake.draw(&mut self.window);
            apple.draw(&mut self.window);
            self.window.display();
            self.window.clear(Color::BLACK);
        }
    }

    fn get_random_position(&self) -> Vector2i {
        let mut rand = rand::thread_rng();

        let x = rand.gen_range(0..self.field_size.x);
        let y = rand.gen_range(0..self.field_size.y);

        Vector2i::new(x as i32, y as i32)
    }
}
