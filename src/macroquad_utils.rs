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

    pub fn x_pos(&self) -> f32 {
        let tile_w = tile_width();
        self.x as f32 * tile_w + tile_w / 2.
    }

    pub fn y_pos(&self) -> f32 {
        let tile_h = tile_height();
        self.y as f32 * tile_h + tile_h / 2.
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

pub fn tile_size() -> Vec2 {
    vec2(tile_width(), tile_height())
}

pub fn tile_width() -> f32 {
    screen_width() / SCREEN_WIDTH as f32
}

pub fn tile_height() -> f32 {
    screen_height() / SCREEN_HEIGHT as f32
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
