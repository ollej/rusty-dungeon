use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Render)]
pub fn entity_render(
    ecs: &SubWorld,
    #[resource] camera: &CameraView,
    #[resource] tileset: &TileSet,
) {
    <(&Point, &Render)>::query()
        .iter(ecs)
        .for_each(|(pos, render)| {
            let x = pos.x - camera.left_x;
            let y = pos.y - camera.top_y;
            tileset.draw_tile(render.sprite, render.color, x, y);
        });
}
