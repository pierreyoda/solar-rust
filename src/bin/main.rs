mod app;
mod game;
mod objects;
mod render;

extern crate rand;
extern crate piston_window;
extern crate graphics;

extern crate solar_rustlib;

fn main() {
    match app::SolarRustApp::new().run() {
        Err(why) => println!("Error while running solar-rust:\n{}", why),
        Ok(_) => (),
    }
}
