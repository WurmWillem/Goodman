use goodman::prelude::*;

use crate::{Paddle, SCREEN_SIZE};

const DIAMETER: f64 = 48.;

#[derive(Debug, Clone, Copy)]
pub struct Ball {
    pos: Vec2,
    vel: Vec2,
}
impl Ball {
    pub fn new() -> Self {
        Self {
            pos: vec2(SCREEN_SIZE.x * 0.5, SCREEN_SIZE.y * 0.5),
            vel: vec2(600., 0.),
        }
    }

    pub fn update(&mut self, frame_time: f64) {
        self.pos += self.vel * frame_time;

        if self.pos.x + DIAMETER > SCREEN_SIZE.x {
            self.pos.x = SCREEN_SIZE.x - DIAMETER;
            self.vel.x *= -1.;
        } else if self.pos.x < 0. {
            self.pos.x = 0.;
            self.vel.x *= -1.;
        }
        if self.pos.y + DIAMETER > SCREEN_SIZE.y {
            self.vel.y *= -1.;
            self.pos.y = SCREEN_SIZE.y - DIAMETER;
        } else if self.pos.y < 0. {
            self.vel.y *= -1.;
            self.pos.y = 0.;
        }
    }

    pub fn resolve_collisions(&mut self, paddle: &Paddle) {
        let paddle_size_x = paddle.rect.w * 0.5;
        //let paddle_size_y = paddle.rect.h * 0.5;

        if
        //self.pos.x < paddle.rect.x + paddle.rect.w
        self.pos.x + DIAMETER > paddle.rect.x
            && self.pos.y + DIAMETER > paddle.rect.y
            && self.pos.y < paddle.rect.y - paddle.rect.h
        {
            self.pos.x = paddle.rect.x - paddle_size_x - DIAMETER;
            self.vel.x *= -1.;
        } /*else if self.pos.x > paddle.rect.x
              && self.pos.x - DIAMETER < paddle.rect.x + paddle_size_x
              && self.pos.y + DIAMETER > paddle.rect.y - paddle_size_y
              && self.pos.y - DIAMETER < paddle.rect.y + paddle_size_y
          {
              self.pos.x = paddle.rect.x + paddle_size_x + DIAMETER;
              self.vel.x *= -1.;
          }*/
    }

    pub fn to_rect(self) -> Rect {
        rect_vec(self.pos, vec2(DIAMETER, DIAMETER))
    }
}
