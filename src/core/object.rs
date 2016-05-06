use std::rc::Rc;
use std::cell::RefCell;

/// The different models of 'Objects' supported.
#[derive(Clone, Debug)]
pub enum ObjectType {
    Star,
    Planet,
    Moon,
    Asteroid,
    Station,
    Satellite,
}

/// Trait for any orbiting object.
pub trait Orbiting {
    /// If needed, return the new coordinates of the orbiting object,
    /// otherwise return None.
    /// - 'elapsed' is the total elapsed time for the object since its creation,
    ///   in seconds.
    fn compute(&self, elapsed: f64) -> Option<(f64, f64)>;
}

#[derive(Clone, Debug)]
pub enum Orbit {
    /// A circular orbit around another 'Object'.
    /// NB: a 'Weak' reference could be used to handle the case when the reference
    /// object is dropped.
    Circular {
        altitude: f64,
        orbital_speed: f64,
        origin: Rc<RefCell<Object>>,
    },
    /// "Fixed" orbit : the object will never move from its initial position.
    /// Useful for stars.
    Fixed,
}

impl<'o> Orbiting for Orbit {
    fn compute(&self, elapsed: f64) -> Option<(f64, f64)> {
        match self {
            &Orbit::Circular { altitude, orbital_speed, ref origin } => {
                let (x, y) = origin.borrow().position();
                Some((x + altitude * f64::cos(orbital_speed * elapsed),
                      y + altitude * f64::sin(orbital_speed * elapsed)))
            }
            &Orbit::Fixed => None,
        }
    }
}

pub type Color = [u8; 4];

/// Describes how an 'Object' should be represented in-game.
#[derive(Clone, Debug)]
pub enum ObjectVisuals {
    Circle {
        radius: f64,
        color: Color,
    },
}

impl ObjectVisuals {
    pub fn circle(radius: f64, rgb: (u8, u8, u8)) -> ObjectVisuals {
        ObjectVisuals::Circle {
            radius: radius,
            color: [rgb.0, rgb.1, rgb.2, 255],
        }
    }
}

/// An orbital object.
#[derive(Clone, Debug)]
pub struct Object {
    object_type: ObjectType,
    position: (f64, f64),
    orbit: Orbit,
    /// Total time since the object's simulation started, in seconds.
    time_alive: f64,
    visuals: ObjectVisuals,
}

impl Object {
    pub fn new(object_type: ObjectType,
               position: (f64, f64),
               orbit: Orbit,
               visuals: ObjectVisuals)
               -> Rc<RefCell<Object>> {
        Rc::new(RefCell::new(Object {
            object_type: object_type,
            position: position,
            orbit: orbit,
            time_alive: 0.0,
            visuals: visuals,
        }))
    }

    pub fn update(&mut self, dt: f64) {
        self.time_alive += dt;
        if let Some(new_position) = self.orbit.compute(self.time_alive) {
            self.position = new_position;
        }
    }

    pub fn position(&self) -> (f64, f64) {
        self.position
    }

    pub fn visuals(&self) -> &ObjectVisuals {
        &self.visuals
    }
}
