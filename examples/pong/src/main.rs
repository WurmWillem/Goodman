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
    let paddle_0_tex = engine.create_texture(paddle_bytes, "paddle0").unwrap();

    let paddle_bytes = include_bytes!("assets/Player.png");
    let paddle_1_tex = engine.create_texture(paddle_bytes, "paddle1").unwrap();

    let ball_bytes = include_bytes!("assets/Ball.png");
    let ball_tex = engine.create_texture(ball_bytes, "ball").unwrap();

    let pong = Pong::new(vec![paddle_0_tex, paddle_1_tex, ball_tex]);

    engine.enter_loop(pong, event_loop);
}

struct Pong {
    left_paddle: Paddle,
    right_paddle: Paddle,
    ball: Ball,
    textures: Vec<Texture>,
}
impl Manager for Pong {
    fn new(textures: Vec<Texture>) -> Self {
        let left_paddle = Paddle::new(80., SCREEN_SIZE.y * 0.5);
        let right_paddle = Paddle::new(SCREEN_SIZE.x - 80., SCREEN_SIZE.y * 0.5);
        let ball = Ball::new();

        Self {
            left_paddle,
            right_paddle,
            ball,
            textures,
        }
    }

    fn update(&mut self, frame_time: f64, input: &Input) {
        /*self.left_paddle
            .update(input.is_w_pressed(), input.is_s_pressed(), frame_time);
        self.right_paddle.update(
            input.is_up_arrow_pressed(),
            input.is_down_arrow_pressed(),
            frame_time,
        );*/
        self.ball.update(frame_time);

        //self.ball.resolve_collisions(&self.left_paddle);
        //self.ball.resolve_collisions(&self.right_paddle);
    }

    fn render(&self, engine: &mut Engine) {
        engine.draw_texture(&self.left_paddle.rect, &self.textures[0], Layer1);
        engine.draw_texture(&self.right_paddle.rect, &self.textures[1], Layer1);
        engine.draw_texture(&self.ball.to_rect(), &self.textures[2], Layer1);
    }
}
