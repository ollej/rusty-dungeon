use crate::prelude::*;

pub struct Player {
    pub position: Point,
}

impl Player {
    pub fn new(position: Point) -> Self {
        Self { position }
    }

    pub fn render(&self, camera: &CameraView) {
        draw_circle(
            tile_pos_x(self.position.x - camera.left_x),
            tile_pos_y(self.position.y - camera.top_y),
            tile_width() / 2.,
            BLACK,
        );
    }

    pub fn update(&mut self, map: &Map, camera: &mut CameraView) {
        if let Some(key) = get_last_key_pressed() {
            let delta = match key {
                KeyCode::Left => Point::new(-1, 0),
                KeyCode::Right => Point::new(1, 0),
                KeyCode::Up => Point::new(0, -1),
                KeyCode::Down => Point::new(0, 1),
                _ => Point::zero(),
            };

            let new_position = self.position + delta;
            if map.can_enter_tile(new_position) {
                self.position = new_position;
                camera.on_player_move(new_position);
            }
        }
    }
}
