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
                let selected_index = self.selected_piece_index;
                let side_clicked = pieces[j][i].side;
                let side_original = pieces[selected_index.0][selected_index.1].side;

                for m in &self.selected_piece_moves {
                    if (j, i) == *m
                        && (side_clicked == side_original.opposite()
                            || side_clicked == Side::None)
                    {
                        for j in 0..8 {
                            for i in 0..8 {
                                // pieces[j][i].selected = false;
                                if let Kind::Pawn(true) = pieces[j][i].kind {
                                    pieces[j][i].kind = Kind::Pawn(false);
                                }
                            }
                        }

                        make_move(pieces, selected_index, *m);
                        
                        let mut all_moves = vec![];
                        for j in 0..8 {
                            for i in 0..8 {
                                if pieces[j][i].side == pieces[m.0][m.1].side {
                                    all_moves.append(&mut calculate_moves(pieces, j, i));
                                }
                            }
                        }
                        println!("ds");
                        for mov in &all_moves {
                            if pieces[mov.0][mov.1].kind == Kind::King && pieces[mov.0][mov.1].side != pieces[m.0][m.1].side {
                                println!("in check {:?}", 0);
                            }
                        }

                        self.turn = Turn::opposite(&self.turn);
                        self.selected_piece_moves = vec![];

                        /*let moved_piece_moves = calculate_moves(pieces, m.0, m.1);
                        for m in moved_piece_moves {
                            if 
                        }*/

                        return;
                    }
                }
            }

            if pieces[j][i].kind == Kind::None {
                return
            }
            pieces[j][i].selected = true;
            // select piece and generate moves for it
            if pieces[j][i].side == Side::as_turn_color(self.turn) 
            {
                self.selected_piece_moves = calculate_moves(pieces, j, i);
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
