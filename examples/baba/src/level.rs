use crate::other::{Character, Noun, Object, Property};
use crate::GRID_SIZE;

#[derive(Debug, Clone, Copy)]
pub enum Level {
    Level1,
    Level2,
    Level3,
    Level4,
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
                *self = Level::Level4;
            }
            Level::Level4 => {
                *self = Level::Level1;
            }
        }
        self.load_level(grid);
    }
    pub fn load_level(&self, grid: &mut Vec<Vec<Object>>) {
        println!("loading level");
        match self {
            Level::Level1 => load_level_1(grid),
            Level::Level2 => load_level_2(grid),
            Level::Level3 => load_level_3(grid),
            Level::Level4 => load_level_4(grid),
        }
    }
}

fn create_hor_pattern(j: usize, i: usize, noun: Noun, prop: Property, grid: &mut Vec<Vec<Object>>) {
    grid[j][i] = Object::Noun(noun);
    grid[j][i+1] = Object::Is;
    grid[j][i+2] = Object::Property(prop);
}
fn create_ver_pattern(j: usize, i: usize, noun: Noun, prop: Property, grid: &mut Vec<Vec<Object>>) {
    grid[j][i] = Object::Noun(noun);
    grid[j+1][i] = Object::Is;
    grid[j+2][i] = Object::Property(prop);
}

fn load_level_4(grid: &mut Vec<Vec<Object>>) {
    *grid = vec![vec![Object::Empty; GRID_SIZE.0]; GRID_SIZE.1];

    create_hor_pattern(0, 0, Noun::Skull, Property::Defeat, grid);
    create_hor_pattern(1, 0, Noun::Flag, Property::Win, grid);

    create_ver_pattern(10, 5, Noun::Baba, Property::You, grid);
    create_hor_pattern(9, 5, Noun::Wall, Property::Stop, grid);

    for i in 8..GRID_SIZE.0 {
        grid[2][i] = Object::Character(Character::Wall);
    }
    grid[3][8] = Object::Character(Character::Wall);
    for j in 3..8 {
        grid[j][7] = Object::Character(Character::Wall);
    }
    


    for j in 0..8 {
        grid[j][12] = Object::Character(Character::Skull);
    }
    for i in 12..GRID_SIZE.0 {
        grid[8][i] = Object::Character(Character::Skull);
    }

    


    grid[2][2] = Object::Character(Character::Baba);

    grid[5][16] = Object::Character(Character::Flag);
}

fn load_level_3(grid: &mut Vec<Vec<Object>>) {
    *grid = vec![vec![Object::Empty; GRID_SIZE.0]; GRID_SIZE.1];

    grid[9][15] = Object::Character(Character::Wall);

    create_ver_pattern(8, 13, Noun::Flag, Property::Stop, grid);
    create_ver_pattern(8, 8, Noun::Wall, Property::You, grid);

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

    create_hor_pattern(13, 7, Noun::Baba, Property::You, grid);
    create_ver_pattern(8, 19, Noun::Flag, Property::Win, grid);
    create_hor_pattern(4, 7, Noun::Wall, Property::Stop, grid);

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

    create_hor_pattern(13, 7, Noun::Baba, Property::You, grid);

    grid[5][5] = Object::Noun(Noun::Flag);
    grid[12][16] = Object::Is;
    grid[8][18] = Object::Property(Property::Win);
}
