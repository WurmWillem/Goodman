use crate::engine_manager::Vec2;
use cgmath::vec2;
use std::ops::{Add, Div, DivAssign, Mul, Sub};

#[derive(Debug, Clone, Copy)]
pub struct Rect {
    pub x: f64,
    pub y: f64,
    pub w: f64,
    pub h: f64,
}
impl Rect {
    pub fn new(pos: Vec2, size: Vec2) -> Self {
        Self {
            x: pos.x,
            y: pos.y,
            w: size.x,
            h: size.y,
        }
    }

    /// Returns the center position of the `Rect`.
    pub fn center(&self) -> Vec2 {
        vec2(self.x + self.w * 0.5, self.y + self.h * 0.5)
    }

    /// Returns an intersection rect if any
    pub fn intersect(&self, other: Rect) -> Option<Rect> {
        let left = self.x.max(other.x);
        let top = self.y.max(other.y);
        let right = (self.x + self.w).min(other.x + other.w);
        let bottom = (self.y + self.h).min(other.y + other.h);

        if right < left || bottom < top {
            return None;
        }

        Some(Rect {
            x: left,
            y: top,
            w: right - left,
            h: bottom - top,
        })
    }
}
impl Div<f64> for Rect {
    #[inline]
    fn div(self, rhs: f64) -> Rect {
        rect(
            vec2(self.x / rhs, self.y / rhs),
            vec2(self.w / rhs, self.h / rhs),
        )
    }
    type Output = Rect;
}
impl DivAssign<f64> for Rect {
    #[inline]
    fn div_assign(&mut self, rhs: f64) {
        self.x.div_assign(rhs);
        self.y.div_assign(rhs);
        self.w.div_assign(rhs);
        self.h.div_assign(rhs);
    }
}
impl Mul<f64> for Rect {
    #[inline]
    fn mul(self, rhs: f64) -> Rect {
        rect(
            vec2(self.x * rhs, self.y * rhs),
            vec2(self.w * rhs, self.h * rhs),
        )
    }
    type Output = Rect;
}
impl Sub<f64> for Rect {
    #[inline]
    fn sub(self, rhs: f64) -> Rect {
        rect(
            vec2(self.x - rhs, self.y - rhs),
            vec2(self.w - rhs, self.h - rhs),
        )
    }
    type Output = Rect;
}
impl Add<f64> for Rect {
    #[inline]
    fn add(self, rhs: f64) -> Rect {
        rect(
            vec2(self.x + rhs, self.y + rhs),
            vec2(self.w + rhs, self.h + rhs),
        )
    }
    type Output = Rect;
}

#[inline]
pub fn rect(pos: Vec2, size: Vec2) -> Rect {
    Rect {
        x: pos.x,
        y: pos.y,
        w: size.x,
        h: size.y,
    }
}
