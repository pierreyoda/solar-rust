mod app;
// mod blueprints;
mod game;
// mod objects;
// mod render;
// mod textures;
mod ui;

#[macro_use] extern crate log;
extern crate rand;
extern crate nalgebra as na;
extern crate specs;
extern crate gfx_window_glutin;
extern crate glutin;
#[macro_use] extern crate solar_rustlib;

fn main() {
    info!("Launching solar-rust...");
    match app::SolarRustApp::new().run() {
        Err(why) => error!("Error while running solar-rust:\n{}", why),
        Ok(_) => (),
    }
    info!("Quitting solar-rust.");
}
