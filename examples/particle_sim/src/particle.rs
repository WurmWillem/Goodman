use goodman::prelude::*;

#[derive(Debug, Clone, Copy)]
pub struct Particle {
    pub kind: PartKind,
    pub has_updated: bool,
    pub vel: Vec32,
}
impl Particle {
    pub fn new(kind: PartKind) -> Self {
        let y = match kind {
            PartKind::Empty => 0.,
            PartKind::Sand => 0.,
            PartKind::Water => 0.,
        };

        Self {
            kind,
            has_updated: false,
            vel: vec2(0., y,)
        }
    }

    pub fn update(&mut self) {
        match self.kind {
            PartKind::Empty => panic!("can't update empty particle"),
            PartKind::Sand => self.vel.y += 0.2,
            PartKind::Water => self.vel.y += 0.2,
        }
    }

    pub fn get_index(&self) -> usize {
        match self.kind {
            PartKind::Empty => panic!("can't render empty particle"),
            PartKind::Sand => 0,
            PartKind::Water => 1,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PartKind {
    Empty,
    Sand,
    Water,
}