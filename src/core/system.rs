use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

use super::object::{Object, ObjectType, ObjectVisuals, Orbit};

/// A gravitationally bound system, owning the multiple 'Object' instances
/// composing it.
#[derive(Debug)]
pub struct System {
    name: String,
    objects: HashMap<String, Rc<RefCell<Object>>>,
}

impl System {
    /// Try and add an 'Object' with the given ID to the 'System'.
    /// If the ID already exists, return false. Otherwise add the 'Object' and return true.
    pub fn add_object(&mut self, id: String, object: Rc<RefCell<Object>>) -> bool {
        if self.objects.contains_key(&id) {
            false
        } else {
            self.objects.insert(id, object);
            true
        }
    }

    pub fn update(&mut self, dt: f64) {
        for (_, object) in self.objects.iter_mut() {
            object.borrow_mut().update(dt);
        }
    }

    pub fn objects(&self) -> &HashMap<String, Rc<RefCell<Object>>> {
        &self.objects
    }

    pub fn test() -> System {
        let mut system = System {
            name: "Test System".into(),
            objects: HashMap::new(),
        };
        let sun = Object::new(ObjectType::Star,
                              (300.0, 225.0),
                              Orbit::Fixed,
                              ObjectVisuals::circle(150.0, (255, 255, 0)));
        let planet1 = Object::new(ObjectType::Planet,
                                  (0.0, 0.0),
                                  Orbit::Circular {
                                      altitude: 200.0,
                                      orbital_speed: 1.0,
                                      origin: sun.clone(),
                                  },
                                  ObjectVisuals::circle(50.0, (40, 15, 180)));
        let moon1 = Object::new(ObjectType::Moon,
                                (0.0, 0.0),
                                Orbit::Circular {
                                    altitude: 50.0,
                                    orbital_speed: 3.0,
                                    origin: planet1.clone(),
                                },
                                ObjectVisuals::circle(15.0, (200, 0, 150)));
        system.add_object("Sun".into(), sun);
        system.add_object("planet1".into(), planet1);
        system.add_object("planet1-moon1".into(), moon1);

        system
    }
}
