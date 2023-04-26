use goodman::prelude::*;

use crate::WINDOW_SIZE;

const SPEED: f64 = 1000.;
const SIZE: Vec2 = vec2(32., 192.);

#[derive(Debug, Clone, Copy)]
pub struct Paddle {
    pub rect: Rect,
}
impl Paddle {
    pub fn new(x: f64, y: f64) -> Self {
        Self {
            rect: rect_vec(vec2(x, y), SIZE),
        }
    }

    pub fn update(&mut self, up_pressed: bool, down_pressed: bool, delta_t: f64) {
        if up_pressed && self.rect.y > 0. {
            self.rect.y -= SPEED * delta_t;
        }
        if down_pressed && self.rect.y + self.rect.h < WINDOW_SIZE.y {
            self.rect.y += SPEED * delta_t;
        }
    }
}
