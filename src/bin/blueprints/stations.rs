use rand::Rng;

use solar_rustlib::core::*;
use objects::*;

pub struct TransfertStationBlueprint;

impl GameObjectBlueprint for TransfertStationBlueprint {
    fn produce(&mut self) -> ObjectHandle {
        use solar_rustlib::core::ObjectPropertyValue::*;

        let init_fn = Box::new(|reg: &mut ObjectRegister, _: &mut Rng| {
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
        });

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
                println!("{:?}", *v);
            });
        });

        GameObjectBuilder::with_visuals(ObjectType::Station,
                                        ObjectVisuals::square(10.0, (100, 200, 200)))
            .init_fn(init_fn)
            .update_fn(update_fn)
            .build()
    }
}
