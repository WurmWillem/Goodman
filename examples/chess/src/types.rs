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
            Kind::Pawn => 0 + side_increment,
            Kind::Knight => 1 + side_increment,
            Kind::Bishop => 2 + side_increment,
            Kind::Rook => 3 + side_increment,
            Kind::Queen => 4 + side_increment,
            Kind::King => 5 + side_increment,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Kind {
    None,
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

pub fn deselect_every_piece(pieces: &mut Vec<Vec<Piece>>) {
    for j in 0..8 {
        for i in 0..8 {
            pieces[j][i].selected = false;
        }
    }
}

#[derive(PartialEq, Debug)]
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
    pub fn opposite(side: &Side) -> Side {
        if *side == Side::White {
            Side::Black
        } else if *side == Side::Black {
            Side::White
        } else {
            panic!("tried to opposite None");
        }
    }
}
