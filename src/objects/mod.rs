pub use apple::Apple;
pub use snake::Snake;

mod apple;
pub mod cell;
mod snake;

use sfml::system::{Vector2f, Vector2i};

pub(crate) fn i_to_f(i: Vector2i) -> Vector2f {
    Vector2f::new(i.x as f32, i.y as f32)
}
