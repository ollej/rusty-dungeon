use crate::prelude::*;

pub struct DungeonTheme {}

impl DungeonTheme {
    pub fn new() -> Box<dyn MapTheme> {
        Box::new(Self {})
    }
}

impl MapTheme for DungeonTheme {
    fn tile_to_render(&self, tile_type: TileType) -> Sprite {
        match tile_type {
            TileType::Floor => TileSet::SPRITE_FLOOR,
            TileType::Wall => TileSet::SPRITE_WALL,
        }
    }
}

pub struct ForestTheme {}

impl MapTheme for ForestTheme {
    fn tile_to_render(&self, tile_type: TileType) -> Sprite {
        match tile_type {
            TileType::Floor => TileSet::SPRITE_GROUND,
            TileType::Wall => TileSet::SPRITE_TREE,
        }
    }
}

impl ForestTheme {
    pub fn new() -> Box<dyn MapTheme> {
        Box::new(Self {})
    }
}
