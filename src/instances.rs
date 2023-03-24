use cgmath::{Basis3, Deg, Rotation, Rotation3};

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
    rotation: f64,
}
impl Instance {
    pub fn to_raw(&self) -> InstanceRaw {
        let rot: Basis3<f64> = Rotation3::from_angle_z(Deg(self.rotation));
        let model = rot.rotate_vector(self.position);
        InstanceRaw {
            model: [model.x as f32, model.y as f32, model.z as f32],
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct InstanceRaw {
    model: [f32; 3],
}
impl InstanceRaw {
    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        use std::mem;
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<InstanceRaw>() as wgpu::BufferAddress,
            // We need to switch from using a step mode of Vertex to Instance
            // This means that our shaders will only change to use the next
            // instance when the shader starts processing a new instance
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &[wgpu::VertexAttribute {
                offset: 0,
                shader_location: 2,
                format: wgpu::VertexFormat::Float32x3,
            }],
        }
    }
}
