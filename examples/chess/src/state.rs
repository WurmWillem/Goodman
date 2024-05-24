use crate::{
    moves::*,
    types::{Board, Kind, Side, Turn},
    SQUARE,
};
use goodman::prelude::*;

pub struct State {
    turn: Turn,
    pub selected_piece_moves: Vec<(usize, usize)>,
    selected_piece_index: (usize, usize),
    white_in_check: bool,
    black_in_check: bool,
}
impl State {
    pub fn new() -> Self {
        Self {
            turn: Turn::White,
            selected_piece_moves: vec![],
            selected_piece_index: (0, 0),
            white_in_check: false,
            black_in_check: false,
        }
    }
    pub fn update_based_on_click(&mut self, board: &mut Board, input: &Input) {
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

                        self.white_in_check = false;
                        self.black_in_check = false;

                        let moves = calculate_moves_of_piece(board, to.0, to.1, true);
                        for m in &moves {
                            if matches!(board[m.0][m.1].kind, Kind::King(_)) {
                                if board[to.0][to.1].side == Side::White {
                                    self.black_in_check = true;
                                } else {
                                    self.white_in_check = true;
                                }
                            }
                        }
                        // println!("white is {}", self.white_in_check);
                        // println!("black is {}", self.black_in_check);

                        // depht = 2, calc move for every move of opponent, then calc every move of you
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
                /*let can_castle = if board[j][i].side == Side::White {
                    !self.white_in_check
                } else {
                    !self.black_in_check
                };*/
                let mut moves = calculate_legal_moves(board, j, i, true);
                
                if matches!(board[j][i].kind, Kind::King(_)) {
                    let mut castle_moves = vec![];
                    let mut remove_castle_moves = false;

                    for i in 0..moves.len() {
                        if (moves[i].0 as isize - i as isize).abs() > 1 {
                            castle_moves.push(i);
                            /* for enemy piece, check if moves of piece control castle squares
                             */
                            for opp_mov in get_opp_moves(board, board[j][i].side.opposite()) {
                                let inc = if i > opp_mov.1 { -1 } else { 1 };
                                let opp_i = opp_mov.1 as isize;
                                let king_i = i as isize;
                                println!("opp i = {}", opp_i);
                                println!("king i = {}", king_i);

                                if opp_mov.0 == j
                                    && (opp_i == king_i || opp_i == king_i + inc || opp_i == king_i + 2 * inc)
                                {
                                    println!("fake move found");
                                    remove_castle_moves = true;
                                }
                            }
                        }
                    }

                    if remove_castle_moves {
                        println!("ds");
                        for mov in castle_moves {
                            println!("{:?}", moves[mov]);
                            moves.remove(mov);
                        }
                    }
                }
                
                

                self.selected_piece_moves = moves;
                self.selected_piece_index = (j, i);
            }
        }
    }
}

fn get_opp_moves(board: &Board, opp_side: Side) -> Vec<(usize, usize)> {
    let mut moves = vec![];
    for j in 0..8 {
        for i in 0..8 {
            if board[j][i].side == opp_side {
                moves.append(&mut calculate_moves_of_piece(board, j, i, true))
            }
        }
    }
    moves
}

fn calculate_legal_moves(
    // depth of fn = 1, calc moves for piece, then calc all responses
    board: &Board,
    j: usize,
    i: usize,
    can_castle: bool,
) -> Vec<(usize, usize)> {
    let pseudo_legal_moves = calculate_moves_of_piece(board, j, i, can_castle);
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

fn is_checkmate(board: &Board, to: (usize, usize)) -> bool {
    // depth = 2
    for j in 0..8 {
        for i in 0..8 {
            if board[j][i].side == board[to.0][to.1].side {
                continue;
            }
            let pseudo_legal_moves = calculate_moves_of_piece(board, j, i, true);

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

fn king_of_side_can_be_taken(board: &Board, side: Side) -> bool {
    // depht = 1
    for j in 0..8 {
        for i in 0..8 {
            if board[j][i].side == side.opposite() {
                for mov in &calculate_moves_of_piece(board, j, i, true) {
                    if let Kind::King(_) = board[mov.0][mov.1].kind {
                        return true;
                    }
                }
            }
        }
    }
    false
}

fn make_pawns_not_en_passantable(board: &mut Board) {
    for j in 0..8 {
        for i in 0..8 {
            if let Kind::Pawn(true) = board[j][i].kind {
                board[j][i].kind = Kind::Pawn(false);
            }
        }
    }
}

fn deselect_every_piece(pieces: &mut Board) {
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
