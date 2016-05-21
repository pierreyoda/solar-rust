use std::iter::Iterator;
use std::collections::HashMap;

use rand::Rng;

use super::object::*;
use generator::ObjectsGenerator;

/// A gravitationally bound solar system, owning the multiple 'Object' instances
/// composing it.
pub struct System {
    name: String,
    objects: HashMap<String, ObjectHandle>,
}

impl System {
    pub fn new(name: &str) -> System {
        System {
            name: name.into(),
            objects: HashMap::new(),
        }
    }

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
}
