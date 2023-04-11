mod camera;
mod instances;
mod math;
mod object_data;
pub mod prelude;
mod state;
mod state_manager;
mod texture;

use cgmath::vec2;
use prelude::{Manager, Texture};
use state::State;
use winit::event_loop::EventLoop;

pub async fn run() {
    let event_loop = EventLoop::new();
    let mut state = State::new(vec2(700., 700.), &event_loop).await;
    let manager = StateManager::new(&mut state, vec![]);

    state.enter_loop(manager, event_loop);
}

struct StateManager;
impl Manager for StateManager {
    fn new(_state: &mut State, _textures: Vec<Texture>) -> Self {
        Self {}
    }
    fn update(&mut self, _state: &State) {}
    fn render(&self, _state: &mut State) {}
}
