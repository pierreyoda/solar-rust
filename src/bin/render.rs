use piston_window::{types, Context, G2d};

use solar_rustlib::core::{ObjectVisuals, Color};
use objects::DefaultObjectDrawFn;


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
pub fn draw_fn_from_visuals(visuals: &ObjectVisuals) -> DefaultObjectDrawFn {
    use graphics::ellipse::Ellipse;
    use graphics::rectangle::Rectangle;

    match visuals {
        &ObjectVisuals::Circle { radius, color } => {
            let ellipse = Ellipse::new_border(CIRCLE_BORDER_COLOR, CIRCLE_BORDER_RADIUS)
                              .color(processed_color(color));
            let neg_half_radius = radius / -2.0;
            Box::new(move |c: Context, g: &mut G2d| {
                ellipse.draw([neg_half_radius, neg_half_radius, radius, radius],
                             &c.draw_state,
                             c.transform,
                             g);
            })
        }
        &ObjectVisuals::Square { size, color } => {
            let square = Rectangle::new_border(SQUARE_BORDER_COLOR, SQUARE_BORDER_RADIUS)
                             .color(processed_color(color));
            Box::new(move |c: Context, g: &mut G2d| {
                square.draw([0.0, 0.0, size, size], &c.draw_state, c.transform, g);
            })
        }
    }
}
