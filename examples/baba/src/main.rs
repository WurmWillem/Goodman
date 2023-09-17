use game::Level;
use goodman::prelude::*;
use other::{
    get_source_from_index, AllCharacterData, Move, NounPropCombi, Object, Property, VecPos,
};

mod game;
mod other;

pub const WINDOW_SIZE: Vec32 = vec2(1200., 750.); //1500x1000
const GRID_SIZE: (usize, usize) = (20, 14);

fn main() {
    block_on(run());
}

async fn run() {
    // The following two lines change the working directory, you should remove them if you are not running this project with the entire goodman project included
    let root = std::path::Path::new("/home/wurmwillem/Programming/Goodman/examples/baba");
    std::env::set_current_dir(root).unwrap();

    let event_loop = EventLoop::new();

    let mut engine = EngineBuilder::new(WINDOW_SIZE)
        .set_target_fps(144)
        // .show_engine_ui()
        .build(&event_loop)
        .await;

    let game = Game::new(&mut engine);

    engine.start_loop(game, event_loop);
}

pub struct Game {
    grid: Vec<Vec<Object>>,
    character_data: AllCharacterData,
    noun_prop_combi: Vec<NounPropCombi>,
    current_level: Level,
    textures: Vec<Texture>,
    source: Buffered<SoundFile>,
    baba_anim: Animation,
}
impl Manager for Game {
    fn new(engine: &mut Engine) -> Self {
        /*let background_music = engine.create_sound_source("src/assets/music.mp3").unwrap();
        engine
            .play_sound(background_music.convert_samples())
            .unwrap();*/

        let source = engine
            .create_sound_source("src/assets/pop.mp3")
            .unwrap()
            .buffered();

        let mut textures = vec![];
        /*create_textures!(engine, textures, "assets/floor.png" "assets/is.png" "assets/baba.png" "assets/baba c.png" "assets/you.png"
        "assets/flag.png" "assets/flag c.png" "assets/win.png" "assets/wall.png" "assets/wall c.png" "assets/stop.png");*/
        create_textures!(engine, textures, "assets/sheet.png");

        let mut grid = vec![vec![]];
        let current_level = Level::Level1;
        current_level.load_level(&mut grid);

        let frames = vec![1, 11, 12]; //11
        let baba_anim = Animation::new(frames, 0.3);

        Self {
            grid,
            character_data: AllCharacterData::new(),
            noun_prop_combi: vec![],
            current_level,
            textures,
            source,
            baba_anim,
        }
    }

    fn start(&mut self) {
        self.update_character_data();
    }

    fn update(&mut self, delta_t: f64, input: &Input, sound: &Sound) {
        self.baba_anim.update(delta_t as f32);

        let mut where_to_move = (0, 0);
        if input.is_button_pressed(Button::W) {
            where_to_move.1 = -1;
        }
        if input.is_button_pressed(Button::D) {
            where_to_move.0 = 1;
        }
        if input.is_button_pressed(Button::S) {
            where_to_move.1 = 1;
        }
        if input.is_button_pressed(Button::A) {
            where_to_move.0 = -1;
        }
        if input.is_button_pressed(Button::One) {
            self.current_level = Level::Level1;
            self.current_level.load_level(&mut self.grid);
            self.reset();
        }
        if input.is_button_pressed(Button::Two) {
            self.current_level = Level::Level2;
            self.current_level.load_level(&mut self.grid);
            self.reset();
        }

        let mut moves: Vec<Move> = vec![];
        for j in 0..self.grid.len() {
            for i in 0..self.grid[0].len() {
                if let Object::Character(char) = self.grid[j][i] {
                    if self.character_data.is_you(char) {
                        if where_to_move == (0, 0) {
                            continue;
                        }

                        let i_j: VecPos = VecPos::new((i, j));

                        let mut should_continue = false;
                        for m in &moves {
                            if m.to == i_j {
                                should_continue = true;
                                break;
                            }
                        }
                        if should_continue {
                            continue;
                        }

                        let next_grid_pos = VecPos::add_i32_tuple(i_j, where_to_move);
                        let mov = Move::new(i_j, next_grid_pos);
                        let mut moves_to_make = vec![mov];

                        loop {
                            let to = moves_to_make[moves_to_make.len() - 1].to;
                            if self.grid.get(to.j).is_none() || self.grid[to.j].get(to.i).is_none()
                            {
                                break;
                            }

                            if self.grid[to.j][to.i] == Object::Empty {
                                for m in moves_to_make.iter().rev() {
                                    moves.push(*m);
                                }
                                break;
                            } else {
                                let from = to;
                                let to = VecPos::add_i32_tuple(from, where_to_move);
                                if let Object::Character(char) = self.grid[from.j][from.i] {
                                    if self.character_data.get_if_enabled(
                                        char.get_corresponding_noun(),
                                        Property::Win,
                                    ) {
                                        self.win();
                                    }
                                }
                                if let Object::Character(char) = self.grid[from.j][from.i] {
                                    if self.character_data.get_if_enabled(
                                        char.get_corresponding_noun(),
                                        Property::Stop,
                                    ) {
                                        break;
                                    }
                                }
                                moves_to_make.push(Move::new(from, to));
                            }
                        }
                    }
                }
            }
        }
        for mov in &moves {
            if self.grid[mov.from.j][mov.from.i] != Object::Empty {
                self.move_object(*mov, sound);
            }
        }

        if !moves.is_empty() {
            self.update_character_data();
        }
    }

    fn render(&self, engine: &mut Engine) {
        let size = vec2(
            WINDOW_SIZE.x / self.grid[0].len() as f32,
            WINDOW_SIZE.y / self.grid.len() as f32,
        );

        for j in 0..self.grid.len() {
            for i in 0..self.grid[0].len() {
                let pos = vec2(i as f32 * size.x, j as f32 * size.y);

                let source = rect32(26., 26., 26., 26.);
                let draw_params = DrawParams::from_source(source);
                engine.render_texture_ex(
                    rect32_vec(pos, size * 1.1),
                    &self.textures[0],
                    draw_params,
                );

                if self.grid[j][i] == Object::Character(other::Character::Baba) {
                    let source = get_source_from_index(self.baba_anim.get_current_frame());
                    let draw_params = DrawParams::from_source(source);
                    engine.render_texture_ex(rect32_vec(pos, size), &self.textures[0], draw_params);
                } else if self.grid[j][i] != Object::Empty {
                    let source = self.grid[j][i].get_source();
                    let draw_params = DrawParams::from_source(source);
                    engine.render_texture_ex(rect32_vec(pos, size), &self.textures[0], draw_params);
                }
            }
        }
    }
}
impl Game {
    fn reset(&mut self) {
        self.noun_prop_combi = vec![];
        self.character_data = AllCharacterData::new();
    }
}
