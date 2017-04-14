use specs;
use super::{Point2f, RadF, Vector2f};

#[derive(Clone, Debug)]
pub struct SpatialComponent {
    pub position: Point2f,
    pub angle: RadF,
}

impl specs::Component for SpatialComponent {
    type Storage = specs::VecStorage<SpatialComponent>;
}

#[derive(Clone, Debug)]
pub struct InertialComponent {
    pub velocity: Vector2f,
    pub angular_velocity: RadF,
}

impl specs::Component for InertialComponent {
    type Storage = specs::VecStorage<InertialComponent>;
}
