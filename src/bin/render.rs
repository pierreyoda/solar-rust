use std::collections::HashMap;

use piston_window::{Context, G2d};
use graphics::ellipse::Ellipse;

use solar_rustlib::core::object::{Object, ObjectVisuals};
use solar_rustlib::core::system::System;

pub type DrawFunction = Box<Fn(Context, &mut G2d, (f64, f64))>;

const CIRCLE_BORDER_COLOR: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
const CIRCLE_BORDER_RADIUS: f64 = 1.0;

/// Structure responsible for rendering a given System.
pub struct SystemRenderer {
    system_prev_size: i32,
    /// Cache a drawing closure for each object ID.
    /// Variables are to be pre-computed and captured by the closure whenever
    /// possible.
    cache: HashMap<String, DrawFunction>,
}

impl SystemRenderer {
    pub fn new() -> SystemRenderer {
        SystemRenderer {
            system_prev_size: -1,
            cache: HashMap::new(),
        }
    }

    /// Cache all the existing, yet uncached objects in the given 'System' in
    /// a rendering-friendly form and draw all the objects.
    /// This function assumes unicity among one 'System'.
    pub fn update_cache(&mut self, system: &System) {
        let objects = system.objects();
        // crude modification detection
        if self.system_prev_size != objects.len() as i32 {
            for (id, object) in objects {
                if !self.cache.contains_key(id) {
                    self.cache_object(id.clone(), &(*object.borrow()));
                }
            }
            self.system_prev_size = objects.len() as i32;
        }
    }

    pub fn render(&mut self, system: &System, c: Context, g: &mut G2d) {
        for (id, object) in system.objects() {
            let draw_fn = match self.cache.get(id) {
                Some(f) => f,
                None => continue,
            };
            draw_fn(c, g, object.borrow().position());
        }
    }

    pub fn cache_custom_object(&mut self, id: String, draw_fn: DrawFunction) {
        self.cache.insert(id, draw_fn);
    }

    fn cache_object(&mut self, id: String, object: &Object) {
        let f = match object.visuals() {
            &ObjectVisuals::Circle { radius, color } => {
                let ellipse_color = [color[0] as f32 / 255.0,
                                     color[1] as f32 / 255.0,
                                     color[2] as f32 / 255.0,
                                     color[3] as f32 / 255.0];
                let ellipse = Ellipse::new_border(CIRCLE_BORDER_COLOR, CIRCLE_BORDER_RADIUS)
                                  .color(ellipse_color);
                let half_radius = radius / 2.0;
                Box::new(move |c: Context, g: &mut G2d, p: (f64, f64)| {
                    let x = p.0 - half_radius;
                    let y = p.1 - half_radius;
                    ellipse.draw([x, y, radius, radius], &c.draw_state, c.transform, g);
                })
            }
            &ObjectVisuals::Custom => return,
        };
        self.cache.insert(id, f);

    }
}
