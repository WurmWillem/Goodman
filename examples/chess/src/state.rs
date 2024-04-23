use crate::{
    moves::*,
    types::{Kind, Piece, Side, Turn},
    SQUARE,
};
use goodman::prelude::*;

pub struct State {
    turn: Turn,
    pub selected_piece_moves: Vec<(usize, usize)>,
    selected_piece_index: (usize, usize),
}
impl State {
    pub fn new() -> Self {
        Self {
            turn: Turn::White,
            selected_piece_moves: vec![],
            selected_piece_index: (0, 0),
        }
    }
    pub fn check_for_move(&mut self, pieces: &mut Vec<Vec<Piece>>, input: &Input) {
        let clicked_coords = get_clicked_square_coords(&input);

        if let Some(coords) = clicked_coords {
            let (i, j) = coords;

            let mut moved_a_piece = false;
            crate::types::deselect_every_piece(pieces);

            // make move if clicked square is in moves
            if self.selected_piece_moves.len() > 0 {
                let index = self.selected_piece_index;
                let side_clicked = pieces[j][i].side;
                let side_original = pieces[index.0][index.1].side;

                for m in &self.selected_piece_moves {
                    if (j, i) == *m
                        && (side_clicked == Side::opposite(&side_original)
                            || side_clicked == Side::None
                            || side_original == Side::None)
                    {
                        make_move(pieces, index, *m);
                        self.turn = Turn::opposite(&self.turn);
                        self.selected_piece_moves = vec![];
                        moved_a_piece = true;
                        break;
                    }
                }
            }

            // select piece and generate moves for it
            let side = pieces[j][i].side;
            if ((side == Side::White && self.turn == Turn::White)
                || (side == Side::Black && self.turn == Turn::Black))
                && pieces[j][i].kind != Kind::None
                && !moved_a_piece
            {
                pieces[j][i].selected = true;
                // pieces[j][i].moves = calculate_moves(pieces, &pieces[j][i], j, i);
                self.selected_piece_moves = calculate_moves(pieces, &pieces[j][i], j, i);
                self.selected_piece_index = (j, i);
            }
        }
    }
}

fn get_clicked_square_coords(input: &Input) -> Option<(usize, usize)> {
    if input.is_button_pressed(Button::LeftMouse) {
        let coords = (
            (input.get_cursor_pos().x as f32 / SQUARE) as usize,
            (input.get_cursor_pos().y as f32 / SQUARE) as usize,
        );
        // println!("{:?}", coords);
        return Some(coords);
    }
    None
}
