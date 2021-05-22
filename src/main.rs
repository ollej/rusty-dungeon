mod map;

mod prelude {
    pub use macroquad::prelude::*;
    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub use crate::map::*;
}

use prelude::*;

struct State {
    map: Map,
}

impl State {
    fn new() -> Self {
        Self { map: Map::new() }
    }

    fn tick(&mut self) {
        clear_background(BLACK);
        self.map.render();
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
