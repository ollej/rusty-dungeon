use crate::prelude::*;

pub struct Player {
    pub position: Point,
}

impl Player {
    pub fn new(position: Point) -> Self {
        Self { position }
    }

    pub fn render(&self) {
        draw_circle(
            self.position.x_pos(),
            self.position.y_pos(),
            tile_width() / 2.,
            BLACK,
        );
    }

    pub fn update(&mut self, map: &Map) {
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
            }
        }
    }
}
