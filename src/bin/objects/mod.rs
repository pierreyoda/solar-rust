mod orbit;
mod system;

use std::rc::Rc;
use std::cell::RefCell;

use rand::Rng;
// use graphics::Transform;
use piston_window::{Context, G2d, Transformed};

use solar_rustlib::core::{ObjectType, ObjectVisuals, ObjectRegister};
use solar_rustlib::util::Vector2f;
use render::draw_fn_from_visuals;
pub use self::orbit::Orbit;
pub use self::system::GameSystem;


pub type ObjectHandle = Rc<RefCell<GameObject>>;
pub type DefaultObjectDrawFn = Box<Fn(Context, &mut G2d)>;
pub type DefaultObjectUpdateFn = Box<Fn(&mut ObjectRegister, f64)>;

/// An independent object evolving inside the game simulation.
pub trait GameObject {
    fn init(&mut self, r: &mut Rng);
    fn update(&mut self, elapsed: f64);
    fn render(&self, c: Context, g: &mut G2d);

    fn object_type(&self) -> ObjectType;
    fn position(&self) -> Vector2f;

    fn register(&self) -> &ObjectRegister;
    fn register_mut(&mut self) -> &mut ObjectRegister;
}

/// The default GameObject implementation.
/// It aims to be sufficient for most of the simple objects in the game.
pub struct DefaultObject {
    /// Time in seconds since the initialization of the object.
    time_alive: f64,
    /// The object's type.
    object_type: ObjectType,
    /// The object's current position.
    position: Vector2f,
    /// The object's current orbit.
    orbit: Orbit,
    /// The object's property register.
    register: ObjectRegister,
    /// The object's update function.
    update_fn: DefaultObjectUpdateFn,
    /// The object's drawing function.
    draw_fn: DefaultObjectDrawFn,
}

impl GameObject for DefaultObject {
    fn init(&mut self, _: &mut Rng) {
        match self.orbit {
            Orbit::Fixed((x, y)) => self.position = (x, y),
            _ => (),
        };
    }

    fn update(&mut self, dt: f64) {
        self.time_alive += dt;
        self.position = self.orbit.compute(dt);
        (self.update_fn)(&mut self.register, dt);
    }

    fn render(&self, c: Context, g: &mut G2d) {
        (self.draw_fn)(c.trans(self.position.0, self.position.1), g);
    }

    fn object_type(&self) -> ObjectType {
        self.object_type.clone()
    }

    fn position(&self) -> Vector2f {
        self.position
    }

    fn register(&self) -> &ObjectRegister {
        &self.register
    }

    fn register_mut(&mut self) -> &mut ObjectRegister {
        &mut self.register
    }
}

/// Implementing this trait allows for easier definition of a class of similar
/// object instances.
pub trait GameObjectBlueprint {
    /// Create and return a new default blueprint.
    fn default() -> Self;
    /// Initialize a new object according to the blueprint specifications.
    fn produce<R: Rng>(&mut self, rng: &mut R) -> ObjectHandle;
}

/// Convenience structure for building any 'DefaultObject' with sensible defaults.
pub struct DefaultObjectBuilder {
    object_type: ObjectType,
    register: ObjectRegister,
    draw_fn: Option<DefaultObjectDrawFn>,
    update_fn: Option<DefaultObjectUpdateFn>,
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

impl DefaultObjectBuilder {
    /// Get a new 'DefaultObject' builder with the given object type and a
    /// basic rendering function corresponding to the chosen 'ObjectVisuals'
    /// simple representation.
    pub fn with_visuals(object_type: ObjectType, visuals: ObjectVisuals) -> Self {
        DefaultObjectBuilder {
            object_type: object_type,
            register: ObjectRegister::new(),
            draw_fn: Some(draw_fn_from_visuals(&visuals)),
            update_fn: None,
            orbit: None,
        }
    }

    /// Build the object with all the registered parameters set and with sensible
    /// defaults for the parameters not specified at this point.
    /// An 'ObjectHandle' will be directly returned.
    pub fn build(self) -> ObjectHandle {
        assert!(self.draw_fn.is_some());
        Rc::new(RefCell::new(DefaultObject {
            time_alive: 0f64,
            object_type: self.object_type,
            position: (0.0, 0.0),
            orbit: self.orbit.unwrap_or(Orbit::Fixed((0.0, 0.0))),
            register: self.register,
            update_fn: self.update_fn
                           .unwrap_or(Box::new(|_: &mut ObjectRegister, _: f64| {})),
            draw_fn: self.draw_fn.unwrap(),
        }))
    }

    pub fn register(mut self, reg: ObjectRegister) -> Self {
        self.register = reg;
        self
    }

    setter_option!(draw_fn, draw_fn, DefaultObjectDrawFn);
    setter_option!(update_fn, update_fn, DefaultObjectUpdateFn);
    setter_option!(orbit, orbit, Orbit);
}
