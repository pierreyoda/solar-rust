use std::rc::Rc;

use super::object::{Object, ObjectType, ObjectVisuals, Orbit};

/// A gravitationally bound system, owning the multiple 'Object' instances
/// composing it.
pub struct System {
    name: String,
    objects: Vec<Rc<Object>>,
}

impl System {
    pub fn test() -> System {
        let sun = Object::new(ObjectType::Star,
                              (0.0, 0.0),
                              Orbit::Fixed,
                              ObjectVisuals::circle(150.0, (255, 255, 0)));
        let planet1 = Object::new(ObjectType::Planet,
                                  (0.0, 0.0),
                                  Orbit::Circular {
                                      altitude: 300.0,
                                      orbital_speed: 10.0,
                                      origin: sun.clone(),
                                  },
                                  ObjectVisuals::circle(50.0, (40, 15, 180)));
        let moon1 = Object::new(ObjectType::Moon,
                                (0.0, 0.0),
                                Orbit::Circular {
                                    altitude: 50.0,
                                    orbital_speed: 20.0,
                                    origin: planet1.clone(),
                                },
                                ObjectVisuals::circle(15.0, (200, 0, 150)));
        System {
            name: "Test System".into(),
            objects: vec![sun, planet1, moon1],
        }
    }
}
