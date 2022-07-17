pub use snake::Snake;
pub use apple::Apple;

pub mod cell;
mod snake;
mod apple;

use sfml::system::{Vector2i, Vector2f};

pub(crate) fn i_to_f(i: Vector2i) -> Vector2f {
    Vector2f::new(i.x as f32, i.y as f32)
}