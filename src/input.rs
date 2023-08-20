use crate::prelude::Vec2;
use cgmath::vec2;
use winit::event::{ElementState, KeyboardInput, MouseButton, VirtualKeyCode, WindowEvent};

struct Button {
    pressed: bool,
    held: bool,
}
impl Button {
    fn new() -> Self {
        Self {
            pressed: false,
            held: false,
        }
    }
    fn set_both(&mut self, boolean: bool) {
        self.pressed = boolean;
        self.held = boolean;
    }
}

macro_rules! CreateInputStruct {
    ($($field_name: ident)*) => {
        pub struct Input {
            cursor_pos: Vec2,
            $($field_name: Button,)*
        }

        impl Input {
            pub fn new() -> Self {
                Self {
                    cursor_pos: vec2(0., 0.),
                    $($field_name: Button::new(),)*
                }
            }
        }
    };
}
CreateInputStruct!(left_mouse right_mouse d a w s right_arrow left_arrow up_arrow down_arrow 
    zero one two three four five six seven eight nine);

macro_rules! set_button_to_is_pressed {
    ($self: ident, $is_pressed: expr, $keycode: expr, $($key_code_name: ident, $field_name: ident)*) => {
        match $keycode {
            $(VirtualKeyCode::$key_code_name => {
                $self.$field_name.set_both($is_pressed);
                true
            })*
            _ => false
        }

    };
}

impl Input {
    pub fn process_events(&mut self, event: &WindowEvent) -> bool {
        match event {
            WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        state,
                        virtual_keycode: Some(keycode),
                        ..
                    },
                ..
            } => {
                let is_pressed = *state == ElementState::Pressed;
                set_button_to_is_pressed!(self, is_pressed, keycode, W,w A,a S,s D,d Right,right_arrow Left,left_arrow Down,down_arrow 
                    Key0,zero Key1,one Key2,two Key3,three Key4,four Key5,five Key6,six Key7,seven Key8,eight Key9,nine)
            }
            WindowEvent::MouseInput { state, button, .. } => {
                let is_pressed = *state == ElementState::Pressed;
                match button {
                    MouseButton::Left => {
                        self.left_mouse.set_both(is_pressed);
                        true
                    }
                    MouseButton::Right => {
                        self.right_mouse.set_both(is_pressed);
                        true
                    }
                    _ => false,
                }
            }
            WindowEvent::CursorMoved { position, .. } => {
                self.cursor_pos = vec2(position.x, position.y);
                false
            }
            _ => false,
        }
    }
    pub fn reset_buttons(&mut self) {
        self.left_mouse.pressed = false;
        self.right_mouse.pressed = false;
        self.d.pressed = false;
        self.a.pressed = false;
        self.w.pressed = false;
        self.s.pressed = false;
        self.right_arrow.pressed = false;
        self.left_arrow.pressed = false;
        self.up_arrow.pressed = false;
        self.down_arrow.pressed = false;
    }
}

macro_rules! is_button_pressed_or_held {
    ($function_name: ident, $field_name: ident) => {
        impl Input {
            pub fn $function_name(&self, c: ButtonEnum) -> bool {
                match c {
                    ButtonEnum::LeftMouse => self.left_mouse.$field_name,
                    ButtonEnum::RightMouse => self.right_mouse.$field_name,
                    ButtonEnum::W => self.w.$field_name,
                    ButtonEnum::A => self.a.$field_name,
                    ButtonEnum::S => self.s.$field_name,
                    ButtonEnum::D => self.d.$field_name,
                    ButtonEnum::RightArrow => self.right_arrow.$field_name,
                    ButtonEnum::LeftArrow => self.left_arrow.$field_name,
                    ButtonEnum::UpArrow => self.up_arrow.$field_name,
                    ButtonEnum::DownArrow => self.down_arrow.$field_name,
                    ButtonEnum::Zero => self.zero.$field_name,
                    ButtonEnum::One => self.one.$field_name,
                    ButtonEnum::Two => self.two.$field_name,
                    ButtonEnum::Three => self.three.$field_name,
                    ButtonEnum::Four => self.four.$field_name,
                    ButtonEnum::Five => self.five.$field_name,
                    ButtonEnum::Six => self.six.$field_name,
                    ButtonEnum::Seven => self.seven.$field_name,
                    ButtonEnum::Eight => self.eight.$field_name,
                    ButtonEnum::Nine => self.nine.$field_name,
                }
            }
        }
    };
}
is_button_pressed_or_held!(is_button_pressed, pressed);
is_button_pressed_or_held!(is_button_held, held);
pub enum ButtonEnum {
    LeftMouse,
    RightMouse,
    W,
    A,
    S,
    D,
    RightArrow,
    LeftArrow,
    UpArrow,
    DownArrow,
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}
