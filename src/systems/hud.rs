use crate::prelude::*;

#[system]
#[read_component(Health)]
#[read_component(Player)]
pub fn hud(ecs: &SubWorld) {
    let mut health_query = <&Health>::query().filter(component::<Player>());
    let player_health = health_query.iter(ecs).nth(0).unwrap();

    print_centered(1, "Explore the Dungeon. Cursor keys to move.");
    bar_horizontal(
        Point::zero(),
        DISPLAY_WIDTH,
        player_health.current,
        player_health.max,
        RED,
        BLACK,
    );
    print_color_centered(
        0,
        format!("Health: {} / {}", player_health.current, player_health.max),
        WHITE,
    );
}
