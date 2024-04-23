use crate::{pieces::Kind, state::Side};

#[derive(Debug, Clone, PartialEq)]
pub struct Piece {
    pub kind: Kind,
    pub selected: bool,
    pub moves: Vec<(usize, usize)>,
    pub side: Side,
}
impl Piece {
    pub fn new(kind: Kind, side: Side) -> Self {
        Self {
            kind,
            selected: false,
            moves: Vec::new(),
            side,
        }
    }
    pub fn new_empty() -> Self {
        Self {
            kind: Kind::None,
            selected: false,
            moves: Vec::new(),
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
