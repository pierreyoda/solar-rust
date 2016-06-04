mod app;
mod blueprints;
mod game;
mod objects;
mod render;
mod textures;
mod ui;

extern crate rand;
extern crate gfx_device_gl;
extern crate piston_window;
extern crate graphics;
#[macro_use] extern crate conrod;

extern crate solar_rustlib;

fn main() {
    match app::SolarRustApp::new().run() {
        Err(why) => println!("Error while running solar-rust:\n{}", why),
        Ok(_) => (),
    }
}
