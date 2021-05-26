use crate::prelude::*;

#[system]
#[write_component(Point)]
#[read_component(Player)]
pub fn player_input(
    ecs: &mut SubWorld,
    #[resource] map: &Map,
    #[resource] key: &Option<KeyCode>,
    #[resource] camera: &mut CameraView,
) {
    if let Some(key) = key {
        let delta = match key {
            KeyCode::Left => Point::new(-1, 0),
            KeyCode::Right => Point::new(1, 0),
            KeyCode::Up => Point::new(0, -1),
            KeyCode::Down => Point::new(0, 1),
            _ => Point::zero(),
        };

        if delta.x != 0 || delta.y != 0 {
            let mut players = <&mut Point>::query().filter(component::<Player>());

            players.iter_mut(ecs).for_each(|pos| {
                let destination = *pos + delta;
                if map.can_enter_tile(destination) {
                    *pos = destination;
                    camera.on_player_move(destination);
                }
            });
        }
    }
}
