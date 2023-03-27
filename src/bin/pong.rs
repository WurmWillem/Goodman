use goodman::prelude::*;

fn main() {
    pollster::block_on(run());
}

pub async fn run() {
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

    let pong = Pong::new(&mut state, vec![paddle_tex, ball_tex]);

    enter_loop(event_loop, state, pong);
}

struct Pong {
    paddle_0: Paddle,
    paddle_1: Paddle,
    ball: Ball,
    textures: Vec<Texture>,
}
impl Manager for Pong {
    fn new(state: &mut State, textures: Vec<Texture>) -> Self {
        let paddle_0 = Paddle::new(vec2(-0.8, 0.));
        let paddle_1 = Paddle::new(vec2(0.8, 0.));
        let ball = Ball::new();

        let rects = vec![paddle_0.rect, paddle_1.rect, ball.to_rect()];
        state.update_instances(rects);

        Self {
            paddle_0,
            paddle_1,
            ball,
            textures,
        }
    }

    fn update(&mut self, state: &State) {
        let paddle_0 = &mut self.paddle_0;
        let paddle_1 = &mut self.paddle_1;

        let frame_time = state.get_frame_time();

        paddle_0.update(state.input.w_pressed, state.input.s_pressed, frame_time);
        paddle_1.update(state.input.up_pressed, state.input.down_pressed, frame_time);
        self.ball.update(paddle_0, paddle_1, frame_time);
    }

    fn render(&self, state: &mut State) {
        state.draw_texture(self.paddle_0.rect, &self.textures[0]);
        state.draw_texture(self.paddle_1.rect, &self.textures[0]);
        state.draw_texture(self.ball.to_rect(), &self.textures[1]);
    }
}

#[derive(Debug, Clone, Copy)]
struct Ball {
    pos: Vec2,
    vel: Vec2,
}
impl Ball {
    const RADIUS: f64 = 1.;
    fn new() -> Self {
        Self {
            pos: vec2(0., 0.),
            vel: vec2(2., 2.),
        }
    }
    fn update(&mut self, paddle_0: &Paddle, paddle_1: &Paddle, frame_time: f64) {
        let radius_scaled = Self::RADIUS * (VERTEX_SCALE as f64);

        let new_pos = self.pos + self.vel * frame_time as f64;
        if new_pos.x + radius_scaled > 1. || new_pos.x - radius_scaled < -1. {
            *self = Ball::new();
            return;
        }

        if new_pos.y + radius_scaled > 1. {
            self.vel.y *= -1.;
            self.pos.y = 1. - radius_scaled;
        }
        if new_pos.y - radius_scaled < -1. {
            self.vel.y *= -1.;
            self.pos.y = -1. + radius_scaled;
        }

        let size_scaled_x = paddle_0.rect.width * VERTEX_SCALE as f64 * 0.5 + 0.02;
        let size_scaled_y = paddle_0.rect.height * VERTEX_SCALE as f64 * 0.5 + 0.02;

        if (new_pos.x + radius_scaled > paddle_1.rect.x - size_scaled_x
            && new_pos.y + radius_scaled > paddle_1.rect.y - size_scaled_y
            && new_pos.y - radius_scaled < paddle_1.rect.y + size_scaled_y
            && self.vel.x > 0.)
            || (new_pos.x - radius_scaled < paddle_0.rect.x + size_scaled_x
                && new_pos.y + radius_scaled > paddle_0.rect.y - size_scaled_y
                && new_pos.y - radius_scaled < paddle_0.rect.y + size_scaled_y
                && self.vel.x < 0.)
        {
            self.vel.x *= -1.;
        }

        self.pos += self.vel * frame_time;
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
    const SIZE: Vec2 = vec2(1., 3.);

    fn new(pos: Vec2) -> Self {
        Self {
            rect: rect(pos, Self::SIZE),
        }
    }

    fn update(&mut self, up_pressed: bool, down_pressed: bool, frame_time: f64) {
        let speed = Self::SPEED * frame_time;
        let size_scaled_y = self.rect.height * VERTEX_SCALE as f64 * 0.5 + speed + 0.07;

        if up_pressed && self.rect.y + size_scaled_y < 1. {
            self.rect.y += speed;
        }
        if down_pressed && self.rect.y - size_scaled_y > -1. {
            self.rect.y -= speed;
        }
    }
}
