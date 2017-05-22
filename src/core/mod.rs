use na;

mod object;
mod world;

pub type FloatPrecision = f32;
pub type Pt2 = na::Point2<FloatPrecision>;
pub type Vec2 = na::Vector2<FloatPrecision>;
pub type Iso2 = na::Isometry2<FloatPrecision>;

pub type TimeDelta = f64;

pub use self::object::{ArtificialObjectComponent};
pub use self::world::{SpatialComponent};