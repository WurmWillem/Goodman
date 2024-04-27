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
    pub fn update_based_on_click(&mut self, board: &mut Vec<Vec<Piece>>, input: &Input) {
        let clicked_coords = get_clicked_square_coords(&input);

        if let Some(coords) = clicked_coords {
            let (i, j) = coords;
            deselect_every_piece(board);

            // make move if clicked square is in moves
            if self.selected_piece_moves.len() > 0 {
                let from = self.selected_piece_index;
                for to in &self.selected_piece_moves {
                    if (j, i) == *to {
                        make_pawns_not_en_passantable(board);
                        make_move(board, from, *to);

                        if is_checkmate(board, *to) {
                            println!("checkmate");
                        }

                        self.turn = Turn::opposite(&self.turn);
                        self.selected_piece_moves = vec![];
                        return;
                    }
                }
            }
            // no moves made
            self.selected_piece_moves = vec![];
            // return if square clicked is empty
            if board[j][i].kind == Kind::None {
                return;
            }
            board[j][i].selected = true;

            if board[j][i].side == Side::as_turn_color(self.turn) {
                // piece is on correct side, generate legal moves
                self.selected_piece_moves = calculate_legal_moves(board, j, i);
                self.selected_piece_index = (j, i);
            }
        }
    }
}

fn calculate_legal_moves(board: &Vec<Vec<Piece>>, j: usize, i: usize) -> Vec<(usize, usize)> {
    let pseudo_legal_moves = calculate_moves(board, j, i);
    let mut legal_moves = vec![];
    /*
    go through pseudo legal moves
    make psuedo move
    check if opp can take king
    if not, add to legal moves
     */
    for pseudo in &pseudo_legal_moves {
        let mut board_clone = board.clone();
        make_move(&mut board_clone, (j, i), *pseudo);

        if !king_of_side_can_be_taken(&board_clone, board[j][i].side) {
            legal_moves.push(*pseudo);
        }
    }
    legal_moves
}

fn is_checkmate(board: &Vec<Vec<Piece>>, to: (usize, usize)) -> bool {
    for j in 0..8 {
        for i in 0..8 {
            if board[j][i].side == board[to.0][to.1].side {
                continue;
            }
            let pseudo_legal_moves = calculate_moves(board, j, i);

            for pseudo in &pseudo_legal_moves {
                let mut board_clone = board.clone();
                make_move(&mut board_clone, (j, i), *pseudo);

                if !king_of_side_can_be_taken(&board_clone, board[j][i].side) {
                    return false;
                }
            }
        }
    }
    true
}

fn king_of_side_can_be_taken(board: &Vec<Vec<Piece>>, side: Side) -> bool {
    for j in 0..8 {
        for i in 0..8 {
            if board[j][i].side == side.opposite() {
                for mov in &calculate_moves(board, j, i) {
                    if let Kind::King(_) = board[mov.0][mov.1].kind {
                        return true;
                    }
                }
            }
        }
    }
    false
}

fn make_pawns_not_en_passantable(board: &mut Vec<Vec<Piece>>) {
    for j in 0..8 {
        for i in 0..8 {
            if let Kind::Pawn(true) = board[j][i].kind {
                board[j][i].kind = Kind::Pawn(false);
            }
        }
    }
}

fn deselect_every_piece(pieces: &mut Vec<Vec<Piece>>) {
    for j in 0..8 {
        for i in 0..8 {
            pieces[j][i].selected = false;
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
