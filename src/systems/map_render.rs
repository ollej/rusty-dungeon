use crate::prelude::*;

#[system]
pub fn map_render(
    #[resource] map: &Map,
    #[resource] camera: &CameraView,
    #[resource] tileset: &TileSet,
) {
    for y in camera.top_y..=camera.bottom_y {
        for x in camera.left_x..camera.right_x {
            let pt = Point::new(x, y);
            if map.in_bounds(pt) {
                let idx = map_idx(x, y);
                let sprite: Sprite = match map.tiles[idx] {
                    TileType::Floor => TileSet::SPRITE_FLOOR,
                    TileType::Wall => TileSet::SPRITE_WALL,
                };
                tileset.draw_tile(sprite, WHITE, x - camera.left_x, y - camera.top_y);
            }
        }
    }
}
