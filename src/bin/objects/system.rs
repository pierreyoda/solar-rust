use piston_window::{Context, G2d};

use solar_rustlib::core::{ObjectRegister, ObjectPropertyValue};
use super::ObjectHandle;

/// The game structure representing a gravitationally bound solar system, owning
/// the multiple 'GameObject' instances composing it inside the game simulation.
pub struct GameSystem {
    /// General-purpose 'ObjectRegister' owned by the system used to store values
    /// not concerning one entity in particular.
    register: ObjectRegister,
    objects: Vec<ObjectHandle>,
}

impl GameSystem {
    pub fn new(name: &str) -> Self {
        use solar_rustlib::core::ObjectPropertyValue::*;

        let mut register = ObjectRegister::new();
        register.add_constant("system_name",
                              Text(name.into()),
                              "The name of the current solar system.");
        register.add_constant("data_version",
                              Integer(0),
                              "For evolution purposes, the version ID for the data of all the \
                               objects' registers in the system (0 = alpha-dev).");

        GameSystem {
            register: register,
            objects: Vec::new(),
        }
    }

    pub fn add_object(&mut self, object: ObjectHandle) {
        self.objects.push(object);
    }

    pub fn update(&mut self, dt: f64) {
        for object_handle in &mut self.objects {
            object_handle.borrow_mut().update(dt);
        }
    }

    pub fn render(&mut self, c: Context, g: &mut G2d) {
        for object_handle in &mut self.objects {
            object_handle.borrow_mut().draw(c, g);
        }
    }
}
