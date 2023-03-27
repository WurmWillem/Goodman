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
    state.set_fps(144);

    let paddle_bytes = include_bytes!("assets/paddle.png");
    let paddle_tex = state.create_texture(paddle_bytes, "paddle.png");

    let game_manager = GameManager::new(&mut state, vec![paddle_tex]);

    enter_loop(event_loop, state, game_manager);
}

struct GameManager {
    square: Square,
    textures: Vec<Texture>,
}
impl Manager for GameManager {
    fn new(state: &mut State, textures: Vec<Texture>) -> Self {
        let square = Square::new();

        state.update_instances(vec![square.rect]);

        Self { square, textures }
    }
    fn update(&mut self, _state: &mut State) {}

    fn render(&self, state: &mut State) {
        state.draw_texture(self.square.rect, &self.textures[0]);
    }
}

struct Square {
    rect: Rect,
}
impl Square {
    fn new() -> Self {
        Self {
            rect: rect(vec2(0., 0.), vec2(1., 3.)),
        }
    }
}
