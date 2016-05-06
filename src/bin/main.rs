mod app;
mod render;

extern crate piston_window;
extern crate graphics;

extern crate solar_rustlib;

fn main() {
    app::SolarRustApp::new().run();
}
