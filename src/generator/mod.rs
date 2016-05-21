use std::f64::consts::PI;

use rand::Rng;
use rand::distributions::{Normal, Range, IndependentSample};

use core::object::*;

/// A structure able of randomly populating a 'System' with 'Object' instances.
pub struct ObjectsGenerator<'a, R: 'a + Rng> {
    fun: Box<Fn(&mut R) -> ObjectHandle>,
    rng: &'a mut R,
}

impl<'a, R: 'a + Rng> ObjectsGenerator<'a, R> {
    pub fn new(spawner_fn: Box<Fn(&mut R) -> ObjectHandle>,
               rng: &'a mut R)
               -> ObjectsGenerator<'a, R> {
        ObjectsGenerator {
            fun: spawner_fn,
            rng: rng,
        }
    }
}

impl<'a, R: 'a + Rng> Iterator for ObjectsGenerator<'a, R> {
    type Item = ObjectHandle;

    fn next(&mut self) -> Option<Self::Item> {
        Some((self.fun)(self.rng))
    }
}

/// Specifies a random distribution.
pub enum Distribution {
    Constant(f64),
    Normal {
        mean: f64,
        std_dev: f64,
    },
    Range {
        low: f64,
        high: f64,
    },
}

impl Distribution {
    /// Create from the specifications the rand-crate based distribution samplers.
    pub fn to_sampler<R: Rng>(&self) -> Box<Fn(&mut R) -> f64> {
        match self {
            &Distribution::Constant(value) => Box::new(move |_| value),
            &Distribution::Normal { mean, std_dev } => {
                let normal = Normal::new(mean, std_dev);
                Box::new(move |rng: &mut R| normal.ind_sample(rng))
            }
            &Distribution::Range { low, high } => {
                let range = Range::new(low, high);
                Box::new(move |rng: &mut R| range.ind_sample(rng))
            }
        }
    }
}

pub struct AsteroidBeltGenerator<R: Rng> {
    altitude: Option<Box<Fn(&mut R) -> f64>>,
    angle: Option<Box<Fn(&mut R) -> f64>>,
    radius: Option<Box<Fn(&mut R) -> f64>>,
    orbit_speed: Option<Box<Fn(&mut R) -> f64>>,
    orbit_origin: Option<SObjectHandle>,
}

macro_rules! setter_sampler {
    ($setter_name: ident, $property_name: ident) => (
        #[allow(dead_code)]
        pub fn $setter_name(mut self, $property_name: Distribution)
            -> AsteroidBeltGenerator<R> {
            self.$property_name = Some($property_name.to_sampler()); self
        }
    )
}

macro_rules! setter {
    ($setter_name: ident, $property_name: ident, $property_type: ty) => (
        #[allow(dead_code)]
        pub fn $setter_name(mut self, $property_name: $property_type)
            -> AsteroidBeltGenerator<R> {
                self.$property_name = Some($property_name); self
            }
    )
}

macro_rules! sample {
    ($self_:ident, $sampler_opt_fn: ident, $rng: ident) => (
        ($self_.$sampler_opt_fn.as_ref().unwrap())($rng)
    )
}

impl<'a, R: 'a + Rng> AsteroidBeltGenerator<R> {
    pub fn new() -> AsteroidBeltGenerator<R> {
        AsteroidBeltGenerator {
            altitude: None,
            angle: Some(Distribution::Range {
                            low: 0.0,
                            high: 2.0 * PI,
                        }
                        .to_sampler()),
            radius: None,
            orbit_speed: None,
            orbit_origin: None,
        }
    }

    setter_sampler!(radius, radius);
    setter_sampler!(orbit_altitude, altitude);
    setter_sampler!(orbit_start_angle, angle);
    setter_sampler!(orbit_speed, orbit_speed);
    setter!(orbit_origin, orbit_origin, SObjectHandle);

    /// Initialize and/or check all necessary members (for instance: textures)
    /// for spawning the objects, then return the spawning closure.
    pub fn spawn_function(self) -> Result<Box<Fn(&mut R) -> ObjectHandle + 'a>, String> {
        if self.altitude.is_none() {
            return Err(format!("AsteroidBeltGenerator::init : altitude distribution missing"));
        } else if self.angle.is_none() {
            return Err(format!("AsteroidBeltGenerator::init : angle distribution missing"));
        } else if self.orbit_speed.is_none() {
            return Err(format!("AsteroidBeltGenerator::init : orbit_speed distribution missing"));
        }

        Ok(Box::new(move |rng: &mut R| {
            let h: f64 = sample!(self, altitude, rng);
            let a: f64 = sample!(self, angle, rng);
            let r: f64 = sample!(self, radius, rng);

            let pos = (h * f64::cos(a), h * f64::sin(a));
            let orbit = match self.orbit_origin {
                Some(ref object) => {
                    Orbit::Circular {
                        altitude: h,
                        orbital_speed: sample!(self, orbit_speed, rng),
                        angle: a,
                        origin: object.clone(),
                    }
                }
                None => Orbit::Fixed,
            };

            SingleObject::new(ObjectType::Asteroid,
                              pos,
                              orbit,
                              ObjectVisuals::Circle {
                                  radius: r,
                                  color: [80, 80, 40, 255],
                              })
        }))
    }
}
