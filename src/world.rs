use crate::entity::{Entity, EntityId};
use crate::tile::TileType;
use std::collections::HashMap;

/// Width of a tile texture in pixels
pub const TILE_WIDTH: usize = 16;
/// height of a tile texture in pixels
pub const TILE_HEIGHT: usize = 16;
/// Demo world formatted as a string
pub const TEST_WORLD_STR: &str = include_str!("world.txt");

/// Tilemap of the world
pub struct World {
    pub tiles: HashMap<(i32, i32), TileType>,
    pub entities: HashMap<EntityId, Entity>,
    pub last_entity_id: EntityId,
}

impl World {
    /// Parse a human readable tilemap
    pub fn from_string_tilemap(tilemap: &str) -> Self {
        let mut tiles = HashMap::new();
        tilemap.lines().enumerate().for_each(|(y, line)| {
            line.chars().enumerate().for_each(|(x, c)| {
                if c != ' ' {
                    let tile_type = TileType::from_char(c).expect("char not a known tile type");
                    tiles.insert(
                        (
                            x.try_into().expect("Should not overflow"),
                            y.try_into().expect("Should not overflow"),
                        ),
                        tile_type,
                    );
                }
            });
        });
        Self {
            tiles,
            entities: HashMap::new(),
            last_entity_id: EntityId(0),
        }
    }

    pub fn spawn_entity(&mut self, entity: Entity) -> EntityId {
        let id = EntityId(
            self.last_entity_id
                .0
                .checked_add(1)
                .expect("EntityIDs should not overflow"),
        );
        self.last_entity_id = id;

        self.entities.insert(id, entity);
        id
    }

    pub fn collide_rect(&self, x: f32, y: f32, w: f32, h: f32) -> bool {
        self.collide_point(x, y)
            || self.collide_point(x + w, y)
            || self.collide_point(x, y + h)
            || self.collide_point(x + w, y + h)
    }

    pub fn collide_point(&self, x: f32, y: f32) -> bool {
        #[allow(clippy::cast_possible_truncation)]
        self.tiles
            .get(&(x.floor() as i32, y.floor() as i32))
            .is_some()
    }
}
