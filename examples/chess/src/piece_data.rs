use crate::{consts::RAYWHITE, pieces::Piece, state::Side};
use goodman::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Data {
    pub index: usize,
    pub selected: bool,
    pub color: Color,
    pub moves: Vec<(usize, usize)>,
    pub side: Side,
}
impl Data {
    pub fn new(index: usize, side: Side) -> Self {
        Self {
            index,
            selected: false,
            color: RAYWHITE,
            moves: Vec::new(),
            side,
        }
    }

    pub fn get_new(piece: &Piece) -> Self {
        let index = match piece {
            Piece::None => 13,
            Piece::Pawn(data)
            | Piece::Knight(data)
            | Piece::Bishop(data)
            | Piece::Rook(data)
            | Piece::Queen(data)
            | Piece::King(data) => data.index,
        };
        Self {
            index,
            selected: false,
            color: RAYWHITE,
            moves: Vec::new(),
            side: Data::get_side(piece),
        }
    }

    pub fn change_value(piece: &Piece, dat: Data) -> Piece {
        return match &piece {
            Piece::Pawn(_) => Piece::Pawn(dat),
            Piece::Knight(_) => Piece::Knight(dat),
            Piece::Bishop(_) => Piece::Bishop(dat),
            Piece::Rook(_) => Piece::Rook(dat),
            Piece::Queen(_) => Piece::Queen(dat),
            Piece::King(_) => Piece::King(dat),
            _ => Piece::None,
        };
    }

    pub fn get_index(piece: &Piece) -> usize {
        match piece {
            Piece::None => panic!(),
            Piece::Pawn(data)
            | Piece::Knight(data)
            | Piece::Bishop(data)
            | Piece::Rook(data)
            | Piece::Queen(data)
            | Piece::King(data) => data.index,
        }
    }

    pub fn get_if_selected(piece: &Piece) -> bool {
        return match &piece {
            Piece::Pawn(dat)
            | Piece::Knight(dat)
            | Piece::Bishop(dat)
            | Piece::Rook(dat)
            | Piece::Queen(dat)
            | Piece::King(dat) => dat.selected,
            _ => false,
        };
    }

    pub fn get_moves(piece: &Piece) -> Vec<(usize, usize)> {
        return match piece {
            Piece::Pawn(dat)
            | Piece::Knight(dat)
            | Piece::Bishop(dat)
            | Piece::Rook(dat)
            | Piece::Queen(dat)
            | Piece::King(dat) => dat.moves.to_vec(),
            _ => Vec::new(),
        };
    }

    pub fn get_side(piece: &Piece) -> Side {
        return match piece {
            Piece::Pawn(dat)
            | Piece::Knight(dat)
            | Piece::Bishop(dat)
            | Piece::Rook(dat)
            | Piece::Queen(dat)
            | Piece::King(dat) => dat.side.clone(),
            _ => Side::None,
        };
    }
}
impl Default for Data {
    fn default() -> Self {
        Self {
            index: 0,
            selected: false,
            color: RAYWHITE,
            moves: Vec::new(),
            side: Side::None,
        }
    }
}
