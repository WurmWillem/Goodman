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
    //engine.set_target_tps(Some(1000000));
    //engine.enable_feature(Feature::EngineUi);
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
    fn set_char_to_property(&mut self, noun: Noun, property: Property, enable: bool) {
        let i = if enable { 1 } else { -1 };
        match noun {
            Noun::Kirb => match property {
                Property::You => {
                    self.kirb.is_you_counter = (self.kirb.is_you_counter as i32 + i) as usize;
                    self.kirb.is_you = self.kirb.is_you_counter > 0
                }
            },
        }
    }
    /*fn get_if_enabled(&self, noun: Noun, property: Property) -> bool {
        match noun {
            Noun::Kirb => match property {
                Property::You => self.kirb.is_you,
            },
        }
    }*/
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
    Hor,
    Ver,
}

struct CharacterData {
    is_you: bool,
    is_you_counter: usize,
}
impl CharacterData {
    fn new() -> Self {
        Self {
            is_you: false,
            is_you_counter: 0,
        }
    }
}
struct Game {
    grid: Vec<Vec<Object>>,
    character_data: AllCharacterData,
    noun_prop_combi: Vec<NounPropCombi>,
    textures: Vec<Texture>,
}
impl Manager for Game {
    fn new(engine: &mut Engine) -> Self {
        let bytes = include_bytes!("assets/baba c.png");
        let baba_c_tex = engine.create_texture(bytes, "baba c").unwrap();
        let bytes = include_bytes!("assets/check.png");
        let baba_tex = engine.create_texture(bytes, "baba").unwrap();
        let bytes = include_bytes!("assets/you.png");
        let is_tex = engine.create_texture(bytes, "is").unwrap();
        let bytes = include_bytes!("assets/check.png");
        let you_tex = engine.create_texture(bytes, "you").unwrap();

        let mut grid = vec![];
        for _ in 0..GRID_SIZE.1 {
            let mut row = vec![];
            for _ in 0..GRID_SIZE.0 {
                row.push(Object::Empty);
            }
            grid.push(row);
        }
        // grid[0][0] = Object::Character(Character::Kirb);
        grid[3][5] = Object::Character(Character::Kirb);

        grid[5][0] = Object::Noun(Noun::Kirb);
        grid[5][1] = Object::Is;
        grid[5][2] = Object::Property(Property::You);

        grid[5][4] = Object::Noun(Noun::Kirb);
        grid[6][4] = Object::Is;
        grid[7][4] = Object::Property(Property::You);

        Self {
            grid,
            character_data: AllCharacterData::new(),
            noun_prop_combi: vec![],
            textures: vec![baba_c_tex, baba_tex, is_tex, you_tex],
        }
    }

    fn start(&mut self) {
        self.update_character_data();
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
        for push in &pushes {
            self.move_object(*push);
        }
        for mov in &moves {
            self.move_object(*mov);
        }

        if !pushes.is_empty() {
            self.update_character_data();
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
                } else if self.grid[j][i] == Object::Noun(Noun::Kirb) {
                    index = 1;
                } else if self.grid[j][i] == Object::Is {
                    index = 2
                } else if self.grid[j][i] == Object::Property(Property::You) {
                    index = 3
                } else {
                    index = 99;
                }
                engine.render_texture(&rect_vec(pos, size), &self.textures[index]);
            }
        }
        /*let pos = vec2(6. * size.x, 5. * size.y);
        engine.render_texture(&rect_vec(pos, size), &self.textures[0]);
        */
    }
}
impl Game {
    fn update_character_data(&mut self) {
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
                self.update_npcs(noun, property, (i, j), Direction::Hor);

                let mut noun = None;
                if let Some(row) = self.grid.get(j - 1) {
                    if let Object::Noun(n) = row[i] {
                        noun = Some(n);
                    }
                }
                let mut property = None;
                if let Some(row) = self.grid.get(j + 1) {
                    if let Object::Property(p) = row[i] {
                        property = Some(p);
                    }
                }
                self.update_npcs(noun, property, (i, j), Direction::Ver);
            }
        }
        let mut i = 0;
        let mut to_remove = vec![];
        for npc in &self.noun_prop_combi {
            let is_index = if npc.p.0 > npc.n.0 {
                npc.p.0 - 1
            } else {
                npc.p.0 + 1
            };
            let mut should_delete = false;

            if npc.dir == Direction::Hor
                && (self.grid[npc.row_or_col][npc.n.0] != Object::Noun(npc.n.1)
                    || self.grid[npc.row_or_col][npc.p.0] != Object::Property(npc.p.1)
                    || self.grid[npc.row_or_col][is_index] != Object::Is)
            {
                should_delete = true;
            } else if npc.dir == Direction::Ver
                && (self.grid[npc.n.0][npc.row_or_col] != Object::Noun(npc.n.1)
                    || self.grid[npc.p.0][npc.row_or_col] != Object::Property(npc.p.1)
                    || self.grid[is_index][npc.row_or_col] != Object::Is)
            {
                should_delete = true
            }

            if should_delete {
                println!("deleted {:?}", npc);
                self.character_data
                    .set_char_to_property(npc.n.1, npc.p.1, false);
                to_remove.push(i);
            }
            i += 1;
        }
        for r in to_remove {
            self.noun_prop_combi.remove(r);
        }
    }

    fn update_npcs(&mut self, noun: Option<Noun>, property: Option<Property>, (i, j): (usize, usize), dir: Direction) {
        if let Some(noun) = noun {
            if let Some(property) = property {
                let npc;
                if dir == Direction::Hor {
                    npc = NounPropCombi::new(j, (i - 1, noun), (i + 1, property), dir);
                } else {
                    npc = NounPropCombi::new(i, (j -1, noun), (j + 1, property), dir)
                }
                if !self.noun_prop_combi.contains(&npc) {
                    self.character_data
                        .set_char_to_property(noun, property, true);
                    self.noun_prop_combi.push(npc);
                    println!("created npc");
                }
            }
        }
    }

    fn move_object(&mut self, object: ((usize, usize), (usize, usize))) {
        let (i, j) = object.0;
        let (next_i, next_j) = object.1;
        self.grid[next_j][next_i] = self.grid[j][i];
        self.grid[j][i] = Object::Empty;
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct NounPropCombi {
    row_or_col: usize,
    n: (usize, Noun),
    p: (usize, Property),
    dir: Direction,
}
impl NounPropCombi {
    fn new(row_or_col: usize, n: (usize, Noun), p: (usize, Property), dir: Direction) -> Self {
        Self { row_or_col, n, p, dir }
    }
}

fn make_usize_tup(i: (i32, i32), u: (usize, usize)) -> (usize, usize) {
    ((i.0 + u.0 as i32) as usize, (i.1 + u.1 as i32) as usize)
}
