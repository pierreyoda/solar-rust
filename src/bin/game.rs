use rand::{Rng, StdRng};
use piston_window::{Context, G2d};

use solar_rustlib::core::object::*;
use solar_rustlib::core::system::System;
use solar_rustlib::generator::{ObjectsGenerator, AsteroidBeltGenerator, Distribution};
use render::SystemRenderer;

/// Main structure for the solar-rust game proper.
pub struct SolarRust<R: Rng> {
    /// Random number generator used by the game (generation, simulation...).
    rng: R,
    /// The currently played solar 'System'.
    system: System,
    system_renderer: SystemRenderer,
}

impl<R: Rng> SolarRust<R> {
    pub fn update(&mut self, dt: f64) {
        self.system.update(dt);
    }

    pub fn render(&mut self, c: Context, g: &mut G2d) {
        self.system_renderer.render(&self.system, c, g);
    }
}

impl SolarRust<StdRng> {
    pub fn test_game() -> Result<SolarRust<StdRng>, String> {
        let mut game = SolarRust {
            rng: try!(StdRng::new().map_err(|e| format!("{:?}", e))),
            system: System::new("Test system"),
            system_renderer: SystemRenderer::new(),
        };
        game.system = try!(test_system(&mut game.rng));
        game.system_renderer.update_cache(&game.system);
        Ok(game)
    }
}

fn test_system<R: 'static + Rng>(rng: &mut R) -> Result<System, String> {
    let mut system = System::new("Test system");
    let sun = SingleObject::new(ObjectType::Star,
                                (300.0, 225.0),
                                Orbit::Fixed,
                                ObjectVisuals::circle(150.0, (255, 255, 0)));
    let planet1 = SingleObject::new(ObjectType::Planet,
                                    (0.0, 0.0),
                                    Orbit::Circular {
                                        altitude: 125.0,
                                        orbital_speed: 0.5,
                                        angle: 0.0,
                                        origin: sun.clone(),
                                    },
                                    ObjectVisuals::circle(40.0, (40, 15, 180)));
    let moon1 = SingleObject::new(ObjectType::Moon,
                                  (0.0, 0.0),
                                  Orbit::Circular {
                                      altitude: 35.0,
                                      orbital_speed: 1.0,
                                      angle: 90.0,
                                      origin: planet1.clone(),
                                  },
                                  ObjectVisuals::circle(10.0, (200, 0, 150)));
    system.add_object("Sun".into(), from_single(sun.clone()));
    system.add_object("planet_1".into(), from_single(planet1));
    system.add_object("planet_1-moon1".into(), from_single(moon1));

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
    system = try!(system.generate_objects("asteroid_belt_1", spawn_fn, rng, 25));

    Ok(system)
}
