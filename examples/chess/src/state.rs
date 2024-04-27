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

            crate::types::deselect_every_piece(board);

            // make move if clicked square is in moves
            if self.selected_piece_moves.len() > 0 {
                /*if make_move_if_needed(
                    j,
                    i,
                    self.selected_piece_index,
                    &self.selected_piece_moves,
                    board,
                ) {
                    println!("checkmate");
                } else {
                    // no legal moves found, no checkmate
                    self.turn = Turn::opposite(&self.turn);
                    self.selected_piece_moves = vec![];
                    return;
                }*/
                let from = self.selected_piece_index;
                let side_to = board[j][i].side;
                let side_from = board[from.0][from.1].side;

                for to in &self.selected_piece_moves {
                    if (j, i) == *to
                        && (side_to == side_from.opposite() || side_to == Side::None)
                    {
                        for j in 0..8 {
                            for i in 0..8 {
                                // pieces[j][i].selected = false;
                                if let Kind::Pawn(true) = board[j][i].kind {
                                    board[j][i].kind = Kind::Pawn(false);
                                }
                            }
                        }
                        make_move(board, from, *to);

                        if is_checkmate(board, *to) {
                            println!("checkmate");
                        } 

                        self.turn = Turn::opposite(&self.turn);
                        self.selected_piece_moves = vec![];
                        //checkmate
                        return;
                    }
                }
            }

            if board[j][i].kind == Kind::None {
                return;
            }
            board[j][i].selected = true;
            // select piece and generate legal moves for it
            if board[j][i].side == Side::as_turn_color(self.turn) {
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

                self.selected_piece_moves = legal_moves;
                self.selected_piece_index = (j, i);
            } else {
                self.selected_piece_moves = vec![];
            }
        }
    }
}

fn is_checkmate(board: &Vec<Vec<Piece>>, to: (usize, usize)) -> bool {
    let mut legal_moves = vec![];
    // check for checkmate
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
                    // legal move found, no checkmate
                    legal_moves.push(*pseudo);
                }
            }
        }
    }
    if legal_moves.len() == 0 {
        println!("checkmate");
        return true;
    }
    false
}

fn make_move_if_needed(
    j: usize,
    i: usize,
    selected_index: (usize, usize),
    selected_moves: &Vec<(usize, usize)>,
    board: &mut Vec<Vec<Piece>>,
) -> bool {
    let side_clicked = board[j][i].side;
    let side_original = board[selected_index.0][selected_index.1].side;
    let mut move_made = false;

    for m in selected_moves {
        if (j, i) == *m && (side_clicked == side_original.opposite() || side_clicked == Side::None)
        {
            for j in 0..8 {
                for i in 0..8 {
                    // pieces[j][i].selected = false;
                    if let Kind::Pawn(true) = board[j][i].kind {
                        board[j][i].kind = Kind::Pawn(false);
                    }
                }
            }
            make_move(board, selected_index, *m);
            move_made = true;

            // check for checkmate
            for j in 0..8 {
                for i in 0..8 {
                    if board[j][i].side == board[m.0][m.1].side {
                        continue;
                    }
                    let pseudo_legal_moves = calculate_moves(board, j, i);

                    for pseudo in &pseudo_legal_moves {
                        let mut board_clone = board.clone();
                        make_move(&mut board_clone, (j, i), *pseudo);

                        if !king_of_side_can_be_taken(&board_clone, board[j][i].side) {
                            // legal move found, no checkmate
                            return false;
                        }
                    }
                }
            }
            //checkmate
            return true;
        }
    }
    false
}

fn king_of_side_can_be_taken(board: &Vec<Vec<Piece>>, side: Side) -> bool {
    for j in 0..8 {
        for i in 0..8 {
            if board[j][i].side == side.opposite() {
                for mov in &calculate_moves(board, j, i) {
                    if board[mov.0][mov.1].kind == Kind::King {
                        return true;
                    }
                }
            }
        }
    }
    false
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
