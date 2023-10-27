use goodman::prelude::*;

#[derive(Debug, Clone, Copy)]
pub struct Particle {
    pub kind: PartKind,
    pub has_updated: bool,
    pub vel: Vec32,
}
impl Particle {
    pub fn new(kind: PartKind) -> Self {
        Self {
            kind,
            has_updated: false,
            vel: vec2(1., 0.,)
        }
    }

    pub fn update(&mut self) {
        match self.kind {
            PartKind::Empty => panic!("can't update empty particle"),
            PartKind::Sand => self.vel.y += 0.05,
            PartKind::Water => self.vel.y += 0.05,
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