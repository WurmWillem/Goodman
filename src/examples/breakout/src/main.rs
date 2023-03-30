use goodman::prelude::*;

fn main() {
    block_on(run());
}

async fn run() {
    let event_loop = EventLoop::new();

    let window = WindowBuilder::new()
        .with_inner_size(LogicalSize::new(700., 700.))
        .build(&event_loop)
        .expect("Failed to build window");

    let mut state = State::new(window).await;
    state.set_fps(144);

    let paddle_bytes = include_bytes!("assets/paddle.png");
    let paddle_tex = state.create_texture(paddle_bytes, "paddle.png");

    let ball_bytes = include_bytes!("assets/ball.png");
    let ball_tex = state.create_texture(ball_bytes, "ball.png");

    let breakout = Breakout::new(&mut state, vec![paddle_tex, ball_tex]);

    enter_loop(event_loop, state, breakout);
}

struct Breakout {
    ball: Ball,
    paddle: Paddle,
    textures: Vec<Texture>,
}
impl Manager for Breakout {
    fn new(state: &mut State, textures: Vec<Texture>) -> Self {
        let paddle = Paddle::new(vec2(0., -0.9));
        let ball = Ball::new(vec2(0., 0.));

        let rects = vec![paddle.rect, ball.to_rect()];
        state.update_instances(rects);

        Self {
            ball,
            paddle,
            textures,
        }
    }

    fn update(&mut self, state: &State) {
        let frame_time = state.get_frame_time();

        self.paddle.update(&state.input, frame_time);
        self.ball.update(frame_time);

        self.ball.resolve_collisions(&self.paddle);
    }

    fn render(&self, state: &mut State) {
        state.draw_texture(self.paddle.rect, &self.textures[0]);
        state.draw_texture(self.ball.to_rect(), &self.textures[1]);
    }
}

#[derive(Debug, Clone, Copy)]
struct Ball {
    pos: Vec2,
    vel: Vec2,
}
impl Ball {
    const RADIUS: f64 = 0.7;
    fn new(pos: Vec2) -> Self {
        Self {
            pos,
            vel: vec2(2., 2.),
        }
    }
    fn update(&mut self, frame_time: f64) {
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

    fn resolve_collisions(&mut self, paddle: &Paddle) {
        let paddle_width = paddle.rect.width * VERTEX_SCALE;
        let paddle_height = paddle.rect.height * VERTEX_SCALE;
        let radius_scaled = Self::RADIUS * VERTEX_SCALE;

        if self.pos.x > paddle.rect.x - paddle_width - radius_scaled
            && self.pos.x < paddle.rect.x + paddle_width + radius_scaled
            && self.pos.y - radius_scaled < paddle.rect.y + paddle_height
        {
            self.pos.y = paddle.rect.y + paddle_height + radius_scaled;
            self.vel.y *= -1.;
        }
    }

    fn to_rect(&self) -> Rect {
        rect(self.pos, vec2(Ball::RADIUS, Ball::RADIUS))
    }
}

#[derive(Debug, Clone, Copy)]
struct Paddle {
    rect: Rect,
}
impl Paddle {
    const SPEED: f64 = 2.5;
    const SIZE: Vec2 = vec2(3., 1.);

    fn new(pos: Vec2) -> Self {
        Self {
            rect: rect(pos, Self::SIZE),
        }
    }

    fn update(&mut self, input: &Input, frame_time: f64) {
        let speed = Self::SPEED * frame_time;
        let size_scaled_x = self.rect.width * VERTEX_SCALE + speed;

        if input.d_pressed && self.rect.x + size_scaled_x < 1. {
            self.rect.x += speed;
        }
        if input.a_pressed && self.rect.x - size_scaled_x > -1. {
            self.rect.x -= speed;
        }
    }
}
