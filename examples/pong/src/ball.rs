use goodman::prelude::*;

use crate::{Paddle, SCREEN_SIZE};

const RADIUS: f64 = 24.;

#[derive(Debug, Clone, Copy)]
pub struct Ball {
    pos: Vec2,
    vel: Vec2,
}
impl Ball {
    pub fn new() -> Self {
        Self {
            pos: vec2(350., 350.),
            vel: vec2(600., 600.),
        }
    }

    pub fn update(&mut self, frame_time: f64) {
        self.pos += self.vel * frame_time;

        if self.pos.x + RADIUS > SCREEN_SIZE.x {
            self.pos.x = SCREEN_SIZE.x - RADIUS;
            self.vel.x *= -1.;
        } else if self.pos.x - RADIUS < 0. {
            self.pos.x = RADIUS;
            self.vel.x *= -1.;
        }
        if self.pos.y + RADIUS > SCREEN_SIZE.y {
            self.vel.y *= -1.;
            self.pos.y = SCREEN_SIZE.y - RADIUS;
        } else if self.pos.y - RADIUS < 0. {
            self.vel.y *= -1.;
            self.pos.y = RADIUS;
        }
    }

    pub fn resolve_collisions(&mut self, paddle: &Paddle) {
        let paddle_size_x = paddle.rect.w * 0.5;
        let paddle_size_y = paddle.rect.h * 0.5;

        if self.pos.x < paddle.rect.x
            && self.pos.x > paddle.rect.x - paddle_size_x - RADIUS
            && self.pos.y > paddle.rect.y - paddle_size_y - RADIUS
            && self.pos.y < paddle.rect.y + paddle_size_y + RADIUS
        {
            self.pos.x = paddle.rect.x - paddle_size_x - RADIUS;
            self.vel.x *= -1.;
        } else if self.pos.x > paddle.rect.x
            && self.pos.x - RADIUS < paddle.rect.x + paddle_size_x
            && self.pos.y + RADIUS > paddle.rect.y - paddle_size_y
            && self.pos.y - RADIUS < paddle.rect.y + paddle_size_y
        {
            self.pos.x = paddle.rect.x + paddle_size_x + RADIUS;
            self.vel.x *= -1.;
        }
    }

    pub fn to_rect(self) -> Rect {
        rect_vec(self.pos, vec2(RADIUS * 2., RADIUS * 2.))
    }
}
