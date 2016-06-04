#![allow(dead_code)]

use std::any::Any;
use std::sync::Arc;

// use conrod::{Backend, Color, Colorable, FontSize, Frameable, FramedRectangle, IndexSlot,
//              Labelable, Positionable, Scalar, Text, Widget};
// use conrod::events::InputProvider;
// use conrod::backend::graphics::ImageSize;
use conrod::*;
use graphics::ImageSize;


/// A HUD widget displaying an amount of resource with the icon.
pub struct ResourceWidget<T> {
    common: CommonBuilder,
    /// Pointer to the icon image.
    icon: Arc<T>,
    /// Amount of resources to display.
    maybe_amount: Option<f64>,
    /// Unique styling for the ResourceWidget.
    pub style: Style,
}

/// Unique kind for the widget.
const KIND: WidgetKind = "SolarRust_ResourceWidget";

widget_style!{
    KIND;
    /// Unique styling for the ResourceWidget.
    style Style {
        /// Color of the ResourceWidget's background area.
        - background_color: Color { theme.shape_color }
        /// Width of the frame surrounding the ResourceWidget
        - frame: Scalar { theme.frame_width }
        /// The color of the frame.
        - frame_color: Color { theme.frame_color }
        /// The color of the ResourceWidget's label.
        - label_color: Color { theme.label_color }
        /// The font size of the ResourceWidget's label.
        - label_font_size: FontSize { theme.font_size_medium }
    }
}

/// Represents the state of the ResourceWidget.
#[derive(Clone, Debug, PartialEq)]
pub struct State {
    rectangle_idx: IndexSlot,
    label_idx: IndexSlot,
    image_idx: IndexSlot,
    image_rect: Rect,
}

#[allow(dead_code)]
impl<T> ResourceWidget<T> {
    /// Construct a new 'ResourceWidget' from a texture that will be used as a icon.
    pub fn from_logo(texture: Arc<T>) -> Self {
        ResourceWidget {
            common: CommonBuilder::new(),
            icon: texture,
            maybe_amount: None,
            style: Style::new(),
        }
    }

    builder_methods!{
        pub with_amount { maybe_amount = Some(f64) }
    }
}


impl<T> Widget for ResourceWidget<T>
    where T: Any + ImageSize
{
    type State = State;
    type Style = Style;

    fn common(&self) -> &CommonBuilder {
        &self.common
    }

    fn common_mut(&mut self) -> &mut CommonBuilder {
        &mut self.common
    }

    fn unique_kind(&self) -> WidgetKind {
        KIND
    }

    fn init_state(&self) -> State {
        State {
            rectangle_idx: IndexSlot::new(),
            label_idx: IndexSlot::new(),
            image_idx: IndexSlot::new(),
            image_rect: {
                let (w, h) = self.icon.get_size();
                Rect::from_xy_dim([0.0, 0.0], [w as f64, h as f64])
            },
        }
    }

    fn style(&self) -> Style {
        self.style.clone()
    }

    /// Update the state of the ResourceWidget.
    fn update<B: Backend>(self, args: UpdateArgs<Self, B>) {
        let UpdateArgs { idx, state, style, rect, mut ui, .. } = args;

        // FramedRectangle widget.
        let rectangle_idx = state.view().rectangle_idx.get(&mut ui);
        let dim = rect.dim();
        let frame = self.style.frame(ui.theme());
        let frame_color = self.style.frame_color(ui.theme());
        FramedRectangle::new(dim)
            .middle_of(idx)
            .graphics_for(idx)
            .color(self.style.background_color(ui.theme()))
            .frame(frame)
            .frame_color(frame_color)
            .set(rectangle_idx, &mut ui);

        // Icon image.
        let image_idx = state.view().image_idx.get(&mut ui);
        Image::from_texture(self.icon)
            .mid_left_of(rectangle_idx)
            .graphics_for(idx)
            .source_rectangle(state.view().image_rect)
            .set(image_idx, &mut ui);

        // Label widget.
        let label_idx = state.view().label_idx.get(&mut ui);
        let color = self.style.label_color(ui.theme());
        let font_size = self.style.label_font_size(ui.theme());
        let label = match self.maybe_amount {
            Some(amount) => format!("{:.0}", amount),
            None => "#NA!".into(),
        };
        Text::new(&label[..])
            .mid_right_of(rectangle_idx)
            .graphics_for(idx)
            .color(color)
            .font_size(font_size)
            .set(label_idx, &mut ui);
    }
}

impl<T> Colorable for ResourceWidget<T> {
    builder_method!{
        color { style.background_color = Some(Color) }
    }
}

impl<T> Frameable for ResourceWidget<T> {
    builder_methods!{
        frame { style.frame = Some(Scalar) }
        frame_color { style.frame_color = Some(Color) }
    }
}

impl<'a, T> Labelable<'a> for ResourceWidget<T> {
    #[allow(unused_mut)]
    fn label(mut self, _: &'a str) -> Self {
        self
    }

    builder_methods!{
        label_color { style.label_color = Some(Color) }
        label_font_size { style.label_font_size = Some(FontSize) }
    }
}
