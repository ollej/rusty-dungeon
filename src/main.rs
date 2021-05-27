mod camera_view;
mod components;
mod macroquad_utils;
mod map;
mod map_builder;
mod spawner;
mod systems;
mod turn_state;

mod prelude {
    pub use legion::systems::CommandBuffer;
    pub use legion::world::SubWorld;
    pub use legion::*;
    pub use macroquad::prelude::*;
    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub const DISPLAY_WIDTH: i32 = SCREEN_WIDTH / 2;
    pub const DISPLAY_HEIGHT: i32 = SCREEN_HEIGHT / 2;
    pub use crate::camera_view::*;
    pub use crate::components::*;
    pub use crate::macroquad_utils::*;
    pub use crate::map::*;
    pub use crate::map_builder::*;
    pub use crate::spawner::*;
    pub use crate::systems::*;
    pub use crate::turn_state::*;
}

use prelude::*;

struct State {
    ecs: World,
    resources: Resources,
    input_systems: Schedule,
    player_systems: Schedule,
    monster_systems: Schedule,
}

impl State {
    fn new(texture: Texture2D) -> Self {
        let tileset = TileSet {
            texture: texture,
            tile_width: 32,
            tile_height: 32,
            columns: 16,
        };
        let mut ecs = World::default();
        let mut resources = Resources::default();
        let map_builder = MapBuilder::new();
        spawn_player(&mut ecs, map_builder.player_start);
        map_builder
            .rooms
            .iter()
            .skip(1)
            .map(|r| r.center())
            .for_each(|pos| spawn_monster(&mut ecs, pos));
        resources.insert(map_builder.map);
        resources.insert(CameraView::new(map_builder.player_start));
        resources.insert(tileset);
        resources.insert(TurnState::AwaitingInput);
        Self {
            ecs,
            resources,
            input_systems: build_input_scheduler(),
            player_systems: build_player_scheduler(),
            monster_systems: build_monster_scheduler(),
        }
    }

    fn tick(&mut self) {
        clear_background(BLACK);
        self.resources.insert(get_last_key_pressed());
        let current_state = self.resources.get::<TurnState>().unwrap().clone();
        match current_state {
            TurnState::AwaitingInput => self
                .input_systems
                .execute(&mut self.ecs, &mut self.resources),
            TurnState::PlayerTurn => self
                .player_systems
                .execute(&mut self.ecs, &mut self.resources),
            TurnState::MonsterTurn => self
                .monster_systems
                .execute(&mut self.ecs, &mut self.resources),
        }
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
