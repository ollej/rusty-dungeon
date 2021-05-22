mod camera_view;
mod macroquad_utils;
mod map;
mod map_builder;
mod player;

mod prelude {
    pub use macroquad::prelude::*;
    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub const DISPLAY_WIDTH: i32 = SCREEN_WIDTH / 2;
    pub const DISPLAY_HEIGHT: i32 = SCREEN_HEIGHT / 2;
    pub use crate::camera_view::*;
    pub use crate::macroquad_utils::*;
    pub use crate::map::*;
    pub use crate::map_builder::*;
    pub use crate::player::*;
}

use prelude::*;

struct State {
    map: Map,
    player: Player,
    camera: CameraView,
    tileset: TileSet,
}

impl State {
    fn new(texture: Texture2D) -> Self {
        let map_builder = MapBuilder::new();
        let tileset = TileSet {
            texture: texture,
            tile_width: 32,
            tile_height: 32,
            columns: 16,
        };
        Self {
            map: map_builder.map,
            player: Player::new(map_builder.player_start),
            camera: CameraView::new(map_builder.player_start),
            tileset,
        }
    }

    fn tick(&mut self) {
        clear_background(BLACK);
        self.player.update(&self.map, &mut self.camera);
        self.map.render(&self.camera, &self.tileset);
        self.player.render(&self.camera, &self.tileset);
    }
}

#[macroquad::main("Rusty Dungeon")]
async fn main() {
    let tileset = load_texture("assets/dungeonfont.png")
        .await
        .expect("Tile texture not found");
    tileset.set_filter(FilterMode::Nearest);

    let mut game = State::new(tileset);
    loop {
        game.tick();
        next_frame().await
    }
}
