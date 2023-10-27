use crate::prelude::Vec64;
use cgmath::vec2;
use winit::event::{
    ElementState, KeyboardInput, MouseButton, MouseScrollDelta, VirtualKeyCode, WindowEvent,
};

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
            cursor_pos: Vec64,
            mouse_wheel: i8, // wheel up = 1, down = -1, no movement = 0,
            $($field_name: Button,)*
        }

        impl Input {
            pub(crate) fn new() -> Self {
                Self {
                    cursor_pos: vec2(0., 0.),
                    mouse_wheel: 0,
                    $($field_name: Button::new(),)*
                }
            }

            pub fn get_cursor_pos(&self) -> Vec64 {
                self.cursor_pos
            }

            pub fn get_wheel_movement(&self) -> i8 {
                self.mouse_wheel // wheel up = 1, down = -1, no movement = 0,
            }
        }
    };
}
CreateInputStruct!(left_mouse right_mouse right_arrow left_arrow up_arrow down_arrow 
    zero one two three four five six seven eight nine 
    a b c d e f g h i j k l m n o p q r s t u v w x y z
    escape f1 f2 f3 f4 f5 f6 f7 f8 f9  f10  f11  f12
    insert home delete end page_down page_up back enter space caps tab period 
    plus minus equals slash backslash apostrophe asterisk comma 
    r_control r_shift r_alt l_control l_shift l_alt);

impl Input {
    pub(crate) fn process_events(&mut self, event: &WindowEvent) -> bool {
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
                set_button_to_is_pressed!(self, is_pressed, keycode, Right,right_arrow Left,left_arrow Down,down_arrow Up,up_arrow
                    Key0,zero Key1,one Key2,two Key3,three Key4,four Key5,five Key6,six Key7,seven Key8,eight Key9,nine
                    A,a B,b C,c D,d E,e F,f G,g H,h I,i J,j K,k L,l M,m N,n O,o P,p Q,q R,r S,s T,t U,u V,v W,w X,x Y,y Z,z
                    Escape,escape F1,f1 F2,f2 F3,f3 F4,f4 F5,f5 F6,f6 F7,f7 F8,f8 F9,f9 F10,f10 F11,f11 F12,f12
                    Insert,insert Home,home Delete,delete End,end PageDown,page_down PageUp,page_up Back,back Return,enter Space,space Capital,caps Tab,tab Period,period
                    Plus,plus Minus,minus Equals,equals Slash,slash Backslash,backslash Apostrophe,apostrophe Asterisk,asterisk Comma,comma
                    RControl,r_control RShift,r_shift RAlt,r_alt LControl,l_control LShift,l_shift LAlt,l_alt
                )
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
            WindowEvent::MouseWheel { delta, .. } => {
                if let MouseScrollDelta::LineDelta(_, y) = delta {
                    self.mouse_wheel = *y as i8;
                    return true;
                }
                false
            }
            WindowEvent::CursorMoved { position, .. } => {
                self.cursor_pos = vec2(position.x, position.y);
                false
            }
            _ => false,
        }
    }
    pub(crate) fn reset_buttons(&mut self) {
        macro_rules! reset_buttons {
            ($($field_name: ident)*) => {
                $(self.$field_name.pressed = false;)*
            };
        }
        reset_buttons!(left_mouse right_mouse d a w s right_arrow left_arrow up_arrow down_arrow
            zero one two three four five six seven eight nine a b c d e f g h i j k l m n o p q r s t u v w x y z
            escape f1 f2 f3 f4 f5 f6 f7 f8 f9  f10  f11  f12
            insert home delete end page_down page_up back enter space caps tab period
            plus minus equals slash backslash apostrophe asterisk comma
            r_control r_shift r_alt l_control l_shift l_alt);

        self.mouse_wheel = 0;
    }
}

macro_rules! is_button_pressed_or_held {
    ($function_name: ident, $field_name: ident, $($button_enum: ident, $button: ident)*) => {
        impl Input {
            pub fn $function_name(&self, c: ButtonEnum) -> bool {
                match c {
                    $(ButtonEnum::$button_enum => self.$button.$field_name,)*
                }
            }
        }
    };
}
is_button_pressed_or_held!(is_button_pressed, pressed,
    LeftMouse,left_mouse RightMouse,right_mouse RightArrow,right_arrow LeftArrow,left_arrow DownArrow,down_arrow UpArrow,up_arrow
    Zero,zero One,one Two,two Three,three Four,four Five,five Six,six Seven,seven Eight,eight Nine,nine
    A,a B,b C,c D,d E,e F,f G,g H,h I,i J,j K,k L,l M,m N,n O,o P,p Q,q R,r S,s T,t U,u V,v W,w X,x Y,y Z,z
    Escape,escape F1,f1 F2,f2 F3,f3 F4,f4 F5,f5 F6,f6 F7,f7 F8,f8 F9,f9 F10,f10 F11,f11 F12,f12
    Insert,insert Home,home Delete,delete End,end PageDown,page_down PageUp,page_up Back,back Enter,enter Space,space Caps,caps Tab,tab Period,period
    Plus,plus Minus,minus Equals,equals Slash,slash Backslash,backslash Apostrophe,apostrophe Asterisk,asterisk Comma,comma
    RControl,r_control RShift,r_shift RAlt,r_alt LControl,l_control LShift,l_shift LAlt,l_alt
);
is_button_pressed_or_held!(is_button_held, held,
    LeftMouse,left_mouse RightMouse,right_mouse RightArrow,right_arrow LeftArrow,left_arrow DownArrow,down_arrow UpArrow,up_arrow
    Zero,zero One,one Two,two Three,three Four,four Five,five Six,six Seven,seven Eight,eight Nine,nine
    A,a B,b C,c D,d E,e F,f G,g H,h I,i J,j K,k L,l M,m N,n O,o P,p Q,q R,r S,s T,t U,u V,v W,w X,x Y,y Z,z
    Escape,escape F1,f1 F2,f2 F3,f3 F4,f4 F5,f5 F6,f6 F7,f7 F8,f8 F9,f9 F10,f10 F11,f11 F12,f12
    Insert,insert Home,home Delete,delete End,end PageDown,page_down PageUp,page_up Back,back Enter,enter Space,space Caps,caps Tab,tab Period,period
    Plus,plus Minus,minus Equals,equals Slash,slash Backslash,backslash Apostrophe,apostrophe Asterisk,asterisk Comma,comma
    RControl,r_control RShift,r_shift RAlt,r_alt LControl,l_control LShift,l_shift LAlt,l_alt
);

pub enum ButtonEnum {
    LeftMouse,
    RightMouse,
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

    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,

    Escape,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,

    Insert,
    Home,
    Delete,
    End,
    PageDown,
    PageUp,
    Back,
    Enter,
    Space,
    Caps,
    Tab,

    Period,
    Plus,
    Minus,
    Equals,
    Slash,
    Backslash,
    Apostrophe,
    Asterisk,
    Comma,

    RControl,
    RShift,
    RAlt,
    LControl,
    LShift,
    LAlt,
}
