use goodman::prelude::*;

use crate::SCREEN_SIZE;

const SPEED: f64 = 1000.;
const SIZE: Vec2 = vec2(48., 144.);

#[derive(Debug, Clone, Copy)]
pub struct Paddle {
    pub rect: Rect,
}
impl Paddle {
    pub fn new(x: f64, y: f64) -> Self {
        Self {
            rect: rect(vec2(x, y), SIZE),
        }
    }

    pub fn update(&mut self, up_pressed: bool, down_pressed: bool, frame_time: f64) {
        if up_pressed && self.rect.y + self.rect.h * 0.5 < SCREEN_SIZE.y {
            self.rect.y += SPEED * frame_time;
        }
        if down_pressed && self.rect.y - self.rect.h * 0.5 > 0. {
            self.rect.y -= SPEED * frame_time;
        }
    }
}
