pub use crate::instances::{rect, Rect};
pub use crate::object_data::VERTEX_SCALE;
pub use crate::state::State;
pub use crate::state_manager::{enter_loop, Input, Manager, Vec2, Vec3};
pub use crate::texture::Texture;

pub use cgmath::vec2;
pub use pollster::block_on;
pub use winit::{dpi::LogicalSize, event_loop::EventLoop, window::WindowBuilder};
//pub use wgpu::Color;
