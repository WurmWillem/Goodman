use cgmath::{vec2, vec3, Deg, Matrix4, Vector4};
use wgpu::{util::DeviceExt, Device};

use crate::{
    math::Rect,
    minor_types::{Vec2, Vec3},
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Instance {
    pub pos: Vec3,
    pub size: Vec2,
    pub rotation: f64,
}
impl Instance {
    pub fn new(rect: Rect) -> Self {
        Self {
            pos: vec3(rect.x, rect.y, 0.),
            size: vec2(rect.w, rect.h),
            rotation: 0.,
        }
    }

    pub fn to_raw(&self) -> InstanceRaw {
        let matrix4 = Matrix4::from_translation(self.pos)
            * Matrix4::from_angle_z(Deg(self.rotation))
            * Matrix4::from_nonuniform_scale(self.size.x, self.size.y, 1.);

        let x = get_f32_array_from_vec4_f64(matrix4.x);
        let y = get_f32_array_from_vec4_f64(matrix4.y);
        let z = get_f32_array_from_vec4_f64(matrix4.z);
        let w = get_f32_array_from_vec4_f64(matrix4.w);

        InstanceRaw {
            model: [x, y, z, w],
        }
    }
}

fn get_f32_array_from_vec4_f64(vec: Vector4<f64>) -> [f32; 4] {
    [vec.x as f32, vec.y as f32, vec.z as f32, vec.w as f32]
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct InstanceRaw {
    model: [[f32; 4]; 4],
}
impl InstanceRaw {
    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        use std::mem;
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<InstanceRaw>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 5,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 4]>() as wgpu::BufferAddress,
                    shader_location: 6,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 8]>() as wgpu::BufferAddress,
                    shader_location: 7,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 12]>() as wgpu::BufferAddress,
                    shader_location: 8,
                    format: wgpu::VertexFormat::Float32x4,
                },
            ],
        }
    }
}

pub fn create_buffer(device: &Device, instance_data: &[InstanceRaw]) -> wgpu::Buffer {
    device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Instance Buffer"),
        contents: bytemuck::cast_slice(instance_data),
        usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
    })
}
