use crate::{engine::Engine, input::Input};
use rodio::{OutputStreamHandle, Source};

pub type Vec2 = cgmath::Vector2<f64>;
pub trait Manager {
    fn new(engine: &mut Engine) -> Self;
    fn start(&mut self) {}
    fn update(&mut self, frame_time: f64, input: &Input, sound: &Sound);
    fn render(&self, engine: &mut Engine);
}

#[derive(Debug, Clone, Copy)]
pub struct DrawParams {
    pub rotation: f64,
}
impl Default for DrawParams {
    fn default() -> Self {
        Self { rotation: 0. }
    }
}

pub struct Sound {
    #[allow(dead_code)] // stream is unused but it has to stay in memory
    stream: rodio::OutputStream,
    stream_handle: OutputStreamHandle,
}
impl Sound {
    pub(crate) fn new() -> Self {
        let (stream, stream_handle) =
            rodio::OutputStream::try_default().expect("can't find output device");
        Self {
            stream,
            stream_handle,
        }
    }
    pub fn play_sound<S>(&self, source: S) -> Result<(), rodio::PlayError>
    where
        S: Source<Item = f32> + Send + 'static,
    {
        self.stream_handle.play_raw(source)?;
        Ok(())
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
