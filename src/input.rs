use cgmath::vec2;
use winit::event::{ElementState, KeyboardInput, MouseButton, VirtualKeyCode, WindowEvent};
use crate::prelude::Vec2;

pub struct Input {
    cursor_pos: Vec2,
    left_mouse_button_pressed: bool,
    right_mouse_button_pressed: bool,
    d_pressed: bool,
    a_pressed: bool,
    w_pressed: bool,
    s_pressed: bool,
    right_arrow_pressed: bool,
    left_arrow_pressed: bool,
    up_arrow_pressed: bool,
    down_arrow_pressed: bool,
    zero_pressed: bool,
    one_pressed: bool,
    two_pressed: bool,
    three_pressed: bool,
    four_pressed: bool,
    five_pressed: bool,
    six_pressed: bool,
    seven_pressed: bool,
    eight_pressed: bool,
    nine_pressed: bool,
}
impl Input {
    pub fn new() -> Self {
        Self {
            cursor_pos: vec2(0., 0.),
            left_mouse_button_pressed: false,
            right_mouse_button_pressed: false,
            d_pressed: false,
            a_pressed: false,
            w_pressed: false,
            s_pressed: false,
            right_arrow_pressed: false,
            left_arrow_pressed: false,
            up_arrow_pressed: false,
            down_arrow_pressed: false,
            zero_pressed: false,
            one_pressed: false,
            two_pressed: false,
            three_pressed: false,
            four_pressed: false,
            five_pressed: false,
            six_pressed: false,
            seven_pressed: false,
            eight_pressed: false,
            nine_pressed: false,
        }
    }
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
                match keycode {
                    VirtualKeyCode::W => {
                        self.w_pressed = is_pressed;
                        true
                    }
                    VirtualKeyCode::A => {
                        self.a_pressed = is_pressed;
                        true
                    }
                    VirtualKeyCode::S => {
                        self.s_pressed = is_pressed;
                        true
                    }
                    VirtualKeyCode::D => {
                        self.d_pressed = is_pressed;
                        true
                    }
                    VirtualKeyCode::Right => {
                        self.right_arrow_pressed = is_pressed;
                        true
                    }
                    VirtualKeyCode::Left => {
                        self.left_arrow_pressed = is_pressed;
                        true
                    }
                    VirtualKeyCode::Up => {
                        self.up_arrow_pressed = is_pressed;
                        true
                    }
                    VirtualKeyCode::Down => {
                        self.down_arrow_pressed = is_pressed;
                        true
                    }
                    VirtualKeyCode::Key0 => {
                        self.zero_pressed = is_pressed;
                        true
                    }
                    VirtualKeyCode::Key1 => {
                        self.one_pressed = is_pressed;
                        true
                    }
                    VirtualKeyCode::Key2 => {
                        self.two_pressed = is_pressed;
                        true
                    }
                    VirtualKeyCode::Key3 => {
                        self.three_pressed = is_pressed;
                        true
                    }
                    VirtualKeyCode::Key4 => {
                        self.four_pressed = is_pressed;
                        true
                    }
                    VirtualKeyCode::Key5 => {
                        self.five_pressed = is_pressed;
                        true
                    }
                    VirtualKeyCode::Key6 => {
                        self.six_pressed = is_pressed;
                        true
                    }
                    VirtualKeyCode::Key7 => {
                        self.seven_pressed = is_pressed;
                        true
                    }
                    VirtualKeyCode::Key8 => {
                        self.eight_pressed = is_pressed;
                        true
                    }
                    VirtualKeyCode::Key9 => {
                        self.nine_pressed = is_pressed;
                        true
                    }
                    _ => false,
                }
            }
            WindowEvent::MouseInput { state, button, .. } => {
                let is_pressed = *state == ElementState::Pressed;
                match button {
                    MouseButton::Left => {
                        self.left_mouse_button_pressed = is_pressed;
                        true
                    }
                    MouseButton::Right => {
                        self.right_mouse_button_pressed = is_pressed;
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
        /*
        make held a thing
         */
        self.left_mouse_button_pressed = false;
        self.right_mouse_button_pressed = false;
        self.d_pressed = false;
        self.a_pressed = false;
        self.w_pressed = false;
        self.s_pressed = false;
        self.right_arrow_pressed = false;
        self.left_arrow_pressed = false;
        self.up_arrow_pressed = false;
        self.down_arrow_pressed = false;
    }

    pub fn get_cursor_pos(&self) -> Vec2 {
        self.cursor_pos
    }
    pub fn is_left_mouse_button_pressed(&self) -> bool {
        self.left_mouse_button_pressed
    }
    pub fn is_right_mouse_button_pressed(&self) -> bool {
        self.right_mouse_button_pressed
    }

    pub fn is_d_pressed(&self) -> bool {
        self.d_pressed
    }
    pub fn is_a_pressed(&self) -> bool {
        self.a_pressed
    }
    pub fn is_w_pressed(&self) -> bool {
        self.w_pressed
    }
    pub fn is_s_pressed(&self) -> bool {
        self.s_pressed
    }

    pub fn is_right_arrow_pressed(&self) -> bool {
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
    }
}