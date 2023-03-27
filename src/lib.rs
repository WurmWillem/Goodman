mod camera;
mod instances;
mod object_data;
pub mod prelude;
mod state;
mod state_manager;
mod texture;

use prelude::{Manager, Texture};
use state::State;
use winit::{dpi::LogicalSize, event_loop::EventLoop, window::WindowBuilder};

pub async fn run() {
    env_logger::init();

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_inner_size(LogicalSize::new(700., 700.))
        .build(&event_loop)
        .expect("Failed to build window");

    let mut state = State::new(window).await;
    let manager = StateManager::new(&mut state, vec![]);

    state_manager::enter_loop(event_loop, state, manager)
}

struct StateManager;
impl Manager for StateManager {
    fn new(_state: &mut State, _textures: Vec<Texture>) -> Self {
        Self {}
    }
    fn update(&mut self, state: &mut State) {
        state.update();
    }
    fn render(&self, _state: &mut State) {}
}
