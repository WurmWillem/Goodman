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
    Character(Character),
    Noun(Noun),
    Property(Property),
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Character {
    Kirb,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Noun {
    Kirb,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Property {
    You,
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
    fn is_you(&self, char: Character) -> bool {
        match char {
            Character::Kirb => self.kirb.is_you,
        }
    }
    fn set_char_to_property(&mut self, noun: Noun, property: Property) {
        match noun {
            Noun::Kirb => match property {
                Property::You => self.kirb.is_you = true,
            },
        }
    }
}

struct CharacterData {
    pub is_you: bool,
}
impl CharacterData {
    fn new() -> Self {
        Self { is_you: false }
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
        let text_bytes = include_bytes!("assets/text.png");
        let text_tex = engine.create_texture(text_bytes, "text").unwrap();

        let mut grid = vec![];
        for _ in 0..GRID_SIZE.1 {
            let mut row = vec![];
            for _ in 0..GRID_SIZE.0 {
                row.push(Object::Empty);
            }
            grid.push(row);
        }
        grid[0][0] = Object::Character(Character::Kirb);
        grid[3][5] = Object::Character(Character::Kirb);

        grid[5][0] = Object::Noun(Noun::Kirb);
        grid[5][1] = Object::Is;
        grid[5][2] = Object::Property(Property::You);

        Self {
            grid,
            character_data: AllCharacterData::new(),
            textures: vec![kirb_tex, text_tex],
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

        let mut moves: Vec<((usize, usize), (usize, usize))> = vec![];
        let mut pushes = vec![];
        for j in 0..self.grid.len() {
            for i in 0..self.grid.len() {
                if let Object::Character(char) = self.grid[j][i] {
                    if self.character_data.is_you(char) {
                        if where_to_move == (0, 0) {
                            continue;
                        }

                        let mut should_continue = false;
                        for m in &moves {
                            if m.1 == (i, j) {
                                should_continue = true;
                                break;
                            }
                        }
                        if should_continue {
                            continue;
                        }

                        let next_grid_pos = make_usize_tup(where_to_move, (i, j));

                        if let Some(row) = self.grid.get(next_grid_pos.1) {
                            if let Some(object) = row.get(next_grid_pos.0) {
                                if *object == Object::Empty {
                                    moves.push(((i, j), next_grid_pos));
                                } else if !matches!(object, Object::Character(_)) {
                                    let next_next_grid_pos =
                                        make_usize_tup(where_to_move, next_grid_pos);

                                    if let Some(row) = self.grid.get(next_next_grid_pos.1) {
                                        if let Some(object) = row.get(next_next_grid_pos.0) {
                                            if *object == Object::Empty {
                                                pushes.push((next_grid_pos, next_next_grid_pos));
                                                moves.push(((i, j), next_grid_pos));
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        for push in pushes {
            self.move_object(push);
        }
        for mov in moves {
            self.move_object(mov);
        }

        //if already_moved.len() > 0 {
        for j in 0..self.grid.len() {
            for i in 0..self.grid.len() {
                if self.grid[j][i] != Object::Is {
                    continue;
                }
                let mut noun = None;
                if let Some(object) = self.grid[j].get(i - 1) {
                    if let Object::Noun(n) = *object {
                        noun = Some(n);
                    }
                }

                let mut property = None;
                if let Some(object) = self.grid[j].get(i + 1) {
                    if let Object::Property(p) = *object {
                        property = Some(p);
                    }
                }

                if let Some(noun) = noun {
                    if let Some(property) = property {
                        self.character_data.set_char_to_property(noun, property);
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
                if self.grid[j][i] == Object::Empty {
                    continue;
                };
                let pos = vec2(i as f64 * size.x, j as f64 * size.y);
                let index;

                if self.grid[j][i] == Object::Character(Character::Kirb) {
                    index = 0;
                } else {
                    index = 1;
                }
                engine.render_texture(&rect_vec(pos, size), &self.textures[index]);
            }
        }
    }
}
impl Game {
    fn move_object(&mut self, object: ((usize, usize), (usize, usize))) {
        let (i, j) = object.0;
        let (next_i, next_j) = object.1;
        self.grid[next_j][next_i] = self.grid[j][i];
        self.grid[j][i] = Object::Empty;
    }
}

fn make_usize_tup(i: (i32, i32), u: (usize, usize)) -> (usize, usize) {
    ((i.0 + u.0 as i32) as usize, (i.1 + u.1 as i32) as usize)
}
