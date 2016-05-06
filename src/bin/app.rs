use piston_window::*;

use solar_rustlib::core;
use super::render::SystemRenderer;

pub struct SolarRustApp;

impl SolarRustApp {
    pub fn new() -> SolarRustApp {
        SolarRustApp
    }

    pub fn run(&mut self) {
        let mut window: PistonWindow = WindowSettings::new("solar-rust", [640, 480])
                                           .exit_on_esc(false)
                                           .samples(4) // anti-aliasing
                                           .build()
                                           .unwrap();

        let mut system = core::system::System::test();
        let mut system_renderer = SystemRenderer::new();
        system_renderer.update_cache(&system);

        while let Some(e) = window.next() {
            if let Some(u) = e.update_args() {
                system.update(u.dt);
            }
            if let Some(_) = e.render_args() {
                window.draw_2d(&e, |c, g| {
                    clear([1.0; 4], g);
                    system_renderer.render(&system, c, g);
                });
            }

        }
    }
}
