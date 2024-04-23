//WARNING: some of this code is really old, proceed with caution

use goodman::prelude::*;
use piece_data::Piece;
use pieces::Kind;
use state::{Side, State};
use textures::get_textures;

mod consts;
mod piece_data;
mod pieces;
mod state;
mod textures;

pub const SCREENSIZE: f32 = 900.0;
pub const SQUARE: f32 = SCREENSIZE / 8.0;

fn main() {
    block_on(run())
}

async fn run() {
    let event_loop = EventLoop::new();
    let mut engine = EngineBuilder::new(vec2(SCREENSIZE, SCREENSIZE))
        // .show_engine_ui()
        .with_window_title("Chess".to_string())
        .with_target_fps(144)
        .build(&event_loop)
        .await;

    let chess = Chess::new(&mut engine);
    engine.start_loop(chess, event_loop)
}

struct Chess {
    pieces: Vec<Vec<Piece>>,
    state: State,
    textures: Vec<Texture>,
}
impl Manager for Chess {
    fn new(engine: &mut Engine) -> Self {
        let textures = get_textures(engine);

        let  none = vec![Piece::new_empty(); 8];
        let mut pieces = vec![none; 8];

        let white_pieces = vec![
            Piece::new(Kind::Rook, Side::White),
            Piece::new(Kind::Knight, Side::White),
            Piece::new(Kind::Bishop, Side::White),
            Piece::new(Kind::Queen, Side::White),
            Piece::new(Kind::King, Side::White),
            Piece::new(Kind::Bishop, Side::White),
            Piece::new(Kind::Knight, Side::White),
            Piece::new(Kind::Rook, Side::White),
        ];

        let black_pieces = vec![
            Piece::new(Kind::Rook, Side::Black),
            Piece::new(Kind::Knight, Side::Black),
            Piece::new(Kind::Bishop, Side::Black),
            Piece::new(Kind::Queen, Side::Black),
            Piece::new(Kind::King, Side::Black),
            Piece::new(Kind::Bishop, Side::Black),
            Piece::new(Kind::Knight, Side::Black),
            Piece::new(Kind::Rook, Side::Black),
        ];

        for j in 0..8 {
            for i in 0..8 {
                if j == 6 {
                    pieces[j][i] = Piece::new(Kind::Pawn, Side::White);
                } else if j == 1 {
                    pieces[j][i] = Piece::new(Kind::Pawn, Side::Black);
                } else if j == 7 {
                    pieces[j] = white_pieces.to_vec();
                } else if j == 0 {
                    pieces[j] = black_pieces.to_vec();
                }
            }
        }
        Self {
            pieces,
            state: State::new(),
            textures,
        }
    }
    fn update(&mut self, _frame_time: f64, input: &Input, _sound: &mut Sound) {
        self.state.check_for_move(&mut self.pieces, input);
    }
    fn render(&mut self, engine: &mut Engine) {
        self.draw_board(engine);

        for j in 0..8 {
            for i in 0..8 {
                if self.pieces[j][i].kind == Kind::None {
                    continue;
                }
                let index = self.pieces[j][i].get_tex_index();
                let rect = rect32(
                    i as f32 * SQUARE + 1.0,
                    j as f32 * SQUARE + 3.0,
                    SQUARE - 3.0,
                    SQUARE - 3.0,
                );
                engine.render_texture(rect, &self.textures[index]);
            }
        }
    }
}
impl Chess {
    fn draw_board(&self, engine: &mut Engine) {
        for j in 0..8 {
            for i in 0..8 {
                let rect = rect32(i as f32 * SQUARE, j as f32 * SQUARE, SQUARE, SQUARE);

                if self.pieces[j][i].selected {
                    engine.render_texture(rect, &self.textures[14]);
                    continue;
                }
                let index = if (j + i) % 2 == 0 { 12 } else { 13 };

                engine.render_texture(rect, &self.textures[index]);
            }
        }
        for j in 0..8 {
            for i in 0..8 {
                let moves = &self.pieces[j][i].moves;
                if moves.len() > 0 {
                    for m in moves {
                        let rect = rect32(
                            m.1 as f32 * SQUARE + SQUARE * 0.345,
                            m.0 as f32 * SQUARE + SQUARE * 0.345,
                            SQUARE * 0.33,
                            SQUARE * 0.33,
                        );
                        engine.render_texture(rect, &self.textures[15]);
                    }
                }
            }
        }
    }
}
