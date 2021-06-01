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

pub fn spawn_entity(ecs: &mut World, pos: Point) {
    let roll = rand::gen_range(1, 6);
    match roll {
        1 => spawn_healing_potion(ecs, pos),
        2 => spawn_magic_mapper(ecs, pos),
        _ => spawn_monster(ecs, pos),
    }
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

pub fn spawn_healing_potion(ecs: &mut World, pos: Point) {
    ecs.push((
        Item,
        pos,
        Render {
            color: WHITE,
            sprite: TileSet::SPRITE_POTION,
        },
        Name("Healing Potion".to_string()),
        ProvidesHealing { amount: 6 },
    ));
}

pub fn spawn_magic_mapper(ecs: &mut World, pos: Point) {
    ecs.push((
        Item,
        pos,
        Render {
            color: WHITE,
            sprite: TileSet::SPRITE_SCROLL,
        },
        Name("Dungeon Map".to_string()),
        ProvidesDungeonMap {},
    ));
}
