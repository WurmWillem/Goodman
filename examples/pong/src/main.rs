use goodman::prelude::*;

mod ball;
use ball::Ball;
mod paddle;
use paddle::Paddle;

pub const SCREEN_SIZE: Vec2 = vec2(1200., 800.);

fn main() {
    block_on(run());
}

async fn run() {
    let event_loop = EventLoop::new();
    let mut engine = Engine::new(SCREEN_SIZE, &event_loop).await;

    engine.set_fps(Some(144));

    let paddle_bytes = include_bytes!("assets/Computer.png");
    let paddle0_tex = engine.create_texture(paddle_bytes, "paddle1.png");

    let paddle_bytes = include_bytes!("assets/Player.png");
    let paddle1_tex = engine.create_texture(paddle_bytes, "paddle.png");

    let ball_bytes = include_bytes!("assets/Ball.png");
    let ball_tex = engine.create_texture(ball_bytes, "ball.png");

    let pong = Pong::new(&mut engine, vec![paddle0_tex ,paddle1_tex, ball_tex]);

    engine.enter_loop(pong, event_loop);
}

struct Pong {
    paddle_0: Paddle,
    paddle_1: Paddle,
    ball: Ball,
    textures: Vec<Texture>,
}
impl Manager for Pong {
    fn new(engine: &mut Engine, textures: Vec<Texture>) -> Self {
        let paddle_0 = Paddle::new(80., SCREEN_SIZE.y * 0.5);
        let paddle_1 = Paddle::new(SCREEN_SIZE.x - 80., SCREEN_SIZE.y * 0.5);
        let ball = Ball::new();

        let rects = vec![paddle_0.rect, paddle_1.rect, ball.to_rect()];
        engine.initialize_instances(rects);

        Self {
            paddle_0,
            paddle_1,
            ball,
            textures,
        }
    }

    fn update(&mut self, frame_time: f64, input: &Input) {
        let paddle_0 = &mut self.paddle_0;
        let paddle_1 = &mut self.paddle_1;

        paddle_0.update(input.is_w_pressed(), input.is_s_pressed(), frame_time);
        paddle_1.update(input.is_up_arrow_pressed(), input.is_down_arrow_pressed(), frame_time);
        self.ball.update(frame_time);

        self.ball.resolve_collisions(paddle_0);
        self.ball.resolve_collisions(paddle_1);
    }

    fn render(&self, engine: &mut Engine) {
        engine.draw_texture(self.paddle_0.rect, &self.textures[0]);
        engine.draw_texture(self.paddle_1.rect, &self.textures[1]);
        engine.draw_texture(self.ball.to_rect(), &self.textures[2]);
    }
}
