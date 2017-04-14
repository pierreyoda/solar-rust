use piston_window::*;
use super::game;

pub struct SolarRustApp;

// pub type Backend = (<G2d<'static> as conrod::Graphics>::Texture, Glyphs);
// pub type Ui = conrod::Ui<Backend>;
// pub type UiCell<'a> = conrod::UiCell<'a, Backend>;

impl SolarRustApp {
    pub fn new() -> SolarRustApp {
        SolarRustApp
    }

    pub fn run(&mut self) -> Result<(), String> {
        const WIDTH: u32 = 1080;
        const HEIGHT: u32 = 720;

        // Create the game window
        let mut window: PistonWindow = WindowSettings::new("solar-rust", [WIDTH, HEIGHT])
                                           .exit_on_esc(false)
                                           .samples(4) // anti-aliasing
                                           .build()
                                           .unwrap();
        window = window.ups(60).max_fps(60);

        // Initialize the game state
        let mut game = try!(game::SolarRust::init());

        // Game main loop
        while let Some(event) = window.next() {
            window.draw_2d(&event, |context, graphics| {
                clear([0.0, 0.0, 0.0, 1.0], graphics);
            });
        }

        Ok(())
    }
}
