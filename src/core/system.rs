use std::iter::Iterator;
use std::collections::HashMap;

use rand::Rng;

use super::object::{Object, ObjectHandle, ObjectType, ObjectVisuals, Orbit};
use super::generator::{ObjectsGenerator, AsteroidBeltGenerator, Distribution};

/// A gravitationally bound solar system, owning the multiple 'Object' instances
/// composing it.
#[derive(Debug)]
pub struct System {
    name: String,
    objects: HashMap<String, ObjectHandle>,
}

impl System {
    /// Try and add an 'Object' with the given ID to the 'System'.
    /// If the ID already exists, return false. Otherwise add the 'Object' and return true.
    pub fn add_object(&mut self, id: String, object: ObjectHandle) -> bool {
        if self.objects.contains_key(&id) {
            false
        } else {
            self.objects.insert(id, object);
            true
        }
    }

    pub fn generate_objects<R>(mut self,
                               base_id: &str,
                               spawner_fn: Box<Fn(&mut R) -> ObjectHandle>,
                               rng: &mut R,
                               n: usize)
                               -> Result<Self, String>
        where R: Rng
    {
        let generator = ObjectsGenerator::new(spawner_fn, rng);
        for (i, object) in generator.take(n).enumerate() {
            let id = format!("{}-{}", base_id, i + 1);
            if !self.add_object(id.clone(), object) {
                return Err(format!("System::generate_objects : id \"{}\" already taken", id));
            }
        }

        Ok(self)
    }

    pub fn update(&mut self, dt: f64) {
        for (_, object) in self.objects.iter_mut() {
            object.borrow_mut().update(dt);
        }
    }

    pub fn objects(&self) -> &HashMap<String, ObjectHandle> {
        &self.objects
    }

    pub fn test<R: 'static + Rng>(rng: &mut R) -> Result<Self, String> {
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
                                      altitude: 125.0,
                                      orbital_speed: 0.5,
                                      angle: 0.0,
                                      origin: sun.clone(),
                                  },
                                  ObjectVisuals::circle(40.0, (40, 15, 180)));
        let moon1 = Object::new(ObjectType::Moon,
                                (0.0, 0.0),
                                Orbit::Circular {
                                    altitude: 35.0,
                                    orbital_speed: 1.0,
                                    angle: 90.0,
                                    origin: planet1.clone(),
                                },
                                ObjectVisuals::circle(10.0, (200, 0, 150)));
        system.add_object("Sun".into(), sun.clone());
        system.add_object("planet_1".into(), planet1);
        system.add_object("planet_1-moon1".into(), moon1);

        let spawn_fn = try!(AsteroidBeltGenerator::new()
                                .radius(Distribution::Normal {
                                    mean: 10.0,
                                    std_dev: 2.0,
                                })
                                .orbit_altitude(Distribution::Normal {
                                    mean: 200.0,
                                    std_dev: 5.0,
                                })
                                .orbit_speed(Distribution::Constant(0.1))
                                .orbit_origin(sun)
                                .spawn_function());
        system = try!(system.generate_objects("asteroid_belt_1", spawn_fn, rng, 100));
        Ok(system)
    }
}
