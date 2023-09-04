use goodman::prelude::{Rect32, rect32};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Object {
    Empty,
    Is,
    Character(Character),
    Noun(Noun),
    Property(Property),
}
impl Object {
    pub fn get_tex_index(&self) -> Rect32 {
        let index = match self {
            Object::Empty => 11,
            Object::Is => 6,
            Object::Noun(Noun::Baba) => 0,
            Object::Character(Character::Baba) => 1,
            Object::Property(Property::You) => 7,
            Object::Noun(Noun::Flag) => 2,
            Object::Character(Character::Flag) => 4,
            Object::Property(Property::Win) => 3,
            Object::Noun(Noun::Wall) => 9,
            Object::Character(Character::Wall) => 10,
            Object::Property(Property::Stop) => 8,
        };
        let j: u32 = (index as f32 / 4.) as u32;
        let i = index % 4;
        rect32(i as f32 * 26., j as f32 * 26., 26., 26.)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Character {
    Baba,
    Flag,
    Wall,
}
impl Character {
    pub fn get_corresponding_noun(&self) -> Noun {
        match self {
            Character::Baba => Noun::Baba,
            Character::Flag => Noun::Flag,
            Character::Wall => Noun::Wall,
        }
    }
    pub fn iterator() -> impl Iterator<Item = Character> {
        [Self::Baba, Self::Flag, Self::Wall].iter().copied()
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Noun {
    Baba,
    Flag,
    Wall,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Property {
    You,
    Win,
    Stop,
}

pub struct AllCharacterData {
    baba: CharacterData,
    flag: CharacterData,
    wall: CharacterData,
}
impl AllCharacterData {
    pub fn new() -> Self {
        Self {
            baba: CharacterData::new(),
            flag: CharacterData::new(),
            wall: CharacterData::new(),
        }
    }
    pub fn is_you(&self, char: Character) -> bool {
        match char {
            Character::Baba => self.baba.is_you,
            Character::Flag => self.flag.is_you,
            Character::Wall => self.wall.is_you,
        }
    }
    pub fn is_win(&self, char: Character) -> bool {
        match char {
            Character::Baba => self.baba.is_win,
            Character::Flag => self.flag.is_win,
            Character::Wall => self.wall.is_you,
        }
    }
    pub fn set_char_to_property(&mut self, noun: Noun, property: Property, enable: bool) {
        let char_data = match noun {
            Noun::Baba => &mut self.baba,
            Noun::Flag => &mut self.flag,
            Noun::Wall => &mut self.wall,
        };

        let i = if enable { 1 } else { -1 };
        match property {
            Property::You => {
                char_data.is_you_counter = (char_data.is_you_counter as i32 + i) as usize;
                char_data.is_you = char_data.is_you_counter > 0
            }
            Property::Win => {
                char_data.is_win_counter = (char_data.is_win_counter as i32 + i) as usize;
                char_data.is_win = char_data.is_win_counter > 0
            }
            Property::Stop => {
                char_data.is_stop_counter = (char_data.is_stop_counter as i32 + i) as usize;
                char_data.is_stop = char_data.is_stop_counter > 0
            }
        };
    }
    pub fn get_if_enabled(&self, noun: Noun, property: Property) -> bool {
        let char_data = match noun {
            Noun::Baba => &self.baba,
            Noun::Flag => &self.flag,
            Noun::Wall => &self.wall,
        };
        match property {
            Property::You => char_data.is_you,
            Property::Win => char_data.is_win,
            Property::Stop => char_data.is_stop,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Direction {
    Hor,
    Ver,
}

#[derive(Debug, Clone, Copy)]
pub struct CharacterData {
    is_you: bool,
    is_you_counter: usize,
    is_win: bool,
    is_win_counter: usize,
    is_stop: bool,
    is_stop_counter: usize,
}
impl CharacterData {
    pub fn new() -> Self {
        Self {
            is_you: false,
            is_you_counter: 0,
            is_win: false,
            is_win_counter: 0,
            is_stop: false,
            is_stop_counter: 0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct NounPropCombi {
    pub row_or_col: usize,
    pub n: (usize, Noun),
    pub p: (usize, Property),
    pub dir: Direction,
}
impl NounPropCombi {
    pub fn new(row_or_col: usize, n: (usize, Noun), p: (usize, Property), dir: Direction) -> Self {
        Self {
            row_or_col,
            n,
            p,
            dir,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Move {
    pub from: VecPos,
    pub to: VecPos,
}
impl Move {
    pub fn new(from: VecPos, to: VecPos) -> Self {
        Self { from, to }
    }
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct VecPos {
    pub i: usize,
    pub j: usize,
}
impl VecPos {
    pub fn new(vec_pos: (usize, usize)) -> Self {
        Self {
            i: vec_pos.0,
            j: vec_pos.1,
        }
    }
    pub fn add_i32_tuple(vec_pos: VecPos, t32_tuple: (i32, i32)) -> VecPos {
        let mut copy = vec_pos;
        copy.i = (copy.i as i32 + t32_tuple.0) as usize;
        copy.j = (copy.j as i32 + t32_tuple.1) as usize;
        copy
    }
}
