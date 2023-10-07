use crate::other::{
    Character, Noun, Object, Property,
};
use crate::GRID_SIZE;

#[derive(Debug, Clone, Copy)]
pub enum Level {
    Level1,
    Level2,
    Level3,
}
impl Level {
    pub fn load_next_level(&mut self, grid: &mut Vec<Vec<Object>>) {
        match self {
            Level::Level1 => {
                *self = Level::Level2;
            }
            Level::Level2 => {
                *self = Level::Level3;
            }
            Level::Level3 => {
                *self = Level::Level1;
            }
        }
        self.load_level(grid);
    }
    pub fn load_level(&self, grid: &mut Vec<Vec<Object>>) {
        println!("loading next level");
        match self {
            Level::Level1 => load_level_1(grid),
            Level::Level2 => load_level_2(grid),
            Level::Level3 => load_level_3(grid),
        }
    }

    
}

fn load_level_3(grid: &mut Vec<Vec<Object>>) {
    *grid = vec![vec![Object::Empty; GRID_SIZE.0]; GRID_SIZE.1];

    grid[9][15] = Object::Character(Character::Wall);

    grid[8][13] = Object::Noun(Noun::Flag);
    grid[9][13] = Object::Is;
    grid[10][13] = Object::Property(Property::Stop);

    grid[8][8] = Object::Noun(Noun::Wall);
    grid[9][8] = Object::Is;
    grid[10][8] = Object::Property(Property::You);

    grid[3][9] = Object::Noun(Noun::Baba);
    grid[2][14] = Object::Property(Property::Win);

    for j in 5..13 {
        grid[j][17] = Object::Character(Character::Flag);
    }
    for j in 5..13 {
        grid[j][11] = Object::Character(Character::Flag);
    }
    for i in 12..17 {
        grid[12][i] = Object::Character(Character::Flag);
    }
    for i in 5..17 {
        grid[5][i] = Object::Character(Character::Flag);
    }
    for j in 0..5 {
        grid[j][5] = Object::Character(Character::Flag);
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