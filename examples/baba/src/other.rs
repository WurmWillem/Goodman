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
            Object::Empty => 0,
            Object::Is => 1,
            Object::Property(Property::You) => 2,
            Object::Property(Property::Win) => 3,
            Object::Noun(Noun::Baba) => 4,
            Object::Character(Character::Baba) => 5,
            Object::Noun(Noun::Flag) => 6,
            Object::Character(Character::Flag) => 7,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Character {
    Baba,
    Flag,
}
impl Character {
    pub fn get_corresponding_noun(&self) -> Noun {
        match self {
            Character::Baba => Noun::Baba,
            Character::Flag => Noun::Flag,
        }
    }
    pub fn iterator() -> impl Iterator<Item = Character> {
        [Self::Baba, Self::Flag].iter().copied()
    }
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
    pub fn is_win(&self, char: Character) -> bool {
        match char {
            Character::Baba => self.baba.is_win,
            Character::Flag => self.flag.is_win,
        }
    }
    pub fn set_char_to_property(&mut self, noun: Noun, property: Property, enable: bool) {
        let char_data = match noun {
            Noun::Baba => &mut self.baba,
            Noun::Flag => &mut self.flag,
        };
        
        let i = if enable { 1 } else { -1 };
        match property {
            Property::You => {
                char_data.is_you_counter = (char_data.is_you_counter as i32 + i) as usize;
                char_data.is_you = char_data.is_you_counter > 0
            },
            Property::Win => {
                char_data.is_win_counter = (char_data.is_win_counter as i32 + i) as usize;
                char_data.is_win = char_data.is_win_counter > 0
            },
        };
    }
    pub fn get_if_enabled(&self, noun: Noun, property: Property) -> bool {
        let char_data = match noun {
            Noun::Baba => &self.baba,
            Noun::Flag => &self.flag,
        };
        match property {
            Property::You => char_data.is_you,
            Property::Win => char_data.is_win,
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

/*pub fn make_usize_tup(i: (i32, i32), u: (usize, usize)) -> (usize, usize) {
    ((i.0 + u.0 as i32) as usize, (i.1 + u.1 as i32) as usize)
}*/
