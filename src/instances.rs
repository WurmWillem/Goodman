use cgmath::{Deg, Vector4};
use wgpu::{Device, util::DeviceExt};

use crate::vertices::VERTEX_SCALE;

const INSTANCES_PER_ROW: u32 = 10;
const INSTANCE_DISPLACEMENT: f64 = 1.;

pub fn create_instances() -> Vec<Instance> {
    (0..INSTANCES_PER_ROW)
        .flat_map(|y| {
            (0..INSTANCES_PER_ROW).map(move |x| {
                let position = cgmath::Vector3 {
                    x: x as f64 * VERTEX_SCALE as f64 * 2.3 - INSTANCE_DISPLACEMENT,
                    y: y as f64 * VERTEX_SCALE as f64 * 2.3 - INSTANCE_DISPLACEMENT,
                    z: 0.,
                };
                let rotation = 0.;
                Instance { position, rotation }
            })
        })
        .collect::<Vec<_>>()
}

pub struct Instance {
    position: cgmath::Vector3<f64>,
    pub rotation: f64,
}
impl Instance {
    pub fn to_raw(&self) -> InstanceRaw {
        let matrix4 = cgmath::Matrix4::from_translation(self.position)
            * cgmath::Matrix4::from_angle_z(Deg(self.rotation));
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
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
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


pub fn create_buffer(device: &Device, instance_data: &Vec<InstanceRaw>) -> wgpu::Buffer {
    device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Instance Buffer"),
        contents: bytemuck::cast_slice(instance_data),
        usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
    })
}