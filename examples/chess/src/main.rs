//WARNING: some of this code is really old, proceed with caution

use crate::types::Kind;
use goodman::prelude::*;
use state::State;
use types::{Piece, Side};

mod consts;
mod moves;
mod state;
mod types;

pub const SCREENSIZE: f32 = 900.0;
pub const SQUARE: f32 = SCREENSIZE / 8.0;

fn main() {
    block_on(run())
}

async fn run() {
    let event_loop = EventLoop::new();
    let mut engine = EngineBuilder::new(vec2(SCREENSIZE, SCREENSIZE))
        //        .show_engine_ui()
        .with_background_color(Color::BLUE)
        .with_window_title("Chess".to_string())
        .with_target_fps(144)
        .build(&event_loop)
        .await;

    let chess = Chess::new(&mut engine);
    engine.start_loop(chess, event_loop)
}

struct Chess {
    board: Vec<Vec<Piece>>,
    state: State,
    textures: Vec<Texture>,
}
impl Manager for Chess {
    fn new(engine: &mut Engine) -> Self {
        let textures = get_textures(engine);

        let none = vec![Piece::new_empty(); 8];
        let mut pieces = vec![none; 8];

        let white_pieces = create_row_of_pieces(Side::White);
        let black_pieces = create_row_of_pieces(Side::Black);

        for j in 0..8 {
            for i in 0..8 {
                if j == 6 {
                    pieces[j][i] = Piece::new(Kind::Pawn(false), Side::White);
                } else if j == 1 {
                    pieces[j][i] = Piece::new(Kind::Pawn(false), Side::Black);
                } else if j == 7 {
                    pieces[j] = white_pieces.to_vec();
                } else if j == 0 {
                    pieces[j] = black_pieces.to_vec();
                }
            }
        }
        Self {
            board: pieces,
            state: State::new(),
            textures,
        }
    }
    fn update(&mut self, _frame_time: f64, input: &Input, _sound: &mut Sound) {
        self.state.update_based_on_click(&mut self.board, input);
    }
    fn render(&mut self, engine: &mut Engine) {
        self.render_board(engine);

        for j in 0..8 {
            for i in 0..8 {
                if self.board[j][i].kind == Kind::None {
                    continue;
                }
                let index = self.board[j][i].get_tex_index();
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
    fn render_board(&self, engine: &mut Engine) {
        /*let moves = &self.state.selected_piece_moves;
        if moves.len() > 0 {
            for m in moves {
                // if self.pieces[m.0][m.1].kind == Kind::None {
                    let rect = rect32(
                        m.1 as f32 * SQUARE + SQUARE * 0.345,
                        m.0 as f32 * SQUARE + SQUARE * 0.345,
                        SQUARE * 0.33,
                        SQUARE * 0.33,
                    );
                    engine.render_texture(rect, &self.textures[15]);
                // }
            }
        }*/
        for j in 0..8 {
            for i in 0..8 {
                let rect = rect32(i as f32 * SQUARE, j as f32 * SQUARE, SQUARE, SQUARE);

                if self.board[j][i].selected {
                    engine.render_texture(rect, &self.textures[14]);
                    continue;
                }
                let index = if (j + i) % 2 == 0 { 12 } else { 13 };

                engine.render_texture(rect, &self.textures[index]);
            }
        }

        let moves = &self.state.selected_piece_moves;
        if moves.len() > 0 {
            for m in moves {
                // if self.pieces[m.0][m.1].kind == Kind::None {
                let rect = rect32(
                    m.1 as f32 * SQUARE + SQUARE * 0.345,
                    m.0 as f32 * SQUARE + SQUARE * 0.345,
                    SQUARE * 0.33,
                    SQUARE * 0.33,
                );
                engine.render_texture(rect, &self.textures[15]);
                // }
            }
        }
    }
}

fn get_textures(engine: &mut Engine) -> Vec<Texture> {
    let mut textures = vec![];
    create_textures!(engine, textures, 
        "assets/white-pawn.png" "assets/white-knight.png" "assets/white-bishop.png" "assets/white-rook.png" "assets/white-queen.png" "assets/white-king.png" 
        "assets/black-pawn.png" "assets/black-knight.png" "assets/black-bishop.png" "assets/black-rook.png" "assets/black-queen.png" "assets/black-king.png"
        "assets/white_square.png" "assets/green_square.png" "assets/selected.png" "assets/move.png");
    textures
}

fn create_row_of_pieces(side: Side) -> Vec<Piece> {
    vec![
        Piece::new(Kind::Rook, side),
        Piece::new(Kind::Knight, side),
        Piece::new(Kind::Bishop, side),
        Piece::new(Kind::Queen, side),
        Piece::new(Kind::King, side),
        Piece::new(Kind::Bishop, side),
        Piece::new(Kind::Knight, side),
        Piece::new(Kind::Rook, side),
    ]
}
