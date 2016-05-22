pub mod properties;

use solar_rustlib::core::object;
use render::DrawFunction;
use self::properties::ObjectRegister;

pub type UpdateFunction = Box<Fn(&mut GameObject, f64)>;

/// An independent 'Object' evolving inside the game simulation.
pub struct GameObject {
    pub object_type: object::ObjectType,
    pub visuals: object::ObjectVisuals,
    pub orbit: object::Orbit,
    pub time_alive: f64,
    pub position: (f64, f64),
    pub draw_fn: DrawFunction,
    pub update_fn: UpdateFunction,
    pub register: ObjectRegister,
}

impl GameObject {}
