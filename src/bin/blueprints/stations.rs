use rand::Rng;

use solar_rustlib::core::{ObjectRegister, ObjectType, ObjectVisuals};
#[macro_use]
use solar_rustlib::util::*;
use objects::*;

pub struct TransfertStationBlueprint {
    orbit: Option<Orbit>,
}

impl TransfertStationBlueprint {
    pub fn new() -> TransfertStationBlueprint {
        TransfertStationBlueprint { orbit: None }
    }

    builder_setters!(options => { orbit { orbit: Orbit } }; others => {});
}

impl<R: Rng> GameObjectBlueprint<R> for TransfertStationBlueprint {
    fn produce(&mut self, _: &mut R) -> Result<ObjectHandle, String> {
        use solar_rustlib::core::ObjectPropertyValue::*;

        let mut reg = ObjectRegister::new();
        reg.add_property("level", Integer(1), "Current level.");

        reg.add_property("minerals",
                         Float(0.0),
                         "Current amount of minerals in stock.");
        reg.add_property("minerals_max",
                         Float(1000.0),
                         "Maximum amount of minerals that can be stored.");

        reg.add_property("energy", Float(0.0), "Current amount of energy in stock.");
        reg.add_property("energy_max",
                         Float(1000.0),
                         "Maximum amount of energy that can be stored.");
        reg.add_property("energy_rate",
                         Float(10.0),
                         "Amount of energy produced in situ by seconds.");

        reg.set_display_name("minerals", "Minerals");
        reg.set_display_name("minerals_max", "Minerals maximum stock");
        reg.set_display_name("energy", "Energy");
        reg.set_display_name("energy_max", "Energy maximum stock");
        reg.set_display_name("energy_rate", "Energy production rate");

        let update_fn = Box::new(|reg: &mut ObjectRegister, dt: f64| {
            let minerals_max = reg.get_float("minerals_max").unwrap().clone();
            reg.get_float_mut("minerals").map(|v| {
                if *v > minerals_max {
                    *v = minerals_max
                }
            });

            let energy_max = reg.get_float("energy_max").unwrap().clone();
            let energy_rate = reg.get_float("energy_rate").unwrap().clone();
            reg.get_float_mut("energy").map(|v| {
                *v += energy_rate * dt;
                if *v > energy_max {
                    *v = energy_max
                }
            });
        });

        let orbit = self.orbit.as_ref().unwrap_or(&Orbit::Fixed((0.0, 0.0))).clone();
        Ok(DefaultObjectBuilder::with_visuals(ObjectType::Station,
                                              ObjectVisuals::square(10.0, (100, 200, 200)))
               .orbit(orbit)
               .register(reg)
               .update_fn(update_fn)
               .build())
    }
}
