use cgmath::{vec3, Deg, Matrix4};
use wgpu::{util::DeviceExt, Device};

use crate::prelude::{Rect32, Texture};

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct TexCoords {
    pub coords: [[f32; 2]; 4],
}
impl TexCoords {
    pub fn default() -> Self {
        Self {
            coords: ([[0., 1.], [1., 1.], [1., 0.], [0., 0.]]),
        }
    }

    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        use std::mem::size_of;
        wgpu::VertexBufferLayout {
            array_stride: size_of::<TexCoords>() as u64,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x2,
                },
                wgpu::VertexAttribute {
                    offset: size_of::<[f32; 2]>() as u64,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float32x2,
                },
                wgpu::VertexAttribute {
                    offset: size_of::<[f32; 4]>() as u64,
                    shader_location: 3,
                    format: wgpu::VertexFormat::Float32x2,
                },
                wgpu::VertexAttribute {
                    offset: size_of::<[f32; 6]>() as u64,
                    shader_location: 4,
                    format: wgpu::VertexFormat::Float32x2,
                },
            ],
        }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Instance {
    model: [[f32; 2]; 3],
    index: u32,
}
impl Instance {
    pub fn new(r: Rect32, rotation: f32, index: u32) -> Self {
        let mat4 = Matrix4::from_translation(vec3(r.x, r.y, 0.))
            * Matrix4::from_angle_z(Deg(rotation))
            * Matrix4::from_nonuniform_scale(r.w, r.h, 1.);

        let x = [mat4.x.x, mat4.x.y];
        let y = [mat4.y.x, mat4.y.y];
        let w = [mat4.w.x, mat4.w.y];

        Self {
            model: [x, y, w],
            index,
        }
    }
    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        use std::mem;
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<Instance>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 5,
                    format: wgpu::VertexFormat::Float32x2,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 2]>() as wgpu::BufferAddress,
                    shader_location: 6,
                    format: wgpu::VertexFormat::Float32x2,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 4]>() as wgpu::BufferAddress,
                    shader_location: 7,
                    format: wgpu::VertexFormat::Float32x2,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 6]>() as wgpu::BufferAddress,
                    shader_location: 8,
                    format: wgpu::VertexFormat::Uint32,
                },
            ],
        }
    }
}

#[rustfmt::skip]
pub const VERTICES: &[Vertex] = &[
    Vertex { position: [0., -2.]},
    Vertex { position: [2., -2.]},
    Vertex { position: [2., 0.]},
    Vertex { position: [0., 0.]},
];

#[rustfmt::skip]
pub const INDICES: &[u16] = &[
    0, 1, 2, 2, 3, 0, 
];

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    position: [f32; 2],
}
impl Vertex {
    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[wgpu::VertexAttribute {
                offset: 0,
                shader_location: 0,
                format: wgpu::VertexFormat::Float32x2,
            }],
        }
    }
}

impl TexCoords {
    pub fn from_rect_tex(mut r: crate::math::Rect32, tex: &Texture) -> TexCoords {
        r.x *= tex.get_inv_width();
        r.w *= tex.get_inv_width();
        r.y *= tex.get_inv_height();
        r.h *= tex.get_inv_height();

        let b = r.y + r.h;
        let c = r.x + r.w;
        let d = r.y + r.h;
        let e = r.x + r.w;

        TexCoords {
            coords: [[r.x, b], [c, d], [e, r.y], [r.x, r.y]],
        }
    }
}

pub fn create_buffers(device: &wgpu::Device) -> (wgpu::Buffer, wgpu::Buffer) {
    let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Vertex Buffer"),
        contents: bytemuck::cast_slice(VERTICES),
        usage: wgpu::BufferUsages::VERTEX,
    });

    let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Index Buffer"),
        contents: bytemuck::cast_slice(INDICES),
        usage: wgpu::BufferUsages::INDEX,
    });
    (vertex_buffer, index_buffer)
}

pub fn create_tex_coords_buffer(device: &Device, tex_coords_data: &[TexCoords]) -> wgpu::Buffer {
    device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Texture Coordinates Buffer"),
        contents: bytemuck::cast_slice(tex_coords_data),
        usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
    })
}

pub fn create_inst_buffer(device: &Device, instance_data: &[Instance]) -> wgpu::Buffer {
    device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Instance Buffer"),
        contents: bytemuck::cast_slice(instance_data),
        usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
    })
}
