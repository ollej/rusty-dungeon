use crate::prelude::*;

#[system]
#[read_component(FieldOfView)]
#[read_component(Player)]
pub fn map_render(
    ecs: &SubWorld,
    #[resource] map: &Map,
    #[resource] camera: &CameraView,
    #[resource] tileset: &TileSet,
) {
    let mut fov = <&FieldOfView>::query().filter(component::<Player>());
    let player_fov = fov.iter(ecs).nth(0).unwrap();

    for y in camera.top_y..=camera.bottom_y {
        for x in camera.left_x..camera.right_x {
            let pt = Point::new(x, y);
            let offset = Point::new(camera.left_x, camera.top_y);
            let idx = map_idx(x, y);
            if map.in_bounds(pt)
                && (player_fov.visible_tiles.contains(&pt) | map.revealed_tiles[idx])
            {
                let tint = if player_fov.visible_tiles.contains(&pt) {
                    WHITE
                } else {
                    DARKGRAY
                };
                let sprite: Sprite = match map.tiles[idx] {
                    TileType::Floor => TileSet::SPRITE_FLOOR,
                    TileType::Wall => TileSet::SPRITE_WALL,
                };
                let pos = pt - offset;
                tileset.draw_tile(sprite, tint, pos.x, pos.y);
            }
        }
    }
}
