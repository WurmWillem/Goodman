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
    let pong = Pong::new(&mut state);

    state.target_fps = 144;

    enter_loop(event_loop, state, pong);
}

struct Pong {
    paddle_0: Paddle,
    paddle_1: Paddle,
    ball: Ball,
}
impl Manager for Pong {
    fn new(state: &mut State) -> Self {
        let paddle_0 = Paddle::new(vec2(-0.8, 0.));
        let paddle_1 = Paddle::new(vec2(0.8, 0.));
        let ball = Ball::new();

        state.square_instances = vec![paddle_0.to_square_instance(), paddle_1.to_square_instance()];
        state.circle_instances = vec![ball.to_circle_instance()];

        state.update_square_instances();
        state.update_circle_instances();

        Self {
            paddle_0,
            paddle_1,
            ball,
        }
    }

    fn update(&mut self, state: &mut State) {
        let paddle_0 = &mut self.paddle_0;
        let paddle_1 = &mut self.paddle_1;

        let frame_time = state.get_frame_time();

        paddle_0.update(state.input.w_pressed, state.input.s_pressed, frame_time);
        paddle_1.update(state.input.up_pressed, state.input.down_pressed, frame_time);
        self.ball.update(paddle_0, paddle_1, frame_time);

        state.square_instances[0] = paddle_0.to_square_instance();
        state.square_instances[1] = paddle_1.to_square_instance();
        state.update_square_instances();

        state.circle_instances[0] = self.ball.to_circle_instance();
        state.update_circle_instances();
    }
}

#[derive(Debug, Clone, Copy)]
struct Ball {
    pos: Vec2,
    vel: Vec2,
    radius: f64,
}
impl Ball {
    fn new() -> Self {
        Self {
            pos: vec2(0., 0.),
            vel: vec2(2., 2.),
            radius: 1.,
        }
    }
    fn update(&mut self, paddle_0: &Paddle, paddle_1: &Paddle, frame_time: f64) {
        let radius_scaled = self.radius * (VERTEX_SCALE as f64);

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

        let size_scaled_x = paddle_0.size.x * VERTEX_SCALE as f64 * 0.5 + 0.02;
        let size_scaled_y = paddle_0.size.y * VERTEX_SCALE as f64 * 0.5 + 0.02;

        if (new_pos.x + radius_scaled > paddle_1.pos.x - size_scaled_x
            && new_pos.y + radius_scaled > paddle_1.pos.y - size_scaled_y
            && new_pos.y - radius_scaled < paddle_1.pos.y + size_scaled_y
            && self.vel.x > 0.)
            || (new_pos.x - radius_scaled < paddle_0.pos.x + size_scaled_x
                && new_pos.y + radius_scaled > paddle_0.pos.y - size_scaled_y
                && new_pos.y - radius_scaled < paddle_0.pos.y + size_scaled_y
                && self.vel.x < 0.)
        {
            self.vel.x *= -1.;
        }

        self.pos += self.vel * frame_time;
    }
}
impl CircleInstanceT for Ball {
    fn to_circle_instance(&self) -> CircleInstance {
        CircleInstance::new(self.pos, self.radius)
    }
}

#[derive(Debug, Clone, Copy)]
struct Paddle {
    pos: Vec2,
    size: Vec2,
}
impl Paddle {
    const PADDLE_SPEED: f64 = 2.5;
    const PADDLE_SIZE: Vec2 = vec2(1., 3.);
    fn new(pos: Vec2) -> Self {
        Self {
            pos,
            size: Self::PADDLE_SIZE,
        }
    }
    fn update(&mut self, up_pressed: bool, down_pressed: bool, frame_time: f64) {
        let speed = Self::PADDLE_SPEED * frame_time;
        let size_scaled_y = self.size.y * VERTEX_SCALE as f64 * 0.5 + speed + 0.07;

        if up_pressed && self.pos.y + size_scaled_y < 1. {
            self.pos.y += speed;
        }
        if down_pressed && self.pos.y - size_scaled_y > -1. {
            self.pos.y -= speed;
        }
    }
}
impl SquareInstanceT for Paddle {
    fn to_square_instance(&self) -> SquareInstance {
        SquareInstance::new(self.pos, Self::PADDLE_SIZE)
    }
}
