use self::Layer::*;
use crate::prelude::{Engine, Texture};

use cgmath::vec2;
use std::slice::Iter;
use winit::event::{ElementState, KeyboardInput, MouseButton, VirtualKeyCode, WindowEvent};

pub type Vec2 = cgmath::Vector2<f64>;
pub type Vec3 = cgmath::Vector3<f64>;

pub type InstIndex = u32;
pub type TexIndex = u32;

pub trait Manager {
    fn new(textures: Vec<Texture>) -> Self;
    fn update(&mut self, frame_time: f64, input: &Input);
    fn render(&self, state: &mut Engine);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Layer {
    Layer1,
    Layer2,
    Layer3,
    Layer4,
    Layer5,
}
impl Layer {
    pub fn iterator() -> Iter<'static, Layer> {
        static LAYERS: [Layer; 5] = [Layer1, Layer2, Layer3, Layer4, Layer5];
        LAYERS.iter()
    }
}

pub struct Input {
    cursor_pos: Vec2,
    left_mouse_button_pressed: bool,
    d_pressed: bool,
    a_pressed: bool,
    w_pressed: bool,
    s_pressed: bool,
    right_arrow_pressed: bool,
    left_arrow_pressed: bool,
    up_arrow_pressed: bool,
    down_arrow_pressed: bool,
}
impl Input {
    pub fn new() -> Self {
        Self {
            cursor_pos: vec2(0., 0.),
            left_mouse_button_pressed: false,
            d_pressed: false,
            a_pressed: false,
            w_pressed: false,
            s_pressed: false,
            right_arrow_pressed: false,
            left_arrow_pressed: false,
            up_arrow_pressed: false,
            down_arrow_pressed: false,
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
        if self.left_mouse_button_pressed {
            self.left_mouse_button_pressed = false;
        }
    }

    pub fn get_cursor_pos(&self) -> Vec2 {
        self.cursor_pos
    }
    pub fn is_left_mouse_button_pressed(&self) -> bool {
        self.left_mouse_button_pressed
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
}

pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
    pub a: f64,
}

#[allow(missing_docs)]
impl Color {
    pub fn new(r: f64, g: f64, b: f64, a: f64) -> Self {
        Color { r, g, b, a }
    }
    pub const TRANSPARENT: Self = Self {
        r: 0.0,
        g: 0.0,
        b: 0.0,
        a: 0.0,
    };
    pub const BLACK: Self = Self {
        r: 0.0,
        g: 0.0,
        b: 0.0,
        a: 1.0,
    };
    pub const WHITE: Self = Self {
        r: 1.0,
        g: 1.0,
        b: 1.0,
        a: 1.0,
    };
    pub const RED: Self = Self {
        r: 1.0,
        g: 0.0,
        b: 0.0,
        a: 1.0,
    };
    pub const GREEN: Self = Self {
        r: 0.0,
        g: 1.0,
        b: 0.0,
        a: 1.0,
    };
    pub const BLUE: Self = Self {
        r: 0.0,
        g: 0.0,
        b: 1.0,
        a: 1.0,
    };
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Windowniform {
    pub size: [f32; 2],
}
