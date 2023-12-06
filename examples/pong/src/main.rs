use goodman::prelude::*;

// paddle.rs and ball.rs don't contain any engine related code and don't do anything special
mod ball;
use ball::Ball;
mod paddle;
use paddle::Paddle;

pub const WINDOW_SIZE: Vec64 = vec2(1200., 800.);

fn main() {
    block_on(run());
}

async fn run() {
    let event_loop = EventLoop::new();

    let window_size = vec2(WINDOW_SIZE.x as f32, WINDOW_SIZE.y as f32);
    let mut engine = EngineBuilder::new(window_size)
        // .show_engine_ui()
        .with_target_fps(144)
        // .with_target_tps(100 * 1000)
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
        // create textures like this with the create_textures! macro
        // textures get safed into the "textures" vector, and get saved in the Pong struct
        let mut textures = vec![];
        create_textures!(engine, textures, "assets/Computer.png" "assets/Player.png" "assets/Ball.png");

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

    fn update(&mut self, delta_t: f64, input: &Input, _sound: &mut Sound) {
        // input gets used to check if certain buttons are held
        // if the button w is held down then input.is_button_held(Button::W) will returnt true, otherwise false
        // delta_t is used to make the game run at a stable framerate
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

        self.ball.update(delta_t);
        self.ball.resolve_collisions_left_paddle(&self.left_paddle);
        self.ball
            .resolve_collisions_right_paddle(&self.right_paddle);
    }

    fn render(&mut self, engine: &mut Engine) {
        // uncomment this to show the ball position in the ui
        /*let mut ui = UserUi::new("Pong");
        ui.add_label(format!("ball position: {} {}", self.ball.pos.x as u32, self.ball.pos.y as u32));
        engine.set_user_ui(ui);*/

        // this is how you render textures, you give a rect(x, y, width, height) for the position and size
        // and you give a reference to the texture you want to use
        engine.render_texture(self.left_paddle.rect.into(), &self.textures[0]);
        engine.render_texture(self.right_paddle.rect.into(), &self.textures[1]);
        engine.render_texture(self.ball.to_rect(), &self.textures[2]);
    }
}
