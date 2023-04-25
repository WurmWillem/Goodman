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
    state.set_fps(Some(144));*/
    let rect = rect(5., 10., 30., 40.);
    let inst = Instance::new(rect).to_raw();
    println!("{:?}", inst.model[0]);
    println!("{:?}", inst.model[1]);
    println!("{:?}", inst.model[2]);
    //println!("{:?}", inst.model[3]);

    /*let manager = StateManager::new(vec![]);
    state.enter_loop(manager, event_loop);*/
}

struct StateManager;
impl Manager for StateManager {
    fn new(_textures: Vec<Texture>) -> Self {
        Self {}
    }
    fn update(&mut self, _frame_time: f64, _input: &Input) {}
    fn render(&self, _state: &mut Engine) {}
}
