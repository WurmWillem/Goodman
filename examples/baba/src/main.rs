use goodman::prelude::*;
use other::{
    make_usize_tup, AllCharacterData, Character, Direction, Noun, NounPropCombi, Object, Property,
};

mod other;

pub const WINDOW_SIZE: Vec2 = vec2(1200., 750.); //1500x1000
const GRID_SIZE: (usize, usize) = (20, 14);

fn main() {
    block_on(run());
}

async fn run() {
    let event_loop = EventLoop::new();

    let mut engine = Engine::new(WINDOW_SIZE, &event_loop, true).await;
    engine.set_target_fps(Some(144));
    // engine.set_target_tps(Some(1000000));
    // engine.enable_feature(Feature::EngineUi);
    //engine.enable_feature(Feature::AverageTPS(0.1));

    let game = Game::new(&mut engine);

    engine.enter_loop(game, event_loop);
}

struct Game {
    grid: Vec<Vec<Object>>,
    character_data: AllCharacterData,
    noun_prop_combi: Vec<NounPropCombi>,
    textures: Vec<Texture>,
}
impl Manager for Game {
    fn new(engine: &mut Engine) -> Self {
        let bytes = include_bytes!("assets/is.png");
        let is_tex = engine.create_texture(bytes, "is").unwrap();

        let bytes = include_bytes!("assets/you.png");
        let you_tex = engine.create_texture(bytes, "you").unwrap();
        let bytes = include_bytes!("assets/win.png");
        let win_tex = engine.create_texture(bytes, "win").unwrap();

        let bytes = include_bytes!("assets/baba c.png");
        let baba_c_tex = engine.create_texture(bytes, "baba c").unwrap();
        let bytes = include_bytes!("assets/baba.png");
        let baba_tex = engine.create_texture(bytes, "baba").unwrap();
        let bytes = include_bytes!("assets/flag c.png");
        let flag_c_tex = engine.create_texture(bytes, "flag c").unwrap();
        let bytes = include_bytes!("assets/flag.png");
        let flag_tex = engine.create_texture(bytes, "flag").unwrap();

        let bytes = include_bytes!("assets/floor.png");
        let floor_tex = engine.create_texture(bytes, "floor").unwrap();

        let mut grid = vec![];
        for _ in 0..GRID_SIZE.1 {
            let mut row = vec![];
            for _ in 0..GRID_SIZE.0 {
                row.push(Object::Empty);
            }
            grid.push(row);
        }

        grid[2][2] = Object::Character(Character::Baba);
        grid[3][3] = Object::Character(Character::Flag);

        grid[5][3] = Object::Noun(Noun::Baba);
        grid[5][4] = Object::Is;
        grid[5][5] = Object::Property(Property::Win);

        grid[7][3] = Object::Noun(Noun::Flag);
        grid[7][4] = Object::Is;
        grid[7][5] = Object::Property(Property::You);

        grid[0][7] = Object::Noun(Noun::Baba);
        grid[0][8] = Object::Is;
        grid[0][9] = Object::Property(Property::You);

        Self {
            grid,
            character_data: AllCharacterData::new(),
            noun_prop_combi: vec![],
            textures: vec![
                is_tex, you_tex, win_tex, baba_tex, flag_tex, baba_c_tex, flag_c_tex, floor_tex,
            ],
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
        for j in 0..self.grid.len() {
            for i in 0..self.grid[0].len() {
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
                        let mut moves_to_make = vec![((i, j), next_grid_pos)];

                        loop {
                            let next_pos = moves_to_make[moves_to_make.len() - 1].1;
                            if self.grid.get(next_pos.1).is_none()
                                || self.grid[next_pos.1].get(next_pos.0).is_none()
                            {
                                break;
                            }

                            if self.grid[next_pos.1][next_pos.0] == Object::Empty {
                                for m in moves_to_make.iter().rev() {
                                    moves.push(*m);
                                }
                                break;
                            } else {
                                let current_pos = next_pos;
                                let next_pos = make_usize_tup(where_to_move, current_pos);
                                if let Object::Character(char) =
                                    self.grid[current_pos.1][current_pos.0]
                                {
                                    if self.character_data.get_if_enabled(
                                        char.get_corresponding_noun(),
                                        Property::Win,
                                    ) {
                                        println!("Win!");
                                    }
                                }
                                moves_to_make.push((current_pos, next_pos));
                            }
                        }
                    }
                }
            }
        }
        for mov in &moves {
            if self.grid[mov.0 .1][mov.0 .0] != Object::Empty {
                self.move_object(*mov);
            }
        }

        if !moves.is_empty() {
            self.update_character_data();
        }
    }

    fn render(&self, engine: &mut Engine) {
        let size = vec2(
            WINDOW_SIZE.x / self.grid[0].len() as f64,
            WINDOW_SIZE.y / self.grid.len() as f64,
        );

        for j in 0..self.grid.len() {
            for i in 0..self.grid[0].len() {
                let pos = vec2(i as f64 * size.x, j as f64 * size.y);

                engine.render_texture(&rect_vec(pos, size), &self.textures[7]);

                if self.grid[j][i] != Object::Empty {
                    let index = self.grid[j][i].get_tex_index();
                    engine.render_texture(&rect_vec(pos, size), &self.textures[index]);
                };
            }
        }
        let pos = vec2(15. * size.x, 5. * size.y);
        engine.render_texture(&rect_vec(pos, size), &self.textures[3]);
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
                println!("deleted {:?} is {:?}", npc.n.1, npc.p.1);
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

    fn update_npcs(
        &mut self,
        noun: Option<Noun>,
        property: Option<Property>,
        (i, j): (usize, usize),
        dir: Direction,
    ) {
        if let Some(noun) = noun {
            if let Some(property) = property {
                let npc;
                if dir == Direction::Hor {
                    npc = NounPropCombi::new(j, (i - 1, noun), (i + 1, property), dir);
                } else {
                    npc = NounPropCombi::new(i, (j - 1, noun), (j + 1, property), dir)
                }
                if !self.noun_prop_combi.contains(&npc) {
                    self.character_data
                        .set_char_to_property(noun, property, true);
                    self.noun_prop_combi.push(npc);
                    println!("created {:?} is {:?}", npc.n.1, npc.p.1);
                }
            }
        }
    }

    fn move_object(&mut self, ((i, j), (next_i, next_j)): ((usize, usize), (usize, usize))) {
        self.grid[next_j][next_i] = self.grid[j][i];
        self.grid[j][i] = Object::Empty;
    }
}
