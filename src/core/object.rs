use specs;

/// A color encoded as normalized RGBA channels.
pub type Color = [f32; 4];

#[derive(Clone, Debug)]
pub struct DrawCircleComponent {
    pub color: Color,
    pub radius: f32,
}

impl specs::Component for DrawCircleComponent {
    type Storage = specs::VecStorage<DrawCircleComponent>;
}

#[derive(Clone, Debug)]
pub struct DrawRectangleComponent {
    pub color: Color,
    pub width: f32,
    pub height: f32,
}

impl specs::Component for DrawRectangleComponent {
    type Storage = specs::VecStorage<DrawRectangleComponent>;
}

/// An artificial object, built by mankind (a mining station for instance).
pub struct ArtificialObjectComponent {
    pub name: String,
}

impl specs::Component for ArtificialObjectComponent {
    type Storage = specs::VecStorage<ArtificialObjectComponent>;
}