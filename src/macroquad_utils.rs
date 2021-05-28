use crate::prelude::*;
use macroquad::math::Rect;

pub fn tile_size() -> Vec2 {
    vec2(tile_width(), tile_height())
}

pub fn tile_width() -> f32 {
    screen_width() / DISPLAY_WIDTH as f32
}

pub fn tile_height() -> f32 {
    screen_height() / DISPLAY_HEIGHT as f32
}

pub fn tile_pos_x(x: i32) -> f32 {
    x as f32 * tile_width()
}

pub fn tile_pos_y(y: i32) -> f32 {
    y as f32 * tile_height()
}

pub fn mouse_tile_position() -> (i32, i32) {
    let pos = mouse_position();
    (
        (pos.0 / tile_width()) as i32,
        (pos.1 / tile_height()) as i32,
    )
}

pub type Sprite = u16;

#[derive(Debug)]
pub struct TileSet {
    pub texture: Texture2D,
    pub tile_width: i32,
    pub tile_height: i32,
    pub columns: u16,
}

impl TileSet {
    pub const SPRITE_PLAYER: Sprite = 64;
    pub const SPRITE_WALL: Sprite = 35;
    pub const SPRITE_FLOOR: Sprite = 46;
    pub const SPRITE_ETTIN: Sprite = 69;
    pub const SPRITE_OGRE: Sprite = 79;
    pub const SPRITE_ORC: Sprite = 111;
    pub const SPRITE_GOBLIN: Sprite = 103;

    pub fn sprite_rect(&self, ix: Sprite) -> Rect {
        let sw = self.tile_width as f32;
        let sh = self.tile_height as f32;
        let sx = (ix % self.columns) as f32 * sw as f32;
        let sy = (ix / self.columns) as f32 * sh as f32;

        Rect::new(sx, sy, sw, sh)
    }

    pub fn draw_tile(&self, sprite: Sprite, color: Color, x: i32, y: i32) {
        let spr_rect = self.sprite_rect(sprite);
        draw_texture_ex(
            self.texture,
            tile_pos_x(x),
            tile_pos_y(y),
            color,
            DrawTextureParams {
                dest_size: Some(tile_size()),
                source: Some(spr_rect),
                ..Default::default()
            },
        );
    }
}

pub fn print_centered<S>(line: usize, text: S)
where
    S: ToString,
{
    print_color_centered(line, WHITE, text);
}

pub fn print_color_centered<S>(line: usize, text_color: Color, text: S)
where
    S: ToString,
{
    let text_params = TextParams {
        color: text_color,
        font_size: tile_height() as u16,
        ..TextParams::default()
    };
    let dimensions = measure_text(
        &text.to_string(),
        Some(Font::default()),
        text_params.font_size,
        text_params.font_scale,
    );
    let x = screen_width() / 2. - dimensions.width / 2.;
    let fudge = (dimensions.height - dimensions.offset_y) / 2.;
    let y = tile_height() * line as f32 + fudge + dimensions.offset_y;
    draw_text_ex(&text.to_string(), x, y, text_params);
}

pub fn print_pos<S>(pos: Point, text: S)
where
    S: ToString,
{
    let text_params = TextParams {
        color: WHITE,
        font_size: tile_height() as u16,
        ..TextParams::default()
    };
    let x = tile_pos_x(pos.x);
    let y = tile_pos_y(pos.y);
    draw_text_ex(&text.to_string(), x, y, text_params);
}
pub fn bar_horizontal(
    pos: Point,
    width: i32,
    current: i32,
    max: i32,
    color: Color,
    background: Color,
) {
    let x = tile_pos_x(pos.x);
    let y = tile_pos_y(pos.y);
    let bar_width = tile_pos_x(width);
    let current_width = current as f32 / max as f32 * bar_width;
    draw_rectangle(x, y, bar_width, tile_height(), background);
    draw_rectangle(x, y, current_width, tile_height(), color);
}
