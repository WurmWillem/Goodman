use crate::{engine::Engine, input::Input, prelude::Rect32, sound::Sound};
pub trait Manager {
    fn new(engine: &mut Engine) -> Self;
    fn start(&mut self) {}
    fn update(&mut self, delta_t: f64, input: &Input, sound: &Sound);
    fn render(&mut self, engine: &mut Engine);
}

#[derive(Debug, Clone, Copy)]
pub struct DrawParams {
    pub rotation: f32,
    pub source: Option<Rect32>,
}
impl DrawParams {
    pub fn from_source(source: Rect32) -> Self {
        DrawParams {
            source: Some(source),
            ..Default::default()
        }
    }
}
impl Default for DrawParams {
    fn default() -> Self {
        Self {
            rotation: 0.,
            source: None,
        }
    }
}

pub struct Animation<T: Copy> {
    frames: Vec<T>,
    current_frame: usize,
    time_passed: f32,
    frame_duration: f32,
}
impl<T: Copy> Animation<T> {
    pub fn new(frames: Vec<T>, frame_duration: f32) -> Self {
        Animation {
            frames,
            current_frame: 0,
            time_passed: 0.,
            frame_duration,
        }
    }
    pub fn update(&mut self, delta_t: f32) {
        self.time_passed += delta_t;
        if self.time_passed > self.frame_duration {
            if self.frames.len() > self.current_frame + 1 {
                self.current_frame += 1;
            } else {
                self.current_frame = 0;
            }

            self.time_passed -= self.frame_duration;
        }
    }
    pub fn get_current_frame(&self) -> T {
        self.frames[self.current_frame]
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct WindowUniform {
    pub size: [f32; 2],
}

pub struct Color {
    /// Red component of the color
    pub r: f64,
    /// Green component of the color
    pub g: f64,
    /// Blue component of the color
    pub b: f64,
    /// Alpha component of the color
    pub a: f64,
}

#[allow(missing_docs)]
impl Color {
    pub const fn new(r: f64, g: f64, b: f64, a: f64) -> Self {
        Self { r, g, b, a }
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
        a: 255.0,
    };
    pub const WHITE: Self = Self {
        r: 255.0,
        g: 255.0,
        b: 255.0,
        a: 255.0,
    };
    pub const RED: Self = Self {
        r: 255.0,
        g: 0.0,
        b: 0.0,
        a: 255.0,
    };
    pub const GREEN: Self = Self {
        r: 0.0,
        g: 255.0,
        b: 0.0,
        a: 255.0,
    };
    pub const BLUE: Self = Self {
        r: 0.0,
        g: 0.0,
        b: 255.0,
        a: 255.0,
    };
}
