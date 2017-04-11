use cgmath::{Point2, Rad, Vector2};

mod object;
mod world;

pub type FloatPrecision = f32;
pub type Point2f = Point2<FloatPrecision>;
pub type Vector2f = Vector2<FloatPrecision>;
pub type RadF = Rad<FloatPrecision>;

pub use self::object::{DrawCircleComponent, DrawRectangleComponent};
pub use self::world::{SpatialComponent, InertialComponent};