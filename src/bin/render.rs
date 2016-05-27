use piston_window::{types, Context, G2d};

use solar_rustlib::core::{ObjectVisuals, ObjectRegister, Color};
use objects::ObjectDrawFunction;


const CIRCLE_BORDER_COLOR: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
const CIRCLE_BORDER_RADIUS: f64 = 1.0;
const SQUARE_BORDER_COLOR: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
const SQUARE_BORDER_RADIUS: f64 = 1.0;

fn processed_color(color: Color) -> types::Color {
    [color[0] as f32 / 255.0,
     color[1] as f32 / 255.0,
     color[2] as f32 / 255.0,
     color[3] as f32 / 255.0]
}

/// Build the function responsible for displaying the simple representation
/// described by the given 'ObjectVisuals' parameter.
pub fn draw_fn_from_visuals(visuals: &ObjectVisuals) -> ObjectDrawFunction {
    use graphics::ellipse::Ellipse;
    use graphics::rectangle::Rectangle;

    match visuals {
        &ObjectVisuals::Circle { radius, color } => {
            let ellipse = Ellipse::new_border(CIRCLE_BORDER_COLOR, CIRCLE_BORDER_RADIUS)
                              .color(processed_color(color));
            let half_radius = radius / 2.0;
            Box::new(move |c: Context, g: &mut G2d, pos: (f64, f64), _: &mut ObjectRegister| {
                let x = pos.0 - half_radius;
                let y = pos.1 - half_radius;
                ellipse.draw([x, y, radius, radius], &c.draw_state, c.transform, g);
            })
        }
        &ObjectVisuals::Square { size, color } => {
            let square = Rectangle::new_border(SQUARE_BORDER_COLOR, SQUARE_BORDER_RADIUS)
                             .color(processed_color(color));
            Box::new(move |c: Context, g: &mut G2d, pos: (f64, f64), _: &mut ObjectRegister| {
                square.draw([pos.0, pos.1, size, size], &c.draw_state, c.transform, g);
            })
        }
        &ObjectVisuals::Custom => {
            Box::new(|_: Context, _: &mut G2d, _: (f64, f64), _: &mut ObjectRegister| {})
        }
    }
}
