use goodman::prelude::{rect32, Rect32};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Object {
    Empty,
    Is,
    Character(Character),
    Noun(Noun),
    Property(Property),
}
impl Object {
    pub fn get_source(&self) -> Rect32 {
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
            Object::Noun(Noun::Skull) => 13,
            Object::Character(Character::Skull) => 14,
            Object::Property(Property::Defeat) => 15,
        };
        get_source_from_index(index)
    }
}

pub fn get_source_from_index(index: u32) -> Rect32 {
    let j = (index as f32 * 0.25) as u32;
    let i = index % 4;
    rect32(i as f32 * 26., j as f32 * 26., 26., 26.)
}

macro_rules! create_character_enum {
    ($($enum: ident)*) => {
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub enum Character {
            $($enum,)*
        }
        impl Character {
            pub fn get_corresponding_noun(&self) -> Noun {
                match self {
                    $(Character::$enum => Noun::$enum,)*
                }
            }
            pub fn iterator() -> impl Iterator<Item = Character> {
                [$(Self::$enum,)*].iter().copied()
            }
        }
    };
}

create_character_enum!(Baba Flag Wall Skull);

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Noun {
    Baba,
    Flag,
    Wall,
    Skull,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Property {
    You,
    Win,
    Stop,
    Defeat,
}

macro_rules! update_field_based_on_counter {
    ($property: expr, $char_data: expr, $i: expr, $($enum: ident, $counter: ident, $bool: ident)*) => {
        match $property {
            $(Property::$enum => {
                $char_data.$counter = ($char_data.$counter as i32 + $i) as usize;
                $char_data.$bool = $char_data.$counter > 0
            })*
        };
    };
}
macro_rules! create_all_character_data {
    ($($field: ident, $enum: ident)*) => {
        pub struct AllCharacterData {
            $($field: CharacterData,)*
        }
        impl AllCharacterData {
            pub fn new() -> Self {
                Self {
                    $($field: CharacterData::new(),)*
                }
            }

            pub fn is_you(&self, char: Character) -> bool {
                match char {
                    $(Character::$enum => self.$field.is_you,)*
                }
            }
            pub fn is_win(&self, char: Character) -> bool {
                match char {
                    $(Character::$enum => self.$field.is_win,)*
                }
            }

            pub fn set_char_to_property(&mut self, noun: Noun, property: Property, enable: bool) {
                let char_data = match noun {
                    $(Noun::$enum => &mut self.$field,)*
                };

                let i = if enable { 1 } else { -1 };
                update_field_based_on_counter!(property, char_data, i,
                    You, is_you_counter, is_you
                    Win, is_win_counter, is_win
                    Stop, is_stop_counter, is_stop
                    Defeat, is_defeat_counter, is_defeat);
            }

            pub fn get_if_enabled(&self, noun: Noun, property: Property) -> bool {
                let char_data = match noun {
                    $(Noun::$enum => &self.$field,)*
                };
                match property {
                    Property::You => char_data.is_you,
                    Property::Win => char_data.is_win,
                    Property::Stop => char_data.is_stop,
                    Property::Defeat => char_data.is_defeat,
                }
            }
        }
    };
}
create_all_character_data!(baba, Baba  flag, Flag  wall, Wall  skull, Skull);

macro_rules! create_character_data {
    ($($bool_field: ident, $counter_field: ident)*) => {
        #[derive(Debug, Clone, Copy)]
        pub struct CharacterData {
            $($bool_field: bool, $counter_field: usize,)*
        }
        impl CharacterData {
            pub fn new() -> Self {
                Self {
                    $($bool_field: false,
                    $counter_field: 0,)*
                }
            }
        }
    };
}
create_character_data!(is_you, is_you_counter is_win, is_win_counter is_stop, is_stop_counter is_defeat, is_defeat_counter);

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

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Direction {
    Hor,
    Ver,
}
