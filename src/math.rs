use cgmath::vec2;
use std::ops::{Div, DivAssign, Mul, MulAssign};

pub type Vec64 = cgmath::Vector2<f64>;
pub type Vec32 = cgmath::Vector2<f32>;

macro_rules! create_rect {
    ($r: ident, $vec: ty, $f: ty, $func: ident) => {
        #[derive(Debug, Copy, Clone)]
        pub struct $r {
            pub x: $f,
            pub y: $f,
            pub w: $f,
            pub h: $f,
        }

        impl $r {
            pub fn new(pos: $vec, size: $vec) -> Self {
                Self {
                    x: pos.x,
                    y: pos.y,
                    w: size.x,
                    h: size.y,
                }
            }
            /// Returns the center position of the `Rect`.
            pub fn center(&self) -> $vec {
                vec2(self.x + self.w * 0.5, self.y + self.h * 0.5)
            }
            /// Returns an intersection rect if any
            pub fn intersect(&self, other: $r) -> Option<$r> {
                let left = self.x.max(other.x);
                let top = self.y.max(other.y);
                let right = (self.x + self.w).min(other.x + other.w);
                let bottom = (self.y + self.h).min(other.y + other.h);

                if right < left || bottom < top {
                    return None;
                }

                Some($r {
                    x: left,
                    y: top,
                    w: right - left,
                    h: bottom - top,
                })
            }
            pub fn xy(&self) -> $vec {
                vec2(self.x, self.y)
            }
            pub fn wh(&self) -> $vec {
                vec2(self.w, self.h)
            }
            pub fn xy_add(&mut self, var: $vec) {
                self.x += var.x;
                self.y += var.y;
            }
        }
        impl Div<$f> for $r {
            #[inline]
            fn div(self, rhs: $f) -> $r {
                $func(self.x / rhs, self.y / rhs, self.w / rhs, self.h / rhs)
            }
            type Output = $r;
        }
        impl DivAssign<$f> for $r {
            #[inline]
            fn div_assign(&mut self, rhs: $f) {
                self.x.div_assign(rhs);
                self.y.div_assign(rhs);
                self.w.div_assign(rhs);
                self.h.div_assign(rhs);
            }
        }
        impl Mul<$f> for $r {
            #[inline]
            fn mul(self, rhs: $f) -> $r {
                $func(self.x * rhs, self.y * rhs, self.w * rhs, self.h * rhs)
            }
            type Output = $r;
        }
        impl MulAssign<$f> for $r {
            #[inline]
            fn mul_assign(&mut self, rhs: $f) {
                self.x.mul_assign(rhs);
                self.y.mul_assign(rhs);
                self.w.mul_assign(rhs);
                self.h.mul_assign(rhs);
            }
        }
    };
}

create_rect!(Rect64, Vec64, f64, rect64);
create_rect!(Rect32, Vec32, f32, rect32);

#[inline]
pub fn rect64(x: f64, y: f64, w: f64, h: f64) -> Rect64 {
    Rect64 { x, y, w, h }
}
#[inline]
pub fn rect64_vec(pos: Vec64, size: Vec64) -> Rect64 {
    Rect64 {
        x: pos.x,
        y: pos.y,
        w: size.x,
        h: size.y,
    }
}
impl From<Rect64> for Rect32 {
    fn from(val: Rect64) -> Self {
        rect32(val.x as f32, val.y as f32, val.w as f32, val.h as f32)
    }
}
impl From<Rect32> for Rect64 {
    fn from(r: Rect32) -> Self {
        rect64(r.x as f64, r.h as f64, r.w as f64, r.h as f64)
    }
}

#[inline]
pub fn rect32(x: f32, y: f32, w: f32, h: f32) -> Rect32 {
    Rect32 { x, y, w, h }
}
#[inline]
pub fn rect32_vec(pos: Vec32, size: Vec32) -> Rect32 {
    Rect32 {
        x: pos.x,
        y: pos.y,
        w: size.x,
        h: size.y,
    }
}
