use goodman::prelude::*;

mod ball;
use ball::Ball;
mod paddle;
use paddle::Paddle;

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
        state.initialize_instances(rects);

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
        self.ball.update(frame_time);

        self.ball.resolve_collisions(paddle_0);
        self.ball.resolve_collisions(paddle_1);
    }

    fn render(&self, state: &mut State) {
        state.draw_texture(self.paddle_0.rect, &self.textures[0]);
        state.draw_texture(self.paddle_1.rect, &self.textures[0]);
        state.draw_texture(self.ball.to_rect(), &self.textures[1]);
    }
}
