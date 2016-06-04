mod orbit;
mod system;

use std::rc::Rc;
use std::cell::RefCell;

use rand::Rng;
// use graphics::Transform;
use piston_window::{Context, G2d, Transformed};

use solar_rustlib::core::{ObjectType, ObjectVisuals, ObjectRegister};
use render::draw_fn_from_visuals;


pub use self::orbit::Orbit;
pub use self::system::GameSystem;

pub type ObjectHandle = Rc<RefCell<GameObject>>;
pub type ObjectDrawFunction = Box<Fn(Context, &mut G2d, &mut ObjectRegister)>;
pub type ObjectInitFunction = Box<Fn(&mut ObjectRegister, &mut Rng)>;
pub type ObjectUpdateFunction = Box<Fn(&mut ObjectRegister, f64)>;

/// An independent 'Object' evolving inside the game simulation.
pub struct GameObject {
    pub object_type: ObjectType,
    pub orbit: Orbit,
    draw_fn: ObjectDrawFunction,
    init_fn: ObjectInitFunction,
    pub update_fn: ObjectUpdateFunction,
    pub register: ObjectRegister,
    time_alive: f64,
    pub position: (f64, f64),
}

impl GameObject {
    pub fn init(&mut self, rng: &mut Rng) {
        match self.orbit {
            Orbit::Fixed((x, y)) => self.position = (x, y),
            _ => (),
        };
        (self.init_fn)(&mut self.register, rng);
    }

    pub fn update(&mut self, dt: f64) {
        self.time_alive += dt;
        self.position = self.orbit.compute(dt);
        (self.update_fn)(&mut self.register, dt);
    }

    pub fn draw(&mut self, c: Context, g: &mut G2d) {
        (self.draw_fn)(c.trans(self.position.0, self.position.1),
                       g,
                       &mut self.register);
    }
}

/// Implementing this trait by leveraging the 'GameObjectBuilder' structure
/// allows for easier definition of a single class of objects.
pub trait GameObjectBlueprint {
    fn produce(&mut self) -> ObjectHandle;
}

/// Convenience structure for building 'GameObject' with sensible defaults.
pub struct GameObjectBuilder {
    object_type: ObjectType,
    draw_fn: Option<ObjectDrawFunction>,
    init_fn: Option<ObjectInitFunction>,
    update_fn: Option<ObjectUpdateFunction>,
    orbit: Option<Orbit>,
}

macro_rules! setter_option {
    ($setter_name: ident, $property_name: ident, $property_type: ty) => (
        #[allow(dead_code)]
        pub fn $setter_name(mut self, $property_name: $property_type)
            -> Self {
                self.$property_name = Some($property_name); self
            }
    )
}

impl GameObjectBuilder {
    /// Get a new 'GameObject' builder with the given object type.
    pub fn new(object_type: ObjectType) -> Self {
        GameObjectBuilder {
            object_type: object_type,
            draw_fn: Some(draw_fn_from_visuals(&ObjectVisuals::Custom)),
            init_fn: None,
            update_fn: None,
            orbit: None,
        }
    }

    /// Get a new 'GameObject' builder with the given object type and a
    /// basic rendering function corresponding to the chosen 'ObjectVisuals'
    /// simple representation.
    pub fn with_visuals(object_type: ObjectType, visuals: ObjectVisuals) -> Self {
        GameObjectBuilder {
            object_type: object_type,
            draw_fn: Some(draw_fn_from_visuals(&visuals)),
            init_fn: None,
            update_fn: None,
            orbit: None,
        }
    }

    /// Build the object with all the registered parameters set and with sensible
    /// defaults for the parameters not specified at this point.
    /// An 'ObjectHandle' will be directly returned.
    pub fn build(self) -> ObjectHandle {
        assert!(self.draw_fn.is_some());
        Rc::new(RefCell::new(GameObject {
            object_type: self.object_type,
            orbit: self.orbit.unwrap_or(Orbit::Fixed((0.0, 0.0))),
            draw_fn: self.draw_fn.unwrap(),
            init_fn: self.init_fn.unwrap_or(Box::new(|_: &mut ObjectRegister, _: &mut Rng| {})),
            update_fn: self.update_fn
                           .unwrap_or(Box::new(|_: &mut ObjectRegister, _: f64| {})),
            register: ObjectRegister::new(),
            time_alive: 0f64,
            position: (0.0, 0.0),
        }))
    }

    setter_option!(draw_fn, draw_fn, ObjectDrawFunction);
    setter_option!(init_fn, init_fn, ObjectInitFunction);
    setter_option!(update_fn, update_fn, ObjectUpdateFunction);
    setter_option!(orbit, orbit, Orbit);
}
