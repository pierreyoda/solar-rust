use rand::{Rng, StdRng};
use cgmath::Rad;
use specs;
use piston_window::*;
use solar_rustlib::core::*;

/// Main structure for the solar-rust game proper.
pub struct SolarRust<R: Rng> {
    /// Random number generator used by the game (generation, simulation...).
    rng: R,
    /// The world containing all our entities and their associated components.
    world: specs::World,
}

impl SolarRust<StdRng> {
    pub fn init() -> Result<Self, String> {
        let mut rng = try!(StdRng::new().map_err(|e| format!("{:?}", e)));

        let mut world = specs::World::new();
        world.register::<SpatialComponent>();
        world.register::<InertialComponent>();
        world.register::<DrawCircleComponent>();
        world.register::<DrawRectangleComponent>();
        world.register::<ArtificialObjectComponent>();

        try!(generate_test_system(&mut rng, &mut world));

        Ok(SolarRust {
               rng: rng,
               world: world,
           })
    }
}

fn generate_test_system<R: Rng>(r: &mut R, w: &mut specs::World) -> Result<(), String> {
    // Sun creation
    w.create_now()
        .with(SpatialComponent {
                  position: Point2f::new(0.0, 0.0),
                  angle: Rad(0.0),
              })
        .with(DrawCircleComponent {
                  color: [1.0, 1.0, 0.0, 1.0],
                  radius: 75.0,
              });

    Ok(())
}
