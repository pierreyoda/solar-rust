use std::path::Path;

use rand::StdRng;
use piston_window::*;
use conrod;

use game::SolarRust;
use textures::TextureStore;

pub struct SolarRustApp;

pub type Backend = (<G2d<'static> as conrod::Graphics>::Texture, Glyphs);
pub type Ui = conrod::Ui<Backend>;
pub type UiCell<'a> = conrod::UiCell<'a, Backend>;

impl SolarRustApp {
    pub fn new() -> SolarRustApp {
        SolarRustApp
    }

    pub fn run(&mut self) -> Result<(), String> {
        let mut window: PistonWindow = WindowSettings::new("solar-rust", [1080, 720])
                                           .exit_on_esc(false)
                                           .samples(4) // anti-aliasing
                                           .build()
                                           .unwrap();
        window = window.ups(60).max_fps(60);

        let mut ui = {
            let font_path = Path::new("assets/fonts/NotoSans/NotoSans-Regular.ttf");
            let theme = conrod::Theme::default();
            let glyph_cache = Glyphs::new(&font_path, window.factory.clone());
            Ui::new(glyph_cache.expect("could not find or load the fonts"),
                    theme)
        };

        let display_size = window.draw_size();
        let store = TextureStore::new(&mut window, Path::new("assets/"));
        let mut game: SolarRust<StdRng> = try!(SolarRust::test_game(store, display_size));

        while let Some(e) = window.next() {
            ui.handle_event(&e);
            if let Some(u) = e.update_args() {
                game.update(u.dt);
                e.update(|_| ui.set_widgets(|mut ui| game.update_ui(&mut ui)));
                // game.set_ui(&mut e);
            } else if let Some(_) = e.render_args() {
                window.draw_2d(&e, |c, g| {
                    clear([0.0; 4], g);
                    game.render(c, g);
                    ui.draw(c, g);
                });
            } else if let Some(_) = e.resize_args() {
                game.on_display_resize(window.draw_size());
                ui.needs_redraw();
            }
        }

        Ok(())
    }
}
