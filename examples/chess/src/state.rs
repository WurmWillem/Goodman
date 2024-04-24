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
    pub fn update_based_on_click(&mut self, pieces: &mut Vec<Vec<Piece>>, input: &Input) {
        let clicked_coords = get_clicked_square_coords(&input);

        if let Some(coords) = clicked_coords {
            let (i, j) = coords;

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
                        for j in 0..8 {
                            for i in 0..8 {
                                // pieces[j][i].selected = false;
                                if let Kind::Pawn(true) = pieces[j][i].kind {
                                    pieces[j][i].kind = Kind::Pawn(false);
                                }
                            }
                        }

                        make_move(pieces, index, *m);
                        self.turn = Turn::opposite(&self.turn);
                        self.selected_piece_moves = vec![];

                        return;
                    }
                }
            }

            // select piece and generate moves for it
            let side = pieces[j][i].side;
            if ((side == Side::White && self.turn == Turn::White)
                || (side == Side::Black && self.turn == Turn::Black))
                && pieces[j][i].kind != Kind::None
            {
                pieces[j][i].selected = true;

                self.selected_piece_moves = calculate_moves(pieces, &pieces[j][i], j, i);
                self.selected_piece_index = (j, i);
            } else {
                self.selected_piece_moves = vec![];
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
