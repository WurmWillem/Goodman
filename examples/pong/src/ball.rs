use goodman::prelude::*;

use crate::{paddle::Paddle, WINDOW_SIZE};

const DIAMETER: f64 = 48.;

#[derive(Debug, Clone, Copy)]
pub struct Ball {
    pub pos: Vec64,
    pub vel: Vec64,
}
impl Ball {
    pub fn new() -> Self {
        Self {
            pos: vec2(WINDOW_SIZE.x * 0.5, WINDOW_SIZE.y * 0.5),
            vel: vec2(800., 800.),
        }
    }

    pub fn update(&mut self, delta_t: f64) {
        self.pos += self.vel * delta_t;

        if self.pos.x + DIAMETER > WINDOW_SIZE.x {
            let vel = self.vel;
            *self = Ball::new();
            self.vel = vel;
        } else if self.pos.x < 0. {
            let vel = self.vel;
            *self = Ball::new();
            self.vel = vel;
        }
        if self.pos.y + DIAMETER > WINDOW_SIZE.y {
            self.vel.y *= -1.;
            self.pos.y = WINDOW_SIZE.y - DIAMETER;
        } else if self.pos.y < 0. {
            self.vel.y *= -1.;
            self.pos.y = 0.;
        }
    }

    pub fn resolve_collisions_right_paddle(&mut self, paddle: &Paddle) {
        if self.pos.x < paddle.rect.x + paddle.rect.w
            && self.pos.x + DIAMETER > paddle.rect.x
            && self.pos.y + DIAMETER > paddle.rect.y
            && self.pos.y < paddle.rect.y + paddle.rect.h
        {
            self.pos.x = paddle.rect.x - DIAMETER;
            self.vel.x *= -1.;
        }
    }

    pub fn resolve_collisions_left_paddle(&mut self, paddle: &Paddle) {
        if self.pos.x + DIAMETER > paddle.rect.x
            && self.pos.x < paddle.rect.x + paddle.rect.w
            && self.pos.y + DIAMETER > paddle.rect.y
            && self.pos.y < paddle.rect.y + paddle.rect.h
        {
            self.pos.x = paddle.rect.x + DIAMETER;
            self.vel.x *= -1.;
        }
    }

    pub fn to_rect(self) -> Rect32 {
        rect64_vec(self.pos, vec2(DIAMETER, DIAMETER)).into()
    }
}
