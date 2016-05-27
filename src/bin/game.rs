use rand::{Rng, StdRng};
use piston_window::{Context, G2d};

use solar_rustlib::core::{ObjectType, ObjectVisuals};
// use solar_rustlib::generator::{ObjectsGenerator, AsteroidBeltGenerator, Distribution};

use objects::*;

/// Main structure for the solar-rust game proper.
pub struct SolarRust<R: Rng> {
    /// Random number generator used by the game (generation, simulation...).
    rng: R,
    system: GameSystem,
}

impl<R: Rng> SolarRust<R> {
    pub fn update(&mut self, dt: f64) {
        self.system.update(dt);
    }

    pub fn render(&mut self, c: Context, g: &mut G2d) {
        self.system.render(c, g);
    }
}

impl SolarRust<StdRng> {
    pub fn test_game() -> Result<SolarRust<StdRng>, String> {
        let mut rng = try!(StdRng::new().map_err(|e| format!("{:?}", e)));
        let mut system = try!(test_system(&mut rng));
        system.init(&mut rng);
        Ok(SolarRust {
            rng: rng,
            system: system,
        })
    }
}

fn test_system<R: 'static + Rng>(rng: &mut R) -> Result<GameSystem, String> {
    use blueprints::TransfertStationBlueprint;

    let mut system = GameSystem::new("Test system");

    let sun = GameObjectBuilder::with_visuals(ObjectType::Star,
                                              ObjectVisuals::circle(75.0, (255, 255, 0)))
                  .orbit(Orbit::Fixed((300.0, 225.0)))
                  .build();
    let planet1 = GameObjectBuilder::with_visuals(ObjectType::Planet,
                                                  ObjectVisuals::circle(40.0, (40, 15, 180)))
                      .orbit(Orbit::Circular {
                          altitude: 175.0,
                          orbital_speed: 0.1,
                          angle: 0f64.to_radians(),
                          origin: sun.clone(),
                      })
                      .build();
    let moon1 = GameObjectBuilder::with_visuals(ObjectType::Moon,
                                                ObjectVisuals::circle(10.0, (200, 0, 150)))
                    .orbit(Orbit::Circular {
                        altitude: 50.0,
                        orbital_speed: 0.3,
                        angle: -90f64.to_radians(),
                        origin: planet1.clone(),
                    })
                    .build();

    let station1 = TransfertStationBlueprint.produce();
    station1.borrow_mut().orbit = Orbit::new_relative_orbit(60f64.to_radians(), 40.0, planet1.clone());

    system.add_object(sun);
    system.add_object(planet1);
    system.add_object(moon1);
    system.add_object(station1);

    // let spawn_fn = try!(AsteroidBeltGenerator::new()
    //                         .radius(Distribution::Normal {
    //                             mean: 10.0,
    //                             std_dev: 2.0,
    //                         })
    //                         .orbit_altitude(Distribution::Normal {
    //                             mean: 200.0,
    //                             std_dev: 5.0,
    //                         })
    //                         .orbit_speed(Distribution::Constant(0.1))
    //                         .orbit_origin(sun)
    //                         .spawn_function());
    // system = try!(system.generate_objects("asteroid_belt_1", spawn_fn, rng, 25));

    Ok(system)
}
