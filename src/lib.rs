mod camera;
mod engine;
mod instances;
mod math;
mod minor_types;
pub mod prelude;
mod texture;

use instances::Instance;
use prelude::*;

pub async fn run() {
    /*let event_loop = EventLoop::new();
    let mut state = Engine::new(vec2(700., 700.), &event_loop).await;
    state.set_fps(Some(144));

    let manager = StateManager::new(vec![]);
    state.enter_loop(manager, event_loop);*/
    let inst = Instance::new(rect(10., 5., 100., 100.,), 20.).to_raw();
    
}

struct StateManager;
impl Manager for StateManager {
    fn update(&mut self, _frame_time: f64, _input: &Input) {}
    fn render(&self, _state: &mut Engine) {}
}
