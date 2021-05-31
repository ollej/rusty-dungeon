mod camera_view;
mod components;
mod macroquad_utils;
mod map;
mod map_builder;
mod spawner;
mod systems;
mod turn_state;

mod prelude {
    pub use bracket_pathfinding::prelude::*;
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
use std::time::SystemTime;

struct State {
    ecs: World,
    resources: Resources,
    input_systems: Schedule,
    player_systems: Schedule,
    monster_systems: Schedule,
    texture: Texture2D,
}

impl State {
    fn new(texture: Texture2D) -> Self {
        rand::srand(
            SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        );
        let mut ecs = World::default();
        let mut resources = Resources::default();
        let map_builder = MapBuilder::new();
        let tileset = Self::tileset(texture);
        spawn_player(&mut ecs, map_builder.player_start);
        spawn_amulet_of_yala(&mut ecs, map_builder.amulet_start);
        map_builder
            .monster_spawns
            .iter()
            .for_each(|pos| spawn_monster(&mut ecs, *pos));
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
            texture,
        }
    }

    fn tileset(texture: Texture2D) -> TileSet {
        TileSet {
            texture: texture,
            tile_width: 32,
            tile_height: 32,
            columns: 16,
        }
    }

    fn tick(&mut self) {
        clear_background(BLACK);
        self.resources.insert(get_last_key_pressed());
        self.resources
            .insert(Point::from_tuple(mouse_tile_position()));
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
            TurnState::GameOver => self.game_over(),
            TurnState::Victory => self.victory(),
        }
    }

    fn game_over(&mut self) {
        print_color_centered(2, RED, "Your quest has ended.");
        print_color_centered(
            4,
            WHITE,
            "Slain by a monster, your hero's journey has come to a \
            premature end.",
        );
        print_color_centered(
            5,
            WHITE,
            "The Amulet of YALA remains unclaimed, and your home town \
            is not saved.",
        );
        print_color_centered(
            8,
            YELLOW,
            "Don't worry, you can always try again with a new hero.",
        );
        print_color_centered(9, GREEN, "Press 1 to play again.");

        if is_key_down(KeyCode::Key1) {
            self.reset_game_state();
        }
    }

    fn victory(&mut self) {
        print_color_centered(2, GREEN, "You have won!");
        print_color_centered(
            4,
            WHITE,
            "You put on the Amulet of YALA and feel its power course through \
            your veins.",
        );
        print_color_centered(
            5,
            WHITE,
            "Your town is saved, and you can return to your normal life.",
        );
        print_color_centered(7, GREEN, "Press 1 to play again.");

        if is_key_down(KeyCode::Key1) {
            self.reset_game_state();
        }
    }

    fn reset_game_state(&mut self) {
        self.ecs = World::default();
        self.resources = Resources::default();
        let map_builder = MapBuilder::new();
        let tileset = Self::tileset(self.texture);
        spawn_player(&mut self.ecs, map_builder.player_start);
        spawn_amulet_of_yala(&mut self.ecs, map_builder.amulet_start);
        map_builder
            .monster_spawns
            .iter()
            .for_each(|pos| spawn_monster(&mut self.ecs, *pos));
        self.resources.insert(map_builder.map);
        self.resources
            .insert(CameraView::new(map_builder.player_start));
        self.resources.insert(tileset);
        self.resources.insert(TurnState::AwaitingInput);
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
