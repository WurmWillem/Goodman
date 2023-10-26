use consts::{SCREENSIZE, SQUARE};
use goodman::prelude::*;
use piece_data::Data;
use pieces::Piece;
use state::{Side, State};
use textures::get_textures;

mod consts;
mod piece_data;
mod pieces;
mod state;
mod textures;

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

        let mut none = Vec::new();
        for _ in 0..8 {
            none.push(Piece::None);
        }

        let mut pieces = Vec::new();
        for _ in 0..8 {
            pieces.push(none.to_vec());
        }

        let white_pieces = vec![
            Piece::Rook(Data::new(3, Side::White)),
            Piece::Knight(Data::new(1, Side::White)),
            Piece::Bishop(Data::new(2, Side::White)),
            Piece::Queen(Data::new(4, Side::White)),
            Piece::King(Data::new(5, Side::White)),
            Piece::Bishop(Data::new(2, Side::White)),
            Piece::Knight(Data::new(1, Side::White)),
            Piece::Rook(Data::new(3, Side::White)),
        ];

        let black_pieces = vec![
            Piece::Rook(Data::new(9, Side::Black)),
            Piece::Knight(Data::new(7, Side::Black)),
            Piece::Bishop(Data::new(8, Side::Black)),
            Piece::Queen(Data::new(10, Side::Black)),
            Piece::King(Data::new(11, Side::Black)),
            Piece::Bishop(Data::new(8, Side::Black)),
            Piece::Knight(Data::new(7, Side::Black)),
            Piece::Rook(Data::new(9, Side::Black)),
        ];

        for j in 0..8 {
            for i in 0..8 {
                if j == 2 || j == 3 || j == 4 || j == 5 {
                    pieces[j][i] = Piece::None;
                } else if j == 6 {
                    pieces[j][i] = Piece::Pawn(Data::new(0, Side::White));
                } else if j == 1 {
                    pieces[j][i] = Piece::Pawn(Data::new(6, Side::Black));
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
                if self.pieces[j][i] == Piece::None {
                    continue;
                }
                let index = Data::get_index(&self.pieces[j][i]);
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

                if Data::get_if_selected(&self.pieces[j][i]) {
                    engine.render_texture(rect, &self.textures[14]);
                    continue;
                }
                let index = if (j + i) % 2 == 0 { 12 } else { 13 };

                engine.render_texture(rect, &self.textures[index]);
            }
        }
        for j in 0..8 {
            for i in 0..8 {
                let moves = Data::get_moves(&self.pieces[j][i]);
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
