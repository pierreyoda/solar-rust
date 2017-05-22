use specs;

/// An artificial object, built by mankind (a mining station for instance).
pub struct ArtificialObjectComponent {
    pub name: String,
}

impl specs::Component for ArtificialObjectComponent {
    type Storage = specs::VecStorage<ArtificialObjectComponent>;
}