// This example only creates a window with UI, The next step is to go to the Pong example for an example of an actual game

// import all necessary things from the game engine
use goodman::prelude::*;

// The size of the window
pub const WINDOW_SIZE: Vec32 = vec2(1200., 800.);

// main function that will be run when te program is executed
fn main() {
    block_on(run());
}

async fn run() {
    // This variable is necessary for the creation of the engine, and is necessary to enter the loop
    let event_loop = EventLoop::new(); 

    let mut engine = EngineBuilder::new(WINDOW_SIZE)
        .show_engine_ui() // specify that you want to show the ui of the engine, which shows stuff like fps and tps 
        .with_target_fps(144) // specify that you want to run the program at 144 fps
        .build(&event_loop) 
        .await;

    // create your struct by running the required new function
    let game = Game::new(&mut engine); 

    // start the loop, from this point onward update will be called every tick/frame (depends on if a target_tps is specified)
    // and update
    engine.start_loop(game, event_loop);
}

struct Game {
    // data needed for game should go in this struct
}
impl Manager for Game {
    // is called once upon creation of the struct, initialize assets in here
    fn new(_engine: &mut Engine) -> Self {
        Self {  }        
    }

    // update your data, gets called every tick/frame (depending on if a target_tps is specified)
    fn update(&mut self, _delta_t: f64, _input: &Input, _sound: &mut Sound) {
        
    }

    // render your specified textures, gets called every frame
    fn render(&mut self, _engine: &mut Engine) {
        
    }
}