#[derive(Debug, Copy, Clone)]
pub enum TileType {
    Grass,
    Dirt,
    Stone,
}

impl TileType {
    /// Texture offset in the tileset
    pub const fn texture_offset(self) -> (usize, usize) {
        match self {
            Self::Grass => (0, 0),
            Self::Dirt => (1, 0),
            Self::Stone => (2, 0),
        }
    }

    /// Parse from a char for a human readable tilemap
    pub const fn from_char(c: char) -> Option<Self> {
        Some(match c {
            'g' => Self::Grass,
            'd' => Self::Dirt,
            's' => Self::Stone,
            _ => return None,
        })
    }
}
