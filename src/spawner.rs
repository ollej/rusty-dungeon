use crate::prelude::*;

pub fn spawn_player(ecs: &mut World, pos: Point) {
    ecs.push((
        Player,
        pos,
        Render {
            color: WHITE,
            sprite: TileSet::SPRITE_PLAYER,
        },
        Health {
            current: 10,
            max: 10,
        },
        FieldOfView::new(8),
    ));
}

pub fn spawn_monster(ecs: &mut World, pos: Point) {
    let (hp, name, sprite) = match rand::gen_range(1, 10) {
        1..=8 => goblin(),
        _ => orc(),
    };
    ecs.push((
        Enemy,
        pos,
        Render {
            color: WHITE,
            sprite: sprite,
        },
        ChasingPlayer {},
        Health {
            current: hp,
            max: hp,
        },
        Name(name),
        FieldOfView::new(6),
    ));
}

fn goblin() -> (i32, String, Sprite) {
    (1, "Goblin".to_string(), TileSet::SPRITE_GOBLIN)
}

fn orc() -> (i32, String, Sprite) {
    (2, "Orc".to_string(), TileSet::SPRITE_ORC)
}

pub fn spawn_amulet_of_yala(ecs: &mut World, pos: Point) {
    ecs.push((
        Item,
        AmuletOfYala,
        pos,
        Render {
            color: WHITE,
            sprite: TileSet::SPRITE_AMULET,
        },
        Name("Amulet of Yala".to_string()),
    ));
}
