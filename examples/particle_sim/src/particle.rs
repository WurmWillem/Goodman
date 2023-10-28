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
            PartKind::Sand => 1.,
            PartKind::Water => 1.,
            PartKind::Wood => 0.,
        };

        Self {
            kind,
            has_updated: false,
            vel: vec2(0., y),
        }
    }

    pub fn update(&mut self) {
        match self.kind {
            PartKind::Empty => panic!("can't update empty particle"),
            PartKind::Sand => self.vel.y += 0.05,
            PartKind::Water => self.vel.y += 0.03,
            PartKind::Wood => panic!("can't update wood particle"),
        }
    }

    pub fn get_index(&self) -> usize {
        match self.kind {
            PartKind::Empty => panic!("can't render empty particle"),
            PartKind::Sand => 0,
            PartKind::Water => 1,
            PartKind::Wood => 2,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PartKind {
    Empty,
    Sand,
    Water,
    Wood,
}
