use std::f64::consts::PI;

use solar_rustlib::util::Norm;
use super::ObjectHandle;

#[derive(Clone)]
pub enum Orbit {
    /// A circular orbit around another 'Object'.
    /// NB: a 'Weak' reference could be used to handle the case when the reference
    /// object is dropped.
    Circular {
        altitude: f64,
        orbital_speed: f64,
        /// Current angle of the orbit, in radians.
        angle: f64,
        origin: ObjectHandle,
    },
    /// "Relative" orbit : the object will always be at the specified position
    /// relative to the origin object.
    Relative {
        position: (f64, f64),
        origin: ObjectHandle,
    },
    /// "Fixed" orbit : the object will never move from its specified initial
    /// position. Useful for stars.
    Fixed((f64, f64)),
}

impl Orbit {
    /// Return the new coordinates of the orbiting object.
    /// - 'elapsed' is the total elapsed time for the object since its creation,
    ///   in seconds.
    pub fn compute(&mut self, elapsed: f64) -> (f64, f64) {
        match *self {
            Orbit::Circular { altitude, orbital_speed, ref mut angle, ref origin } => {
                *angle = (*angle - orbital_speed * elapsed) % (2.0 * PI);
                let (x, y) = origin.borrow().position;
                (x + altitude * angle.cos(), y + altitude * angle.sin())
            }
            Orbit::Relative { position, ref origin } => {
                let (x, y) = origin.borrow().position;
                (x + position.0, y + position.1)
            }
            Orbit::Fixed(position) => position,
        }
    }

    /// If it makes sense, return the maximum altitude the orbiting entity can find
    /// itself while orbiting over its origin point.
    fn max_altitude(&self) -> Option<f64> {
        match *self {
            Orbit::Circular { altitude, .. } => Some(altitude),
            Orbit::Relative { position, ref origin } => {
                Some(origin.borrow().orbit.max_altitude().unwrap_or(0.0) + position.norm())
            }
            Orbit::Fixed(_) => None,
        }
    }
}
