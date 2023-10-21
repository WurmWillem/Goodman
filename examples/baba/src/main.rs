use goodman::prelude::*;
use level::Level;
use other::{
    get_source_from_index, AllCharacterData, Move, NounPropCombi, Object, Property, VecPos,
};

mod game;
mod level;
mod other;

pub const WINDOW_SIZE: Vec32 = vec2(1200., 750.); //1500x1000
const GRID_SIZE: (usize, usize) = (20, 14);

fn main() {
    block_on(run());
}

async fn run() {
    let event_loop = EventLoop::new();

    let mut engine = EngineBuilder::new(WINDOW_SIZE)
        .with_target_fps(144)
        // .show_engine_ui()
        .with_window_title("Baba".to_string())
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
    baba_anim: Animation<u32>,
}
impl Manager for Game {
    fn new(engine: &mut Engine) -> Self {
        // engine.use_sound(false);

        let background_music = engine
            .create_sound_source("examples/baba/src/assets/background.wav")
            .unwrap();
        engine
            .play_sound(background_music.convert_samples().repeat_infinite())
            .unwrap();

        let source = engine
            .create_sound_source("examples/baba/src/assets/pop.mp3")
            .unwrap()
            .buffered();

        let mut textures = vec![];
        create_textures!(engine, textures, "assets/sheet.png");

        let mut grid = vec![vec![]];
        let current_level = Level::Level4;
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

    fn update(&mut self, delta_t: f64, input: &Input, sound: &mut Sound) {
        self.baba_anim.update(delta_t as f32);

        macro_rules! load_level_if_button_pressed {
            ($button: ident, $level: ident) => {
                if input.is_button_pressed(Button::$button) {
                    self.current_level = Level::$level;
                    self.current_level.load_level(&mut self.grid);
                    self.reset();
                }
            };
        }
        load_level_if_button_pressed!(One, Level1);
        load_level_if_button_pressed!(Two, Level2);
        load_level_if_button_pressed!(Three, Level3);
        load_level_if_button_pressed!(Four, Level4);

        if input.is_button_pressed(Button::R) {
            self.current_level.load_level(&mut self.grid);
            self.reset();
        }

        if input.is_button_pressed(Button::M) {
            sound.use_sound(!sound.uses_sound());
        }

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
        if where_to_move == (0, 0) {
            return;
        }

        let mut moves: Vec<Move> = vec![];
        'outer: for j in 0..self.grid.len() {
            for i in 0..self.grid[0].len() {
                let char = match self.grid[j][i] {
                    Object::Character(char) => char,
                    _ => continue,
                };
                if !self.character_data.is_you(char) {
                    continue;
                }

                let from = VecPos::new((i, j));
                let next_grid_pos = VecPos::add_i32_tuple(from, where_to_move);
                let mov = Move::new(from, next_grid_pos);
                let mut moves_to_make = vec![mov];

                loop {
                    let to = moves_to_make[moves_to_make.len() - 1].to;
                    if self.grid.get(to.j).is_none() || self.grid[to.j].get(to.i).is_none() {
                        break;
                    }

                    if self.grid[to.j][to.i] == Object::Empty {
                        for i in (0..moves_to_make.len()).rev() {
                            moves.push(moves_to_make[i]);
                            let to = moves_to_make[i].to;
                            if let Object::Character(char) = self.grid[to.j][to.i] {
                                if self.character_data.is_you(char) {
                                    break;
                                }
                            }
                        }
                        break;
                    } else {
                        let from = to;
                        let to = VecPos::add_i32_tuple(from, where_to_move);

                        macro_rules! do_action_after_checking_property {
                            ($char: ident, $property: ident, $($action: expr)*) => {
                                if self.character_data.get_if_enabled(
                                    $char.get_corresponding_noun(),
                                    Property::$property,
                                ) {
                                    $($action;)*
                                }
                            };
                        }

                        if let Object::Character(char) = self.grid[from.j][from.i] {
                            do_action_after_checking_property!(char, Win, self.win() break 'outer);
                            do_action_after_checking_property!(char, Stop, break);
                            do_action_after_checking_property!(
                                char,
                                Defeat,
                                if moves_to_make.len() < 2 {
                                    self.grid[j][i] = Object::Empty;
                                    break;
                                }
                            );
                        }

                        moves_to_make.push(Move::new(from, to));
                    }
                }
            }
        }

        if where_to_move.0 != 0 {
            moves.sort_by(|a, b| a.from.i.cmp(&b.from.i));
        } else {
            moves.sort_by(|a, b| a.from.j.cmp(&b.from.j));
        }
        if where_to_move.0 == -1 || where_to_move.1 == -1 {
            for mov in &moves {
                if self.grid[mov.from.j][mov.from.i] != Object::Empty {
                    self.move_object(*mov);
                }
            }
        } else {
            for mov in moves.iter().rev() {
                if self.grid[mov.from.j][mov.from.i] != Object::Empty {
                    self.move_object(*mov);
                }
            }
        }

        if !moves.is_empty() {
            if !self.is_you_char_exists() {
                self.current_level.load_level(&mut self.grid);
                self.reset();
            }

            self.update_character_data();
            sound
                .play_sound(self.source.clone().convert_samples())
                .unwrap();
        }
    }

    fn render(&mut self, engine: &mut Engine) {
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
