use na;
use specs;
use super::Iso2;

#[derive(Clone, Debug)]
pub struct SpatialComponent {
    /// The 2D transformation (translation & rotation) relative to the
    /// entity's parent.
    local_transform: Iso2,
    /// The entity's transformation is defined relative to this entity if not None.
    parent_entity: Option<specs::Entity>,
}

impl specs::Component for SpatialComponent {
    type Storage = specs::VecStorage<SpatialComponent>;
}

impl SpatialComponent {
    pub fn new(transform: Iso2, parent: specs::Entity) -> SpatialComponent {
        SpatialComponent {
            local_transform: transform,
            parent_entity: Some(parent),
        }
    }

    pub fn new_root(transform: Iso2) -> SpatialComponent {
        SpatialComponent {
            local_transform: Iso2::one(),
            parent_entity: None,
        }
    }
}
