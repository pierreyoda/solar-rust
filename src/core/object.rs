/// The different models of 'Objects' supported.
#[derive(Clone, Debug, PartialEq)]
pub enum ObjectType {
    Star,
    Planet,
    Moon,
    Asteroid,
    Station,
    Satellite,
}

/// A color encoded as 4 8-bit RGBA channels.
pub type Color = [u8; 4];

/// Describes how an 'Object' should be represented in-game.
/// Only defines very crude guidelines, advanced representation must be defined
/// game-side.
#[derive(Clone, Debug)]
pub enum ObjectVisuals {
    Circle {
        radius: f64,
        color: Color,
    },
    Square {
        size: f64,
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

    pub fn square(size: f64, rgb: (u8, u8, u8)) -> ObjectVisuals {
        ObjectVisuals::Square {
            size: size,
            color: [rgb.0, rgb.1, rgb.2, 255],
        }
    }
}
