use goodman::prelude::*;

use crate::Paddle;

#[derive(Debug, Clone, Copy)]
pub struct Ball {
    pos: Vec2,
    vel: Vec2,
}
impl Ball {
    const RADIUS: f64 = 1.;

    pub fn new() -> Self {
        Self {
            pos: vec2(0., 0.),
            vel: vec2(2., 2.),
        }
    }

    pub fn update(&mut self, frame_time: f64) {
        self.pos += self.vel * frame_time;
        let radius_scaled = Self::RADIUS * VERTEX_SCALE;

        if self.pos.x + radius_scaled > 1. {
            self.pos.x = 1. - radius_scaled;
            self.vel.x *= -1.;
        } else if self.pos.x - radius_scaled < -1. {
            self.pos.x = -1. + radius_scaled;
            self.vel.x *= -1.;
        }
        if self.pos.y + radius_scaled > 1. {
            self.vel.y *= -1.;
            self.pos.y = 1. - radius_scaled;
        } else if self.pos.y - radius_scaled < -1. {
            self.vel.y *= -1.;
            self.pos.y = -1. + radius_scaled;
        }
    }

    pub fn resolve_collisions(&mut self, paddle: &Paddle) {
        let paddle_size_x = paddle.rect.width * VERTEX_SCALE;
        let paddle_size_y = paddle.rect.height * VERTEX_SCALE;
        let radius_scaled = Self::RADIUS * VERTEX_SCALE;

        if self.pos.x < paddle.rect.x
            && self.pos.x > paddle.rect.x - paddle_size_x - radius_scaled
            && self.pos.y > paddle.rect.y - paddle_size_y - radius_scaled
            && self.pos.y < paddle.rect.y + paddle_size_y + radius_scaled
        {
            self.pos.x = paddle.rect.x - paddle_size_x - radius_scaled;
            self.vel.x *= -1.;
        } else if self.pos.x > paddle.rect.x
            && self.pos.x - radius_scaled < paddle.rect.x + paddle_size_x
            && self.pos.y + radius_scaled > paddle.rect.y - paddle_size_y
            && self.pos.y - radius_scaled < paddle.rect.y + paddle_size_y
        {
            self.pos.x = paddle.rect.x + paddle_size_x + radius_scaled;
            self.vel.x *= -1.;
        }
    }

    pub fn to_rect(&self) -> Rect {
        rect(self.pos, vec2(Ball::RADIUS, Ball::RADIUS))
    }
}
