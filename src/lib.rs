mod camera;
mod engine;
mod engine_manager;
mod instances;
mod math;
mod object_data;
pub mod prelude;
mod texture;

use cgmath::vec2;
use engine::Engine;
use prelude::{Manager, Texture};
use winit::event_loop::EventLoop;

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
    fn update(&mut self, _state: &Engine) {}
    fn render(&self, _state: &mut Engine) {}
}
