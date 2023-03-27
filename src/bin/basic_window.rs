use goodman::prelude::*;

fn main() {
    pollster::block_on(run());
}

pub async fn run() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_inner_size(LogicalSize::new(700., 700.))
        .build(&event_loop)
        .expect("Failed to build window");

    let mut state = State::new(window).await;
    let game_manager = GameManager::new(&mut state, vec![]);

    state.target_fps = 144;

    enter_loop(event_loop, state, game_manager);
}

struct GameManager {
    square: Square,
}
impl Manager for GameManager {
    fn new(state: &mut State, _textures: Vec<Texture>) -> Self {
        let square = Square::new();
        state.instances = vec![square.to_instance()];
        state.update_square_instances();

        Self { square }
    }
    fn update(&mut self, _state: &mut State) {}

    fn render(&self, state: &mut State) -> Result<(), wgpu::SurfaceError> {
        state.render()
    }
}

struct Square {
    pos: Vec2,
    size: Vec2,
}
impl Square {
    fn new() -> Self {
        Self {
            pos: vec2(0., 0.), // Center of the screen
            size: vec2(1., 3.),
        }
    }
}
impl SquareInstanceT for Square {
    fn to_instance(&self) -> SquareInstance {
        SquareInstance::new(self.pos, self.size)
    }
}
