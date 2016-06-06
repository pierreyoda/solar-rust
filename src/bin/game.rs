use rand::{Rng, StdRng};
use piston_window::*;

use solar_rustlib::core::{ObjectType, ObjectVisuals, ObjectPropertyValue};
use solar_rustlib::generator::{TypeGenerator, Distribution};
use app::UiCell;
use ui::ResourceWidget;
use textures::{TextureStore, TextureHandle};

use objects::*;

const UI_BAR_HEIGHT_FACTOR: f64 = 1.0 / 10.0;
const UI_MAIN_HEIGHT_FACTOR: f64 = 1.0 - 2.0 * UI_BAR_HEIGHT_FACTOR;

/// Main structure for the solar-rust game proper.
pub struct SolarRust<R: Rng> {
    /// Random number generator used by the game (generation, simulation...).
    rng: R,
    system: GameSystem,
    display_width: f64,
    display_height: f64,
    gameview_width: f64,
    gameview_height: f64,
    texture_store: TextureStore,
    texture_icon_minerals: TextureHandle,
    texture_icon_energy: TextureHandle,
    object_home: ObjectHandle,
}

impl<R: Rng> SolarRust<R> {
    pub fn on_display_resize(&mut self, display_size: Size) {
        let (w, h) = (display_size.width as f64, display_size.height as f64);
        self.display_width = w;
        self.display_height = h;
        self.gameview_width = w * UI_MAIN_HEIGHT_FACTOR;
        self.gameview_height = h * UI_MAIN_HEIGHT_FACTOR;
    }

    pub fn update(&mut self, dt: f64) {
        self.system.update(dt);
    }

    pub fn update_ui(&mut self, ui: &mut UiCell) {
        use conrod::*;

        widget_ids!(
            MASTER,
            SECTION_TOP,
            SECTION_MAIN,
            SECTION_BOTTOM,
            TITLE,
            RESOURCES_MINERALS,
            RESOURCES_ENERGY,
        );

        // Master canvas :
        // /------------------\
        // |   TOP UI BAR     |
        // |      ----        |
        // |                  |
        // |      MAIN        |
        // |                  |
        // |      ----        |
        // |  BOTTOM UI BAR   |
        // \------------------/
        let bar_height = self.display_height * UI_BAR_HEIGHT_FACTOR;
        let middle_height = self.display_height * UI_MAIN_HEIGHT_FACTOR;
        Canvas::new()
            .color(color::TRANSPARENT)
            .frame_color(color::TRANSPARENT)
            .flow_down(&[(SECTION_TOP,
                          Canvas::new()
                              .color(color::CHARCOAL)
                              .length(bar_height)),
                         (SECTION_MAIN,
                          Canvas::new()
                              .color(color::TRANSPARENT)
                              .frame_color(color::TRANSPARENT)
                              .length(middle_height)),
                         (SECTION_BOTTOM,
                          Canvas::new()
                              .color(color::CHARCOAL)
                              .length(bar_height))])
            .pad(0.0)
            .set(MASTER, ui);

        const TITLE_TEXT: &'static str = "SolarRust alpha-dev";
        Text::new(TITLE_TEXT)
            .color(color::WHITE)
            .middle_of(SECTION_TOP)
            .set(TITLE, ui);

        let object_home = self.object_home.borrow();
        let home_object_reg = object_home.register();
        ResourceWidget::from_logo(self.texture_icon_minerals.clone())
            .mid_left_of(SECTION_TOP)
            .frame(0.0)
            .color(color::TRANSPARENT)
            .with_amount(*home_object_reg.get_float("minerals")
                                         .expect("ResourceWidget : no minerals in target object"))
            .set(RESOURCES_MINERALS, ui);
        ResourceWidget::from_logo(self.texture_icon_energy.clone())
            .mid_right_of(SECTION_TOP)
            .frame(0.0)
            .color(color::TRANSPARENT)
            .with_amount(*home_object_reg.get_float("energy")
                                         .expect("ResourceWidget : no energy in target object"))
            .set(RESOURCES_ENERGY, ui);
    }

    pub fn render(&mut self, c: Context, g: &mut G2d) {
        // use graphics::rectangle::Rectangle;

        // TODO better aspect ratio handling
        let f = self.gameview_height / self.display_height;
        let c = Context::new_abs(self.gameview_width / f, self.gameview_height / f);
        let x = (self.display_width - self.gameview_width) / 2.0;
        let c = c.trans(x, self.display_height * UI_BAR_HEIGHT_FACTOR)
                 .trans(self.gameview_width / 2.0, self.gameview_height / 2.0);

        // Rectangle::new([1.0, 0.0, 0.0, 0.6])
        //     .draw([0.0, 0.0, self.gameview_width, self.gameview_height],
        //           &c.draw_state,
        //           c.transform,
        //           g); // debug background

        self.system.render(c, g);
    }
}

impl SolarRust<StdRng> {
    pub fn test_game(textures: TextureStore,
                     display_size: Size)
                     -> Result<SolarRust<StdRng>, String> {
        assert_eq!(2.0 * UI_BAR_HEIGHT_FACTOR + UI_MAIN_HEIGHT_FACTOR, 1.0);

        let mut rng = try!(StdRng::new().map_err(|e| format!("{:?}", e)));
        let (mut system, home) = try!(test_system(&mut rng));
        system.init(&mut rng);

        let texture_icon_minerals = textures.get("minerals_icon.png");
        let texture_icon_energy = textures.get("energy_icon.png");

        let (w, h) = (display_size.width as f64, display_size.height as f64);
        Ok(SolarRust {
            rng: rng,
            system: system,
            display_width: w,
            display_height: h,
            gameview_width: w * UI_MAIN_HEIGHT_FACTOR,
            gameview_height: h * UI_MAIN_HEIGHT_FACTOR,
            texture_store: textures,
            texture_icon_minerals: texture_icon_energy,
            texture_icon_energy: texture_icon_minerals,
            object_home: home,
        })
    }
}

fn test_system<R: 'static + Rng>(rng: &mut R) -> Result<(GameSystem, ObjectHandle), String> {
    use blueprints::TransfertStationBlueprint;

    let mut system = GameSystem::new("Test system");

    let sun = DefaultObjectBuilder::with_visuals(ObjectType::Star,
                                                 ObjectVisuals::circle(75.0, (255, 255, 0)))
                  .orbit(Orbit::Fixed((0.0, 0.0)))
                  .build();
    let planet1 = DefaultObjectBuilder::with_visuals(ObjectType::Planet,
                                                     ObjectVisuals::circle(40.0, (40, 15, 180)))
                      .orbit(Orbit::Circular {
                          altitude: 125.0,
                          orbital_speed: 0.1,
                          angle: 0f64.to_radians(),
                          origin: sun.clone(),
                      })
                      .build();
    let moon1 = DefaultObjectBuilder::with_visuals(ObjectType::Moon,
                                                   ObjectVisuals::circle(10.0, (200, 0, 150)))
                    .orbit(Orbit::Circular {
                        altitude: 50.0,
                        orbital_speed: 0.3,
                        angle: -90f64.to_radians(),
                        origin: planet1.clone(),
                    })
                    .build();

    let station1 = try!(TransfertStationBlueprint::new()
                            .orbit(Orbit::new_relative_orbit(60f64.to_radians(),
                                                             40.0,
                                                             planet1.clone()))
                            .produce(rng));
    station1.borrow_mut()
            .register_mut()
            .add_constant("name", ObjectPropertyValue::text("Station One"), "");

    let mut asteroid_belt_blueprint = AsteroidBeltBlueprint::<R>::new()
                                          .asteroids_number(35)
                                          .generator(AsteroidGenerator::default()
                                                         .radius(Distribution::Normal {
                                                                     mean: 10.0,
                                                                     std_dev: 5.0,
                                                                 }
                                                                 .into())
                                                         .altitude(Distribution::Normal {
                                                                       mean: 200.0,
                                                                       std_dev: 10.0,
                                                                   }
                                                                   .into())
                                                         .orbital_speed(Distribution::Normal {
                                                                            mean: 0.2,
                                                                            std_dev: 0.05,
                                                                        }
                                                                        .into()));
    let asteroid_belt = try!(asteroid_belt_blueprint.produce(rng));

    system.add_object(sun);
    system.add_object(planet1);
    system.add_object(moon1);
    system.add_object(station1.clone());
    system.add_object(asteroid_belt);

    Ok((system, station1))
}
