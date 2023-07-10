use goodman::prelude::*;

pub const WINDOW_SIZE: Vec2 = vec2(800., 800.);
const GRID_SIZE: (usize, usize) = (10, 10);

fn main() {
    block_on(run());
}

async fn run() {
    let event_loop = EventLoop::new();

    let mut engine = Engine::new(WINDOW_SIZE, &event_loop, true).await;
    engine.set_target_fps(Some(144));
    //engine.set_target_tps(Some(144));
    engine.enable_feature(Feature::EngineUi);
    //engine.enable_feature(Feature::AverageTPS(0.1));

    let game = Game::new(&mut engine);

    engine.enter_loop(game, event_loop);
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Object {
    Empty,
    Is,
    Character(Characters),
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Characters {
    Kirb,
}

struct AllCharacterData {
    kirb: CharacterData,
}
impl AllCharacterData {
    fn new() -> Self {
        Self {
            kirb: CharacterData::new(),
        }
    }
    fn is_you(&self, char: Characters) -> bool {
        match char {
            Characters::Kirb => self.kirb.is_you,
        }
    }
}

struct CharacterData {
    pub is_you: bool,
}
impl CharacterData {
    fn new() -> Self {
        Self { is_you: true }
    }
}
struct Game {
    grid: Vec<Vec<Object>>,
    character_data: AllCharacterData,
    textures: Vec<Texture>,
}
impl Manager for Game {
    fn new(engine: &mut Engine) -> Self {
        let kirb_bytes = include_bytes!("assets/kirb.png");
        let kirb_tex = engine.create_texture(kirb_bytes, "kirb").unwrap();

        let mut grid = vec![];
        for _ in 0..GRID_SIZE.1 {
            let mut row = vec![];
            for _ in 0..GRID_SIZE.0 {
                row.push(Object::Empty);
            }
            grid.push(row);
        }
        grid[0][0] = Object::Character(Characters::Kirb);

        Self {
            grid,
            character_data: AllCharacterData::new(),
            textures: vec![kirb_tex],
        }
    }

    fn update(&mut self, _delta_t: f64, input: &Input) {
        let mut where_to_move = (0, 0);
        if input.is_w_pressed() {
            where_to_move.1 = -1;
        }
        if input.is_d_pressed() {
            where_to_move.0 = 1;
        }
        if input.is_s_pressed() {
            where_to_move.1 = 1;
        }
        if input.is_a_pressed() {
            where_to_move.0 = -1;
        }

        let mut already_moved = vec![];
        for j in 0..self.grid.len() {
            for i in 0..self.grid.len() {
                if let Object::Character(char) = self.grid[j][i] {
                    if self.character_data.is_you(char) {
                        if where_to_move == (0, 0) {
                            continue;
                        } 

                        let mut should_continue = false;
                        for m in already_moved.iter() {
                            if *m == (i, j) {
                                should_continue = true;
                                break;
                            }
                        }
                        if should_continue {
                            continue
                        }

                        let indexes = (
                            (i as i32 + where_to_move.0) as usize,
                            (j as i32 + where_to_move.1) as usize,
                        );                        
                        
                        if let Some(row) = self.grid.get(indexes.1) {
                            if let Some(object) = row.get(indexes.0) {
                                if *object == Object::Empty {
                                    self.grid[indexes.1][indexes.0] = self.grid[j][i];
                                    self.grid[j][i] = Object::Empty;
                                    already_moved.push(indexes);
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    fn render(&self, engine: &mut Engine) {
        let size = vec2(
            WINDOW_SIZE.y / self.grid.len() as f64,
            WINDOW_SIZE.x / self.grid[0].len() as f64,
        );
        for j in 0..self.grid.len() {
            for i in 0..self.grid.len() {
                if self.grid[j][i] != Object::Empty {
                    let pos = vec2(i as f64 * size.x, j as f64 * size.y);
                    engine.render_texture(&rect_vec(pos, size), &self.textures[0]);
                }
            }
        }

        // engine.render_texture(&self.left_paddle.rect, &self.textures[0]);
    }
}
