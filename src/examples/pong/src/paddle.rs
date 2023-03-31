use goodman::prelude::*;

#[derive(Debug, Clone, Copy)]
pub struct Paddle {
    pub rect: Rect,
}
impl Paddle {
    const SPEED: f64 = 2.5;
    const SIZE: Vec2 = vec2(1., 3.);

    pub fn new(pos: Vec2) -> Self {
        Self {
            rect: rect(pos, Self::SIZE),
        }
    }

    pub fn update(&mut self, up_pressed: bool, down_pressed: bool, frame_time: f64) {
        let speed = Self::SPEED * frame_time;
        let size_scaled_y = self.rect.height * VERTEX_SCALE + speed;

        if up_pressed && self.rect.y + size_scaled_y < 1. {
            self.rect.y += speed;
        }
        if down_pressed && self.rect.y - size_scaled_y > -1. {
            self.rect.y -= speed;
        }
    }
}
