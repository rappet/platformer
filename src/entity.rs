/// Global and unique ID of an entity
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct EntityId(pub usize);

/// An entity that can be placed on the map
#[derive(Debug, Copy, Clone)]
pub struct Entity {
    pub entity_type: EntityType,
    pub x: f32,
    pub y: f32,
    pub dx: f32,
    pub dy: f32,
}

/// Type of the entity, used to determine logic
#[derive(Debug, Copy, Clone)]
pub enum EntityType {
    Player,
}

impl EntityType {
    /// Texture offset in the tileset
    pub const fn texture_offset(self) -> (u16, u16) {
        match self {
            Self::Player => (0, 7),
        }
    }
}
