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
    screen_width() / DISPLAY_WIDTH as f32
}

pub fn tile_height() -> f32 {
    screen_height() / DISPLAY_HEIGHT as f32
}

pub fn tile_pos_x(x: i32) -> f32 {
    let w = tile_width();
    x as f32 * w + w / 2.
}

pub fn tile_pos_y(y: i32) -> f32 {
    let h = tile_height();
    y as f32 * h + h / 2.
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
