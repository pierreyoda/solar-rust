use piston_window::{Context, G2d};
use graphics::ellipse::Ellipse;

use solar_rustlib::core::{ObjectVisuals, ObjectRegister};
use objects::ObjectDrawFunction;


const CIRCLE_BORDER_COLOR: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
const CIRCLE_BORDER_RADIUS: f64 = 1.0;

/// Build the function responsible for displaying the simple representation
/// described by the given 'ObjectVisuals' parameter.
pub fn draw_fn_from_visuals(visuals: &ObjectVisuals) -> ObjectDrawFunction {
    match visuals {
        &ObjectVisuals::Circle { radius, color } => {
            let ellipse_color = [color[0] as f32 / 255.0,
                                 color[1] as f32 / 255.0,
                                 color[2] as f32 / 255.0,
                                 color[3] as f32 / 255.0];
            let ellipse = Ellipse::new_border(CIRCLE_BORDER_COLOR, CIRCLE_BORDER_RADIUS)
                              .color(ellipse_color);
            let half_radius = radius / 2.0;
            Box::new(move |c: Context, g: &mut G2d, pos: (f64, f64), _: &mut ObjectRegister| {
                let x = pos.0 - half_radius;
                let y = pos.1 - half_radius;
                ellipse.draw([x, y, radius, radius], &c.draw_state, c.transform, g);
            })
        }
        &ObjectVisuals::Custom => {
            Box::new(|_: Context, _: &mut G2d, _: (f64, f64), _: &mut ObjectRegister| {})
        }
    }
}
