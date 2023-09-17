use crate::{engine::Engine, input::Input, prelude::Rect32, sound::Sound};
pub trait Manager {
    fn new(engine: &mut Engine) -> Self;
    fn start(&mut self) {}
    fn update(&mut self, frame_time: f64, input: &Input, sound: &Sound);
    fn render(&self, engine: &mut Engine);
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

pub struct Animation {
    frames: Vec<u32>,
    current_frame: usize,
    time_passed: f32,
    frame_duration: f32,
}
impl Animation {
    pub fn new(frames: Vec<u32>, frame_duration: f32) -> Self {
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
    pub fn get_current_frame(&self) -> u32 {
        self.frames[self.current_frame]
    }
}

#[macro_export]
macro_rules! create_textures {
    ($engine: expr, $textures: expr, $($name: expr)*) => {
        let mut i = 0;
        $(
            let tex_bytes = include_bytes!($name);
            $textures.push($engine.create_texture(tex_bytes).unwrap());
            i += 1;
        )*
       $engine.use_textures(&$textures, i);
    };
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct WindowUniform {
    pub size: [f32; 2],
}
