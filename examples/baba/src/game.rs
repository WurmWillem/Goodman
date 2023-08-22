use goodman::prelude::Sound;
use rodio::Source;

use crate::other::{Character, Direction, Move, Noun, NounPropCombi, Object, Property};
use crate::{Game, GRID_SIZE};

impl Game {
    pub fn update_character_data(&mut self) {
        for j in 0..self.grid.len() {
            for i in 0..self.grid[0].len() {
                if self.grid[j][i] != Object::Is {
                    continue;
                }

                let mut noun = None;
                if let Some(object) = self.grid[j].get(i - 1) {
                    if let Object::Noun(n) = *object {
                        noun = Some(n);
                    }
                }
                let mut property = None;
                if let Some(object) = self.grid[j].get(i + 1) {
                    if let Object::Property(p) = *object {
                        property = Some(p);
                    }
                }
                self.update_npcs(noun, property, (i, j), Direction::Hor);

                let mut noun = None;
                if let Some(row) = self.grid.get(j - 1) {
                    if let Object::Noun(n) = row[i] {
                        noun = Some(n);
                    }
                }
                let mut property = None;
                if let Some(row) = self.grid.get(j + 1) {
                    if let Object::Property(p) = row[i] {
                        property = Some(p);
                    }
                }
                self.update_npcs(noun, property, (i, j), Direction::Ver);
            }
        }

        let mut i = 0;
        let mut to_remove = vec![];
        for npc in &self.noun_prop_combi {
            let is_index = if npc.p.0 > npc.n.0 {
                npc.p.0 - 1
            } else {
                npc.p.0 + 1
            };

            let mut should_delete = false;
            if npc.dir == Direction::Hor
                && (self.grid[npc.row_or_col][npc.n.0] != Object::Noun(npc.n.1)
                    || self.grid[npc.row_or_col][npc.p.0] != Object::Property(npc.p.1)
                    || self.grid[npc.row_or_col][is_index] != Object::Is)
            {
                should_delete = true;
            } else if npc.dir == Direction::Ver
                && (self.grid[npc.n.0][npc.row_or_col] != Object::Noun(npc.n.1)
                    || self.grid[npc.p.0][npc.row_or_col] != Object::Property(npc.p.1)
                    || self.grid[is_index][npc.row_or_col] != Object::Is)
            {
                should_delete = true
            }

            if should_delete {
                println!("deleted {:?} is {:?}", npc.n.1, npc.p.1);
                self.character_data
                    .set_char_to_property(npc.n.1, npc.p.1, false);
                to_remove.push(i);
            }
            i += 1;
        }
        for r in to_remove {
            self.noun_prop_combi.remove(r);
        }

        for char in Character::iterator() {
            if self.character_data.is_win(char) && self.character_data.is_you(char) {
                self.win();
                break;
            }
        }
    }

    pub fn update_npcs(
        &mut self,
        noun: Option<Noun>,
        property: Option<Property>,
        (i, j): (usize, usize),
        dir: Direction,
    ) {
        if let Some(noun) = noun {
            if let Some(property) = property {
                let npc;
                if dir == Direction::Hor {
                    npc = NounPropCombi::new(j, (i - 1, noun), (i + 1, property), dir);
                } else {
                    npc = NounPropCombi::new(i, (j - 1, noun), (j + 1, property), dir)
                }
                if !self.noun_prop_combi.contains(&npc) {
                    self.character_data
                        .set_char_to_property(noun, property, true);

                    self.noun_prop_combi.push(npc);
                    println!("created {:?} is {:?}", npc.n.1, npc.p.1);
                }
            }
        }
    }

    pub fn move_object(&mut self, mov: Move, sound: &Sound) {
        self.grid[mov.to.j][mov.to.i] = self.grid[mov.from.j][mov.from.i];
        self.grid[mov.from.j][mov.from.i] = Object::Empty;

        if self.grid[mov.to.j][mov.to.i] == Object::Character(crate::other::Character::Baba) {
            sound
                .play_sound(self.source.clone().convert_samples())
                .unwrap();
        };
    }

    pub fn win(&mut self) {
        println!("Win!");
        self.current_level.load_next_level(&mut self.grid);
        self.reset();
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Level {
    Level1,
    Level2,
}
impl Level {
    pub fn load_next_level(&mut self, grid: &mut Vec<Vec<Object>>) {
        match self {
            Level::Level1 => {
                *self = Level::Level2;
            }
            Level::Level2 => {
                *self = Level::Level1;
            }
        }
        self.load_level(grid);
    }
    pub fn load_level(&self, grid: &mut Vec<Vec<Object>>) {
        match self {
            Level::Level1 => Self::load_level_1(grid),
            Level::Level2 => Self::load_level_2(grid),
        }
    }

    fn load_level_1(grid: &mut Vec<Vec<Object>>) {
        *grid = vec![vec![Object::Empty; GRID_SIZE.0]; GRID_SIZE.1];

        grid[2][2] = Object::Character(Character::Baba);
        grid[11][15] = Object::Character(Character::Flag);

        grid[13][7] = Object::Noun(Noun::Baba);
        grid[13][8] = Object::Is;
        grid[13][9] = Object::Property(Property::You);

        grid[8][19] = Object::Noun(Noun::Flag);
        grid[9][19] = Object::Is;
        grid[10][19] = Object::Property(Property::Win);

        grid[4][7] = Object::Noun(Noun::Wall);
        grid[4][8] = Object::Is;
        grid[4][9] = Object::Property(Property::Stop);

        for i in 0..grid[6].len() {
            grid[6][i] = Object::Character(Character::Wall);
        }
    }

    fn load_level_2(grid: &mut Vec<Vec<Object>>) {
        *grid = vec![vec![Object::Empty; GRID_SIZE.0]; GRID_SIZE.1];

        grid[6][13] = Object::Character(Character::Baba);
        grid[6][8] = Object::Character(Character::Flag);

        for j in 3..11 {
            grid[j][15] = Object::Character(Character::Wall);
        }
        for j in 3..11 {
            grid[j][3] = Object::Character(Character::Wall);
        }
        for i in 3..16 {
            grid[2][i] = Object::Character(Character::Wall);
        }
        for i in 3..16 {
            grid[10][i] = Object::Character(Character::Wall);
        }

        grid[6][15] = Object::Character(Character::Wall);

        grid[13][7] = Object::Noun(Noun::Baba);
        grid[13][8] = Object::Is;
        grid[13][9] = Object::Property(Property::You);

        grid[5][5] = Object::Noun(Noun::Flag);
        grid[12][16] = Object::Is;
        grid[8][18] = Object::Property(Property::Win);
    }
}
