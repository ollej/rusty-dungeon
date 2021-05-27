use crate::prelude::*;
use std::ops;

#[derive(Eq, PartialEq, Copy, Clone, Debug, Hash)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn zero() -> Self {
        Self { x: 0, y: 0 }
    }

    pub fn from_tuple(t: (f32, f32)) -> Self {
        Self {
            x: (t.0 / tile_width()) as i32,
            y: (t.1 / tile_height()) as i32,
        }
    }
}

impl ops::Add<Point> for Point {
    type Output = Point;
    fn add(mut self, rhs: Point) -> Point {
        self.x += rhs.x;
        self.y += rhs.y;
        self
    }
}

impl ops::Mul<i32> for Point {
    type Output = Point;
    fn mul(mut self, rhs: i32) -> Point {
        self.x *= rhs;
        self.y += rhs;
        self
    }
}

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

pub trait Bracket {
    fn center(&self) -> Point;
    fn for_each<F>(&self, f: F)
    where
        F: FnMut(Point);
}

impl Bracket for Rect {
    fn center(&self) -> Point {
        Point::new((self.x + self.w / 2.) as i32, (self.y + self.h / 2.) as i32)
    }

    fn for_each<F>(&self, mut f: F)
    where
        F: FnMut(Point),
    {
        for y in self.y as i32..=self.bottom() as i32 {
            for x in self.x as i32..=self.right() as i32 {
                f(Point::new(x, y));
            }
        }
    }
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
    print_color_centered(line, text, WHITE);
}

pub fn print_color_centered<S>(line: usize, text: S, text_color: Color)
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
