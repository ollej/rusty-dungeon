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

pub fn tile_size() -> Vec2 {
    vec2(
        screen_width() / SCREEN_WIDTH as f32,
        screen_height() / SCREEN_HEIGHT as f32,
    )
}

pub struct Map {
    pub tiles: Vec<TileType>,
}

impl Map {
    pub fn new() -> Self {
        Self {
            tiles: vec![TileType::Floor; NUM_TILES],
        }
    }

    pub fn render(&self) {
        for y in 0..SCREEN_HEIGHT {
            for x in 0..SCREEN_WIDTH {
                let idx = map_idx(x, y);
                match self.tiles[idx] {
                    TileType::Floor => {
                        self.render_tile(x, y, YELLOW);
                    }
                    TileType::Wall => {
                        self.render_tile(x, y, GREEN);
                    }
                }
            }
        }
    }

    pub fn render_tile(&self, x: i32, y: i32, color: Color) {
        let tile_size = tile_size();
        draw_rectangle(
            tile_size.x * x as f32,
            tile_size.y * y as f32,
            tile_size.x,
            tile_size.y,
            color,
        );
    }
}
