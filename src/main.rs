mod macroquad_utils;
mod map;
mod player;

mod prelude {
    pub use macroquad::prelude::*;
    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub use crate::macroquad_utils::*;
    pub use crate::map::*;
    pub use crate::player::*;
}

use prelude::*;

struct State {
    map: Map,
    player: Player,
}

impl State {
    fn new() -> Self {
        Self {
            map: Map::new(),
            player: Player::new(Point::new(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2)),
        }
    }

    fn tick(&mut self) {
        clear_background(BLACK);
        self.player.update(&self.map);
        self.map.render();
        self.player.render();
    }
}

#[macroquad::main("Rusty Dungeon")]
async fn main() {
    let mut game = State::new();
    loop {
        game.tick();
        next_frame().await
    }
}
