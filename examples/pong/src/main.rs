use goodman::prelude::*;

mod ball;
use ball::Ball;
mod paddle;
use paddle::Paddle;

pub const WINDOW_SIZE: Vec2 = vec2(1200., 800.);

fn main() {
    block_on(run());
}

async fn run() {
    let event_loop = EventLoop::new();
    let mut engine = Engine::new(WINDOW_SIZE, &event_loop, true).await;

    engine.set_target_fps(Some(144));
    engine.set_target_tps(Some(144 * 10000));

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
    rot: f64,
}
impl Pong {
    fn new(textures: Vec<Texture>) -> Self {
        let left_paddle = Paddle::new(80., WINDOW_SIZE.y * 0.5);
        let right_paddle = Paddle::new(WINDOW_SIZE.x - 80., WINDOW_SIZE.y * 0.5);
        let ball = Ball::new();

        Self {
            left_paddle,
            right_paddle,
            ball,
            textures,
            rot: 0.,
        }
    }
}
impl Manager for Pong {
    fn update(&mut self, delta_t: f64, input: &Input) {
        self.left_paddle
            .update(input.is_w_pressed(), input.is_s_pressed(), delta_t);
        self.right_paddle.update(
            input.is_up_arrow_pressed(),
            input.is_down_arrow_pressed(),
            delta_t,
        );
        self.ball.update(delta_t);
        self.rot += 0.1;
        self.ball.resolve_collisions_left_paddle(&self.left_paddle);
        self.ball
            .resolve_collisions_right_paddle(&self.right_paddle);
    }

    fn render(&self, engine: &mut Engine) {
        let draw_p = DrawParams {
            rotation: 0.,
            ..Default::default()
        };
        engine.render_texture_ex(&self.left_paddle.rect, &self.textures[0], draw_p);
        engine.render_texture(&self.right_paddle.rect, &self.textures[1]);
        engine.render_texture(&self.ball.to_rect(), &self.textures[2]);
    }
}
