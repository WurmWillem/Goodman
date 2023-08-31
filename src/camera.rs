use wgpu::{util::DeviceExt, Device};

use crate::{input::Input, prelude::Button};

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    pos: [f32; 2],
}
impl CameraUniform {
    pub fn new() -> Self {
        Self { pos: [0., 0.] }
    }
}

pub struct Camera {
    pub uniform: CameraUniform,
    movement_enabled: bool,
}
impl Camera {
    const SPEED: f32 = 0.0001;
    pub fn new(movement_enabled: bool) -> Self {
        Self {
            movement_enabled,
            uniform: CameraUniform::new(),
        }
    }
    pub fn update(&mut self, input: &Input) -> bool {
        if !self.movement_enabled {
            false
        } else if input.is_button_held(Button::D) {
            self.uniform.pos[0] += Camera::SPEED;
            true
        } else if input.is_button_held(Button::A) {
            self.uniform.pos[0] -= Camera::SPEED;
            true
        } else if input.is_button_held(Button::W) {
            self.uniform.pos[1] += Camera::SPEED;
            true
        } else if input.is_button_held(Button::S) {
            self.uniform.pos[1] -= Camera::SPEED;
            true
        } else {
            false
        }
    }
}

pub fn create_buffer(device: &Device, uniform: CameraUniform) -> wgpu::Buffer {
    device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Camera Buffer"),
        contents: bytemuck::cast_slice(&[uniform]),
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
    })
}
pub fn create_bind_group_layout(device: &Device) -> wgpu::BindGroupLayout {
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        entries: &[wgpu::BindGroupLayoutEntry {
            binding: 0,
            visibility: wgpu::ShaderStages::VERTEX,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Uniform,
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        }],
        label: Some("camera_bind_group_layout"),
    })
}
pub fn create_bind_group(
    device: &Device,
    camera_buffer: &wgpu::Buffer,
    camera_bind_group_layout: &wgpu::BindGroupLayout,
) -> wgpu::BindGroup {
    device.create_bind_group(&wgpu::BindGroupDescriptor {
        layout: camera_bind_group_layout,
        entries: &[wgpu::BindGroupEntry {
            binding: 0,
            resource: camera_buffer.as_entire_binding(),
        }],
        label: Some("camera_bind_group"),
    })
}
