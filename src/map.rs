use crate::prelude::*;

const NUM_TILES: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;

#[derive(Copy, Clone, PartialEq)]
pub enum TileType {
    Wall,
    Floor,
}

pub fn map_idx(x: i32, y: i32) -> usize {
    ((y * SCREEN_WIDTH) + x) as usize
}

pub struct Map {
    pub tiles: Vec<TileType>,
}

impl Map {
    const SPRITE_WALL: u32 = 35;
    const SPRITE_FLOOR: u32 = 46;

    pub fn new() -> Self {
        Self {
            tiles: vec![TileType::Floor; NUM_TILES],
        }
    }

    pub fn in_bounds(&self, point: Point) -> bool {
        point.x >= 0 && point.x < SCREEN_WIDTH && point.y >= 0 && point.y < SCREEN_HEIGHT
    }

    pub fn can_enter_tile(&self, point: Point) -> bool {
        self.in_bounds(point) && self.tiles[map_idx(point.x, point.y)] == TileType::Floor
    }

    pub fn render(&self, camera: &CameraView, tileset: &TileSet) {
        for y in camera.top_y..camera.bottom_y {
            for x in camera.left_x..camera.right_x {
                if self.in_bounds(Point::new(x, y)) {
                    let idx = map_idx(x, y);
                    match self.tiles[idx] {
                        TileType::Floor => {
                            tileset.draw_tile(
                                Self::SPRITE_FLOOR,
                                tile_pos_x(x - camera.left_x),
                                tile_pos_y(y - camera.top_y),
                            );
                        }
                        TileType::Wall => {
                            tileset.draw_tile(
                                Self::SPRITE_WALL,
                                tile_pos_x(x - camera.left_x),
                                tile_pos_y(y - camera.top_y),
                            );
                        }
                    }
                }
            }
        }
    }

    pub fn try_idx(&self, point: Point) -> Option<usize> {
        if !self.in_bounds(point) {
            None
        } else {
            Some(map_idx(point.x, point.y))
        }
    }
}
