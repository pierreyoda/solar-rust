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
        const TITLE: &'static str = "solar-rust";

        // Create the game window
        let builder = glutin::WindowBuilder::new()
            .with_title(TITLE.to_string())
            .with_gl(glutin::GlRequest::Specific(glutin::Api::OpenGl, (3, 2)));
        let (window, device, mut factory, main_color, _main_depth) =
            gfx_window_glutin::init::<sys::draw::ColorFormat, DepthFormat>(builder);

        // Initialize the game state
        let mut game = try!(game::SolarRust::init());

        // Game main loop
        'main: while let Some(_swing) = pegasus.swing() {
            window.swap_buffers().unwrap();
            for event in window.poll_events() {
                match event {
                    glutin::Event::KeyboardInput(_, _, Some(glutin::VirtualKeyCode::Escape)) |
                    glutin::Event::Closed => break 'main,
                    _ => ev_send.process_glutin(event),
                }
            }
        }

        Ok(())
    }
}
