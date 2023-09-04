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

    let mut engine = EngineBuilder::new(WINDOW_SIZE)
        .show_engine_ui()
        // .set_target_fps(144)
        // .set_target_tps(100 * 1000)
        .build(&event_loop)
        .await;

    let pong = Pong::new(&mut engine);

    engine.start_loop(pong, event_loop);
}

struct Pong {
    left_paddle: Paddle,
    right_paddle: Paddle,
    ball: Ball,
    textures: Vec<Texture>,
}
impl Manager for Pong {
    fn new(engine: &mut Engine) -> Self {
        let mut textures = vec![];
        create_textures!(engine, textures, "assets/Computer.png" "assets/Player.png" "assets/Ball.png" "assets/dak.png");
        // engine.use_textures(&textures);

        let left_paddle = Paddle::new(80., WINDOW_SIZE.y * 0.5);
        let right_paddle = Paddle::new(WINDOW_SIZE.x - 80., WINDOW_SIZE.y * 0.5);
        let ball = Ball::new();

        Self {
            left_paddle,
            right_paddle,
            ball,
            textures,
        }
    }

    fn update(&mut self, delta_t: f64, input: &Input, _sound: &Sound) {
        self.left_paddle.update(
            input.is_button_held(Button::W),
            input.is_button_held(Button::S),
            delta_t,
        );
        self.right_paddle.update(
            input.is_button_held(Button::UpArrow),
            input.is_button_held(Button::DownArrow),
            delta_t,
        );

        /*self.ball.update(delta_t);
        self.ball.resolve_collisions_left_paddle(&self.left_paddle);
        self.ball
            .resolve_collisions_right_paddle(&self.right_paddle);*/
    }

    fn render(&self, engine: &mut Engine) {
        /*let mut ui = GoodManUI::new();
        ui.set_title("Pong");
        ui.add_label(format!("ball position: {} {}", self.ball.pos.x as u32, self.ball.pos.y as u32));
        engine.set_game_ui(ui);*/
        let you = DrawParams {
            source: Some(rect(24., 0., 24., 24.)),
            ..Default::default()
        };
        let win = DrawParams {
            source: Some(rect(0., 0., 24., 24.)),
            ..Default::default()
        };
        engine.render_texture_ex(&self.ball.to_rect(), &self.textures[3], you);
        engine.render_texture_ex(&self.right_paddle.rect, &self.textures[3], win);

        /*engine.render_texture(&self.left_paddle.rect, &self.textures[0]);
        engine.render_texture(&self.right_paddle.rect, &self.textures[1]);
        engine.render_texture(&self.ball.to_rect(), &self.textures[2]);*/
    }
}
