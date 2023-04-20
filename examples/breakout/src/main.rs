use goodman::prelude::*;

fn main() {
    block_on(run());
}

const SCREEN_SIZE: Vec2 = vec2(1200., 900.);

async fn run() {
    let event_loop = EventLoop::new();
    let mut engine: Engine = Engine::new(SCREEN_SIZE, &event_loop).await;

    engine.set_fps(Some(144));

    let paddle_bytes = include_bytes!("assets/paddle.png");
    let paddle_tex = engine.create_texture(paddle_bytes, "paddle.png");
    let ball_bytes = include_bytes!("assets/ball.png");
    let ball_tex = engine.create_texture(ball_bytes, "ball.png");
    let block_bytes = include_bytes!("assets/block.png");
    let block_tex = engine.create_texture(block_bytes, "block.png");

    let breakout = Breakout::new(&mut engine, vec![paddle_tex, ball_tex, block_tex]);

    engine.enter_loop(breakout, event_loop);
}

struct Breakout {
    ball: Ball,
    paddle: Paddle,
    blocks: Vec<Vec<Block>>,
    textures: Vec<Texture>,
}
impl Manager for Breakout {
    fn new(state: &mut Engine, textures: Vec<Texture>) -> Self {
        let paddle = Paddle::new(vec2(SCREEN_SIZE.x * 0.5, SCREEN_SIZE.y * 0.1));
        let ball = Ball::new(vec2(0., 0.));

        let mut rects = vec![paddle.rect, ball.to_rect()];

        let mut blocks = Vec::new();
        for j in 0..10 {
            let mut row = Vec::new();
            for i in 0..10 {
                let block = Block::new(i as f64 * 100. + 150., j as f64 * 50. + 500.);
                rects.push(block.rect);
                row.push(block);
            }
            blocks.push(row);
        }

        state.initialize_instances(rects);

        Self {
            ball,
            paddle,
            blocks,
            textures,
        }
    }

    fn update(&mut self, frame_time: f64, input: &Input) {
        /*self.paddle.update(input, frame_time);
        self.ball.update(frame_time);

        self.ball.resolve_paddle_collision(&self.paddle);*/

        /*self.blocks.iter_mut().for_each(|row| {
            row.iter_mut().for_each(|mut block| {
                if resolve_collision(&mut self.ball.to_rect(), &mut self.ball.vel, block.rect) {
                    block.lives -= 1;
                }
            });
            row.retain(|block| block.lives > 0);
        });*/
    }

    fn render(&self, state: &mut Engine) {
        state.draw_texture(self.paddle.rect, &self.textures[0]);
        state.draw_texture(self.ball.to_rect(), &self.textures[1]);

        self.blocks.iter().for_each(|row| {
            row.iter().for_each(|block| {
                state.draw_texture(block.rect, &self.textures[2]);
            })
        });
    }
}

fn resolve_collision(a: &mut Rect, vel: &mut Vec2, b: Rect) -> bool {
    // early exit
    let intersection = match a.intersect(b) {
        Some(intersection) => intersection,
        None => return false,
    };
    println!("colliding");

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

struct Block {
    rect: Rect,
    lives: usize,
}
impl Block {
    const SIZE: Vec2 = vec2(100., 50.);
    pub fn new(x: f64, y: f64) -> Self {
        Self {
            rect: rect(vec2(x, y), Self::SIZE),
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
    const RADIUS: f64 = 25.;
    fn new(pos: Vec2) -> Self {
        Self {
            pos,
            vel: vec2(200., 200.),
        }
    }
    fn update(&mut self, frame_time: f64) {
        self.pos += self.vel * frame_time;

        if self.pos.x + Self::RADIUS > SCREEN_SIZE.x {
            self.pos.x = SCREEN_SIZE.x - Self::RADIUS;
            self.vel.x *= -1.;
        } else if self.pos.x - Self::RADIUS < 0. {
            self.pos.x = Self::RADIUS;
            self.vel.x *= -1.;
        }
        if self.pos.y + Self::RADIUS > SCREEN_SIZE.y {
            self.vel.y *= -1.;
            self.pos.y = SCREEN_SIZE.y - Self::RADIUS;
        } else if self.pos.y - Self::RADIUS < 0. {
            self.pos.y = Self::RADIUS;
            self.vel.y *= -1.;
        }
    }

    fn resolve_paddle_collision(&mut self, paddle: &Paddle) {
        if self.pos.x + Self::RADIUS > paddle.rect.x - paddle.rect.w * 0.5
            && self.pos.x - Self::RADIUS < paddle.rect.x + paddle.rect.w * 0.5
            && self.pos.y - Self::RADIUS < paddle.rect.y + paddle.rect.h * 0.5
        {
            self.pos.y = paddle.rect.y + paddle.rect.h * 0.5 + Self::RADIUS;
            self.vel.y *= -1.;
        }
    }

    fn to_rect(self) -> Rect {
        rect(self.pos, vec2(Ball::RADIUS * 2., Ball::RADIUS * 2.))
    }
}

#[derive(Debug, Clone, Copy)]
struct Paddle {
    rect: Rect,
}
impl Paddle {
    const SPEED: f64 = 500.;
    const SIZE: Vec2 = vec2(180., 60.);

    fn new(pos: Vec2) -> Self {
        Self {
            rect: rect(pos, Self::SIZE),
        }
    }

    fn update(&mut self, input: &Input, frame_time: f64) {
        let speed = Self::SPEED * frame_time;
        let width = self.rect.w * 0.5;

        if input.is_d_pressed() && self.rect.x + width < SCREEN_SIZE.x {
            self.rect.x += speed;
        }
        if input.is_a_pressed() && self.rect.x - width > 0. {
            self.rect.x -= speed;
        }
    }
}
