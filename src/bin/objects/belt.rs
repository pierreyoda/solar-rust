use std::rc::Rc;
use std::cell::RefCell;
use std::f64::consts::PI;

use rand::Rng;
use piston_window::{Context, G2d, Transformed, Ellipse};

use solar_rustlib::core::{ObjectType, ObjectRegister};
use solar_rustlib::generator::*;
use solar_rustlib::util::*;
use objects::*;

const ASTEROID_BORDER_COLOR: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
const ASTEROID_BORDER_RADIUS: f64 = 1.0;

/// An individual asteroid.
pub struct Asteroid {
    radius: f64,
    color: [f32; 4],
    orbit_altitude: f64,
    orbit_angle: f64,
    orbit_speed: f64,
    position: Vector2f,
    visuals: (Ellipse, f64),
}

/// 'Object' regrouping several asteroids.
pub struct AsteroidBeltObject {
    register: ObjectRegister,
    asteroids: Vec<Asteroid>,
    origin_position: Vector2f,
    origin_orbit: Orbit,
}

impl GameObject for AsteroidBeltObject {
    fn init(&mut self, _: &mut Rng) {
        match self.origin_orbit {
            Orbit::Fixed((x, y)) => self.origin_position = (x, y),
            _ => (),
        };
    }

    fn update(&mut self, dt: f64) {
        self.origin_position = self.origin_orbit.compute(dt);
        for asteroid in &mut self.asteroids {
            let angle = (asteroid.orbit_angle - asteroid.orbit_speed * dt) % (2.0 * PI);
            asteroid.orbit_angle = angle;
            asteroid.position = (asteroid.orbit_altitude * angle.cos(),
                                 asteroid.orbit_altitude * angle.sin());
        }
    }

    fn render(&self, c: Context, g: &mut G2d) {


        let c = c.trans(self.origin_position.0, self.origin_position.1);
        for asteroid in &self.asteroids {
            let context = c.trans(asteroid.position.0, asteroid.position.1);
            asteroid.visuals.0.draw([asteroid.visuals.1,
                                     asteroid.visuals.1,
                                     asteroid.radius,
                                     asteroid.radius],
                                    &context.draw_state,
                                    context.transform,
                                    g);
        }
    }

    fn object_type(&self) -> ObjectType {
        ObjectType::Asteroid
    }

    fn position(&self) -> Vector2f {
        self.origin_position
    }

    fn register(&self) -> &ObjectRegister {
        &self.register
    }
    fn register_mut(&mut self) -> &mut ObjectRegister {
        &mut self.register
    }
}

pub struct AsteroidBeltBlueprint<R: Rng> {
    generator: AsteroidGenerator<R>,
    asteroids_number: Option<usize>,
    orbit: Option<Orbit>,
}

impl<R: Rng> AsteroidBeltBlueprint<R> {
    pub fn new() -> Self {
        AsteroidBeltBlueprint {
            generator: AsteroidGenerator::default(),
            asteroids_number: None,
            orbit: None,
        }
    }

    builder_setters!(
        options => {
            asteroids_number { asteroids_number: usize },
            origin_orbit { orbit: Orbit }
        };
        others => {
            generator { generator: AsteroidGenerator<R> }
        }
    );
}

impl<R: Rng> GameObjectBlueprint<R> for AsteroidBeltBlueprint<R> {
    fn produce(&mut self, rng: &mut R) -> Result<ObjectHandle, String> {
        let n = unwrap_or_err!(self.asteroids_number,
                               "AsteroidBeltBlueprint : missing asteroids number parameter");
        let orbit = self.orbit.as_ref().unwrap_or(&Orbit::Fixed((0.0, 0.0))).clone();

        Ok(Rc::new(RefCell::new(AsteroidBeltObject {
            register: ObjectRegister::new(),
            asteroids: try!(self.generator.generate_many(rng, n)),
            origin_position: match &orbit {
                &Orbit::Fixed(position) => position,
                _ => (0.0, 0.0),
            },
            origin_orbit: orbit,
        })))
    }
}

pub struct AsteroidGenerator<R: Rng> {
    altitude: Option<SamplerFunction<f64, R>>,
    angle: Option<SamplerFunction<f64, R>>,
    radius: Option<SamplerFunction<f64, R>>,
    speed: Option<SamplerFunction<f64, R>>,
    color: ColorGenerator<R>,
}

impl<R: Rng> AsteroidGenerator<R> {
    builder_setters!(
        options => {
            altitude { altitude: SamplerFunction<f64, R> },
            angle { angle: SamplerFunction<f64, R> },
            radius { radius: SamplerFunction<f64, R> },
            orbital_speed { speed: SamplerFunction<f64, R> }
        };
        others => {
            color { color: ColorGenerator<R> }
        }
    );
}

impl<R: Rng> TypeGenerator<R> for AsteroidGenerator<R> {
    type Generated = Asteroid;

    fn default() -> AsteroidGenerator<R> {
        AsteroidGenerator {
            altitude: None,
            angle: Some(Distribution::Range {
                            low: 0.0,
                            high: 2.0 * PI,
                        }
                        .to_sampler()),
            radius: None,
            speed: None,
            color: ColorGenerator::default(),
        }
    }

    fn generate(&mut self, rng: &mut R) -> Result<Asteroid, String> {
        if self.radius.is_none() {
            Err("AsteroidGenerator : unspecified radius distribution".into())
        } else if self.altitude.is_none() {
            Err("AsteroidGenerator : unspecified altitude distribution".into())
        } else if self.angle.is_none() {
            Err("AsteroidGenerator : unspecified angle distribution".into())
        } else if self.speed.is_none() {
            Err("AsteroidGenerator : unspecified speed distribution".into())
        } else {
            let radius = generator_sample!(self, radius, rng);
            let color = try!(self.color.generate(rng));
            let ellipse = Ellipse::new_border(ASTEROID_BORDER_COLOR, ASTEROID_BORDER_RADIUS)
                              .color(color);
            let neg_half_radius = radius / -2.0;
            Ok(Asteroid {
                radius: radius,
                color: color,
                orbit_altitude: generator_sample!(self, altitude, rng),
                orbit_angle: generator_sample!(self, angle, rng),
                orbit_speed: generator_sample!(self, speed, rng),
                position: (0.0, 0.0),
                visuals: (ellipse, neg_half_radius),
            })
        }
    }
}
