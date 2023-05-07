pub const WINDOW_SIZE: Vec2 = vec2(1200., 800.);

use goodman::prelude::*;
fn main() {
    block_on(run());
}

async fn run() {
    let event_loop = EventLoop::new();
    let mut engine = Engine::new(WINDOW_SIZE, &event_loop, true).await;

    let background_color = Color::new(223., 173., 96., 255.);
    engine.set_background_color(background_color);
    engine.set_target_fps(Some(144));
    engine.set_target_tps(Some(100 * 1000));
    engine.enable_feature(Feature::EngineUi);

    let game = Game::new(&mut engine);

    engine.enter_loop(game, event_loop);
}

struct Game {
    player: Player,
    cursor_pos: Vec2,
    textures: Vec<Texture>,
}
impl Manager for Game {
    fn new(engine: &mut Engine) -> Self {
        let crosshair_bytes = include_bytes!("assets/crosshair.png");
        let crosshair = engine.create_texture(crosshair_bytes, "crosshair").unwrap();
        let player_bytes = include_bytes!("assets/char.png");
        let player = engine.create_texture(player_bytes, "player").unwrap();

        Self {
            player: Player::new(vec2(0., 0.)),
            cursor_pos: vec2(0., 0.),
            textures: vec![crosshair, player],
        }
    }

    fn update(&mut self, delta_t: f64, input: &Input) {
        self.player.update(delta_t, input);
        self.cursor_pos = input.get_cursor_pos();
    }

    fn render(&self, engine: &mut Engine) {
        engine.render_texture(&self.player.rect, &self.textures[1]);
        engine.render_texture(
            &rect(self.cursor_pos.x - 8., self.cursor_pos.y - 8., 16., 16.),
            &self.textures[0],
        );
    }
}

struct Player {
    rect: Rect,
    vel: Vec2,
}
impl Player {
    const ACC: f64 = 5.;
    fn new(pos: Vec2) -> Self {
        let rect = rect(pos.x, pos.y, 128., 128.);
        Self {
            rect,
            vel: vec2(0., 0.),
        }
    }
    fn update(&mut self, delta_t: f64, input: &Input) {
        if input.is_w_pressed() {
            self.vel.y -= Self::ACC;
        }
        if input.is_s_pressed() {
            self.vel.y += Self::ACC;
        }
        if input.is_a_pressed() {
            self.vel.x -= Self::ACC;
        }
        if input.is_d_pressed() {
            self.vel.x += Self::ACC;
        }
        self.vel *= 0.99;
        self.rect.xy_add(self.vel * delta_t);
        //println!("{}", self.rect.x);
    }
}
