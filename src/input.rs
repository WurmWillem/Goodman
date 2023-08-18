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

macro_rules! CreateInput {
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
CreateInput!(left_mouse right_mouse d a w s right_arrow left_arrow up_arrow down_arrow zero one two three four five six seven eight nine);


// CreateCharEnum!(hello, d.held);

/*pub struct Input {
    cursor_pos: Vec2,

    left_mouse: Button,
    right_mouse: Button,
    d: Button,
    a: Button,
    w: Button,
    s: Button,
    right_arrow: Button,
    left_arrow: Button,
    up_arrow: Button,
    down_arrow: Button,
    zero: Button,
    one: Button,
    two: Button,
    three: Button,
    four: Button,
    five: Button,
    six: Button,
    seven: Button,
    eight: Button,
    nine: Button,
}*/
impl Input {
    /*pub fn new() -> Self {
        Self {
            cursor_pos: vec2(0., 0.),
            left_mouse: Button::new(),
            right_mouse: Button::new(),
            d: Button::new(),
            a: Button::new(),
            w: Button::new(),
            s: Button::new(),
            right_arrow: Button::new(),
            left_arrow: Button::new(),
            up_arrow: Button::new(),
            down_arrow: Button::new(),
            zero: Button::new(),
            one: Button::new(),
            two: Button::new(),
            three: Button::new(),
            four: Button::new(),
            five: Button::new(),
            six: Button::new(),
            seven: Button::new(),
            eight: Button::new(),
            nine: Button::new(),
        }
    }*/
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
                let mut return_bool = true;
                match keycode {
                    VirtualKeyCode::W => {
                        self.w.set_both(is_pressed);
                    }
                    VirtualKeyCode::A => {
                        self.a.set_both(is_pressed);
                    }
                    VirtualKeyCode::S => {
                        self.s.set_both(is_pressed);
                    }
                    VirtualKeyCode::D => {
                        self.d.set_both(is_pressed);
                    }
                    VirtualKeyCode::Right => {
                        self.right_arrow.set_both(is_pressed);
                    }
                    VirtualKeyCode::Left => {
                        self.left_arrow.set_both(is_pressed);
                    }
                    VirtualKeyCode::Up => {
                        self.up_arrow.set_both(is_pressed);
                    }
                    VirtualKeyCode::Down => {
                        self.down_arrow.set_both(is_pressed);
                    }
                    VirtualKeyCode::Key0 => {
                        self.zero.set_both(is_pressed);
                    }
                    VirtualKeyCode::Key1 => {
                        self.one.set_both(is_pressed);
                    }
                    VirtualKeyCode::Key2 => {
                        self.two.set_both(is_pressed);
                    }
                    VirtualKeyCode::Key3 => {
                        self.three.set_both(is_pressed);
                    }
                    VirtualKeyCode::Key4 => {
                        self.four.set_both(is_pressed);
                    }
                    VirtualKeyCode::Key5 => {
                        self.five.set_both(is_pressed);
                    }
                    VirtualKeyCode::Key6 => {
                        self.six.set_both(is_pressed);
                    }
                    VirtualKeyCode::Key7 => {
                        self.seven.set_both(is_pressed);
                    }
                    VirtualKeyCode::Key8 => {
                        self.eight.set_both(is_pressed);
                    }
                    VirtualKeyCode::Key9 => {
                        self.nine.set_both(is_pressed);
                    }
                    _ => return_bool = false,
                };
                return_bool
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

    pub fn get_cursor_pos(&self) -> Vec2 {
        self.cursor_pos
    }
    pub fn is_left_mouse_button_pressed(&self) -> bool {
        self.left_mouse.pressed
    }
    pub fn is_right_mouse_button_pressed(&self) -> bool {
        self.right_mouse.pressed
    }

    pub fn is_d_pressed(&self) -> bool {
        self.d.pressed
    }
    pub fn is_a_pressed(&self) -> bool {
        self.a.pressed
    }
    pub fn is_w_pressed(&self) -> bool {
        self.w.pressed
    }
    pub fn is_s_pressed(&self) -> bool {
        self.s.pressed
    }

    /*pub fn is_right_arrow_pressed(&self) -> bool {
        self.right_arrow_pressed
    }
    pub fn is_left_arrow_pressed(&self) -> bool {
        self.left_arrow_pressed
    }
    pub fn is_up_arrow_pressed(&self) -> bool {
        self.up_arrow_pressed
    }
    pub fn is_down_arrow_pressed(&self) -> bool {
        self.down_arrow_pressed
    }

    pub fn is_zero_pressed(&self) -> bool {
        self.zero_pressed
    }
    pub fn is_one_pressed(&self) -> bool {
        self.one_pressed
    }
    pub fn is_two_pressed(&self) -> bool {
        self.two_pressed
    }
    pub fn is_three_pressed(&self) -> bool {
        self.three_pressed
    }
    pub fn is_four_pressed(&self) -> bool {
        self.four_pressed
    }
    pub fn is_five_pressed(&self) -> bool {
        self.five_pressed
    }
    pub fn is_six_pressed(&self) -> bool {
        self.six_pressed
    }
    pub fn is_seven_pressed(&self) -> bool {
        self.seven_pressed
    }
    pub fn is_eight_pressed(&self) -> bool {
        self.eight_pressed
    }
    pub fn is_nine_pressed(&self) -> bool {
        self.nine_pressed
    }*/
}
enum Char {
    W,
    A,
    S,
    D,
    RightArrow,
    LeftArrow,
    UpArrow,
    DonwArrow,
}
