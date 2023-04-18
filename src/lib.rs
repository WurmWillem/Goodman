mod camera;
mod engine;
mod instances;
mod math;
mod object_data;
mod minor_types;
mod texture;
pub mod prelude;

use prelude::*;

pub async fn run() {
    let event_loop = EventLoop::new();
    let mut state = Engine::new(vec2(700., 700.), &event_loop).await;
    //state.set_fps(Some(144));

    let manager = StateManager::new(&mut state, vec![]);
    state.enter_loop(manager, event_loop);
}

struct StateManager;
impl Manager for StateManager {
    fn new(_state: &mut Engine, _textures: Vec<Texture>) -> Self {
        Self {}
    }
    fn update(&mut self, _frame_time: f64, _input: &Input) {}
    fn render(&self, _state: &mut Engine) {}
}
