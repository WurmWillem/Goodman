use goodman::prelude::*;

fn main() {
    block_on(run());
}

const WINDOW_SIZE: Vec2 = vec2(1200., 900.);

async fn run() {
    let event_loop = EventLoop::new();
    let mut engine: Engine = Engine::new(WINDOW_SIZE, &event_loop, false).await;

    engine.set_target_fps(Some(144));

    let paddle_bytes = include_bytes!("assets/paddle.png");
    let paddle_tex = engine.create_texture(paddle_bytes, "paddle").unwrap();
    let ball_bytes = include_bytes!("assets/ball.png");
    let ball_tex = engine.create_texture(ball_bytes, "ball").unwrap();
    let block_bytes = include_bytes!("assets/block.png");
    let block_tex = engine.create_texture(block_bytes, "block").unwrap();

    let breakout = Breakout::new(vec![paddle_tex, ball_tex, block_tex]);

    engine.enter_loop(breakout, event_loop);
}

struct Breakout {
    ball: Ball,
    paddle: Paddle,
    blocks: Vec<Vec<Block>>,
    textures: Vec<Texture>,
}
impl Breakout {
    fn new(textures: Vec<Texture>) -> Self {
        let paddle = Paddle::new(vec2(WINDOW_SIZE.x * 0.5, WINDOW_SIZE.y * 0.9));
        let ball = Ball::new(vec2(0., WINDOW_SIZE.y));

        let mut blocks = Vec::new();
        for j in 0..100 {
            let mut row = Vec::new();
            for i in 0..100 {
                let block = Block::new(i as f64 * 12., j as f64 * 9.);
                row.push(block);
            }
            blocks.push(row);
        }

        Self {
            ball,
            paddle,
            blocks,
            textures,
        }
    }
}
impl Manager for Breakout {
    fn new(_engine: &mut Engine) -> Self {
        todo!()
    }
    fn update(&mut self, delta_t: f64, input: &Input) {
        self.paddle.update(input, delta_t);
        self.ball.update(delta_t);

        self.ball.resolve_paddle_collision(&self.paddle);

        self.blocks.iter_mut().for_each(|row| {
            row.iter_mut().for_each(|mut block| {
                if resolve_collision(&mut self.ball.to_rect(), &mut self.ball.vel, block.rect) {
                    block.lives -= 1;
                }
            });
            row.retain(|block| block.lives > 0);
        });
    }

    fn render(&self, state: &mut Engine) {
        state.render_texture(&self.paddle.rect, &self.textures[0]);
        state.render_texture(&self.ball.to_rect(), &self.textures[1]);

        self.blocks.iter().for_each(|row| {
            row.iter().for_each(|block| {
                state.render_texture(&block.rect, &self.textures[2]);
            })
        });
    }
}

struct Block {
    rect: Rect,
    lives: usize,
}
impl Block {
    const SIZE: Vec2 = vec2(12., 9.);
    pub fn new(x: f64, y: f64) -> Self {
        Self {
            rect: rect_vec(vec2(x, y), Self::SIZE),
            lives: 1,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Ball {
    pos: Vec2,
    vel: Vec2,
}
impl Ball {
    const DIAMETER: f64 = 64.;
    fn new(pos: Vec2) -> Self {
        Self {
            pos,
            vel: vec2(400., -400.),
        }
    }
    fn update(&mut self, delta_t: f64) {
        self.pos += self.vel * delta_t;
        let diameter = Self::DIAMETER;

        if self.pos.x + diameter > WINDOW_SIZE.x {
            self.pos.x = WINDOW_SIZE.x - diameter;
            self.vel.x *= -1.;
        } else if self.pos.x < 0. {
            self.pos.x = 0.;
            self.vel.x *= -1.;
        }
        if self.pos.y + diameter > WINDOW_SIZE.y {
            self.vel.y *= -1.;
            self.pos.y = WINDOW_SIZE.y - diameter;
        } else if self.pos.y < 0. {
            self.pos.y = 0.;
            self.vel.y *= -1.;
        }
    }

    fn resolve_paddle_collision(&mut self, paddle: &Paddle) {
        if self.pos.x + Self::DIAMETER > paddle.rect.x
            && self.pos.x < paddle.rect.x + paddle.rect.w
            && self.pos.y + Self::DIAMETER > paddle.rect.y
        {
            self.pos.y = paddle.rect.y - Self::DIAMETER;
            self.vel.y *= -1.;
        }
    }

    fn to_rect(self) -> Rect {
        rect_vec(self.pos, vec2(Ball::DIAMETER, Ball::DIAMETER))
    }
}

#[derive(Debug, Clone, Copy)]
struct Paddle {
    rect: Rect,
}
impl Paddle {
    const SPEED: f64 = 500.;
    const SIZE: Vec2 = vec2(192., 64.);

    fn new(pos: Vec2) -> Self {
        Self {
            rect: rect_vec(pos, Self::SIZE),
        }
    }

    fn update(&mut self, input: &Input, delta_t: f64) {
        let speed = Self::SPEED * delta_t;

        if input.is_d_pressed() && self.rect.x + self.rect.w < WINDOW_SIZE.x {
            self.rect.x += speed;
        }
        if input.is_a_pressed() && self.rect.x > 0. {
            self.rect.x -= speed;
        }
    }
}

fn resolve_collision(a: &mut Rect, vel: &mut Vec2, b: Rect) -> bool {
    // early exit
    let intersection = match a.intersect(b) {
        Some(intersection) => intersection,
        None => return false,
    };

    let to = b.center() - a.center();
    let to_signum = vec2(to.x.signum(), to.y.signum());
    if intersection.w > intersection.h {
        // bounce on y
        a.y -= to_signum.y * intersection.h;
        vel.y = -to_signum.y * vel.y.abs();
    } else {
        // bounce on x
        a.x -= to_signum.x * intersection.w;
        vel.x = -to_signum.x * vel.x.abs();
    }
    true
}
