fn main() {
    block_on(run())
}

use goodman::prelude::*;

async fn run() {
    let event_loop = EventLoop::new();
    let mut engine = EngineBuilder::new(vec2(700., 700.))
        .show_engine_ui()
        .build(&event_loop)
        .await;

    let thing = StateManager::new(&mut engine);
    engine.start_loop(thing, event_loop)
}

struct StateManager;
impl Manager for StateManager {
    fn new(_engine: &mut Engine) -> Self {
        Self {}
    }
    fn update(&mut self, _frame_time: f64, _input: &Input, _sound: &mut Sound) {}
    fn render(&mut self, _state: &mut Engine) {}
}
