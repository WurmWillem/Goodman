#[derive(Debug, Clone, PartialEq)]
pub struct Piece {
    pub kind: Kind,
    pub selected: bool,
    pub side: Side,
}
impl Piece {
    pub fn new(kind: Kind, side: Side) -> Self {
        Self {
            kind,
            selected: false,
            side,
        }
    }
    pub fn new_empty() -> Self {
        Self {
            kind: Kind::None,
            selected: false,
            side: Side::None,
        }
    }
    pub fn get_tex_index(&self) -> usize {
        let side_increment = if self.side == Side::Black { 6 } else { 0 };
        match self.kind {
            Kind::None => 0 + side_increment,
            Kind::Pawn(_) => 0 + side_increment,
            Kind::Knight => 1 + side_increment,
            Kind::Bishop => 2 + side_increment,
            Kind::Rook(_) => 3 + side_increment,
            Kind::Queen => 4 + side_increment,
            Kind::King(_) => 5 + side_increment,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Kind {
    None,
    Pawn(bool), // bool = has just moved 2 spaces, so can be en passanted
    Knight,
    Bishop,
    Rook(bool), // bool = has moved, so cannot castle anymore
    Queen,
    King(bool), // bool = has moved, so cannot castle anymore
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Turn {
    White,
    Black,
}
impl Turn {
    pub fn opposite(turn: &Turn) -> Turn {
        if *turn == Turn::White {
            Turn::Black
        } else if *turn == Turn::Black {
            Turn::White
        } else {
            panic!("tried to opposite None");
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Side {
    None,
    White,
    Black,
}
impl Side {
    pub fn opposite(&self) -> Self {
        match *self {
            Side::White => Side::Black,
            Side::Black => Side::White,
            _ => panic!("tried to opposite None"),
        }
    }

    pub fn as_turn_color(turn: Turn) -> Side {
        match turn {
            Turn::White => Side::White,
            Turn::Black => Side::Black,
        }
    }
}
