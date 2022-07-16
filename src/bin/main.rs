use snake::objects::snake::snake::Snake;
use sfml::graphics::{Color, RenderTarget, RenderWindow};
use sfml::window::{
    VideoMode,
    Style,
    ContextSettings
};

fn main() {
    let mut window = RenderWindow::new(
        VideoMode::new(300, 300, 8),
        "Snake",
        Style::default(),
        &ContextSettings::default()
    );

    window.set_framerate_limit(10);

    let mut snake = Snake::new();

    while window.is_open() {
        snake.update(1.0);

        snake.draw(&mut window);
        window.display();
        window.clear(Color::BLACK);
    }
}
