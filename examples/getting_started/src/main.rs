use goodman::prelude::*;

fn main() {
    block_on(run())
}

const WINDOW_SIZE: Vec32 = vec2(700., 700.);

async fn run() {
    let event_loop = EventLoop::new();
    let mut engine = EngineBuilder::new(WINDOW_SIZE)
        .show_engine_ui()
        .build(&event_loop)
        .await;

    let game = Game::new(&mut engine);
    engine.start_loop(game, event_loop);
}

struct Game;
impl Manager for Game {
    fn new(_engine: &mut Engine) -> Self {
        Self {
            // Initialize self and create textures and sound files
        }
    }
    fn update(&mut self, _frame_time: f64, _input: &Input, _sound: &mut Sound) {
        // update self (based on input) and play sounds
    }
    fn render(&mut self, _engine: &mut Engine) {
        // Render code here, engine.render_texture(rect, texture)
    }
}
