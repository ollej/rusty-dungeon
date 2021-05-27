use crate::prelude::*;

pub fn spawn_player(ecs: &mut World, pos: Point) {
    ecs.push((
        Player,
        pos,
        Render {
            color: WHITE,
            sprite: TileSet::SPRITE_PLAYER,
        },
    ));
}

pub fn spawn_monster(ecs: &mut World, pos: Point) {
    ecs.push((
        Enemy,
        pos,
        Render {
            color: WHITE,
            sprite: match rand::gen_range(0, 4) {
                0 => TileSet::SPRITE_ETTIN,
                1 => TileSet::SPRITE_OGRE,
                2 => TileSet::SPRITE_ORC,
                _ => TileSet::SPRITE_GOBLIN,
            },
        },
        MovingRandomly {},
    ));
}
