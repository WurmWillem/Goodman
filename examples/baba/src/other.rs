#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Object {
    Empty,
    Is,
    Character(Character),
    Noun(Noun),
    Property(Property),
}
impl Object {
    pub fn get_tex_index(&self) -> usize {
        match self {
            Object::Empty => panic!("this should not be possible"),
            Object::Is => 0,
            Object::Property(Property::You) => 1,
            Object::Property(Property::Win) => 2,
            Object::Noun(Noun::Baba) => 3,
            Object::Noun(Noun::Flag) => 4,
            Object::Character(Character::Baba) => 5,
            Object::Character(Character::Flag) => 6,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Character {
    Baba,
    Flag,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Noun {
    Baba,
    Flag,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Property {
    You,
    Win,
}

pub struct AllCharacterData {
    baba: CharacterData,
    flag: CharacterData,
}
impl AllCharacterData {
    pub fn new() -> Self {
        Self {
            baba: CharacterData::new(),
            flag: CharacterData::new(),
        }
    }
    pub fn is_you(&self, char: Character) -> bool {
        match char {
            Character::Baba => self.baba.is_you,
            Character::Flag => self.flag.is_you,
        }
    }
    pub fn set_char_to_property(&mut self, noun: Noun, property: Property, enable: bool) {
        let i = if enable { 1 } else { -1 };
        match noun {
            Noun::Baba => match property {
                Property::You => {
                    self.baba.is_you_counter = (self.baba.is_you_counter as i32 + i) as usize;
                    self.baba.is_you = self.baba.is_you_counter > 0
                }
                Property::Win => {}
            },
            Noun::Flag => match property {
                Property::You => {
                    self.flag.is_you_counter = (self.flag.is_you_counter as i32 + i) as usize;
                    self.flag.is_you = self.flag.is_you_counter > 0
                }
                Property::Win => {
                    self.flag.is_win_counter = (self.flag.is_win_counter as i32 + i) as usize;
                    self.flag.is_win = self.flag.is_win_counter > 0
                }
            },
        }
    }
    /*fn get_if_enabled(&self, noun: Noun, property: Property) -> bool {
        match noun {
            Noun::Kirb => match property {
                Property::You => self.kirb.is_you,
            },
        }
    }*/
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Direction {
    Hor,
    Ver,
}

pub struct CharacterData {
    is_you: bool,
    is_you_counter: usize,
    is_win: bool,
    is_win_counter: usize,
}
impl CharacterData {
    pub fn new() -> Self {
        Self {
            is_you: false,
            is_you_counter: 0,
            is_win: false,
            is_win_counter: 0,
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

pub fn make_usize_tup(i: (i32, i32), u: (usize, usize)) -> (usize, usize) {
    ((i.0 + u.0 as i32) as usize, (i.1 + u.1 as i32) as usize)
}
