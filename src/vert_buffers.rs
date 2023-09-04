use cgmath::{vec3, Deg, Matrix4};
use wgpu::{util::DeviceExt, Device};

pub const DEFAULT_TEX_COORDS: [TexCoords; 4] = [
    TexCoords { coords: [0., 1.] },
    TexCoords { coords: [1., 1.] },
    TexCoords { coords: [1., 0.] },
    TexCoords { coords: [0., 0.] },
];

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct TexCoords {
    pub coords: [f32; 2],
}
impl TexCoords {
    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        use std::mem;
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<TexCoords>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[wgpu::VertexAttribute {
                offset: 0,
                shader_location: 1,
                format: wgpu::VertexFormat::Float32x2,
            }],
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
    pub fn new(x: f64, y: f64, width: f64, height: f64, rotation: f64, index: u32) -> Self {
        let mat4 = Matrix4::from_translation(vec3(x, y, 0.))
            * Matrix4::from_angle_z(Deg(rotation))
            * Matrix4::from_nonuniform_scale(width, height, 1.);

        let x = [mat4.x.x as f32, mat4.x.y as f32];
        let y = [mat4.y.x as f32, mat4.y.y as f32];
        let w = [mat4.w.x as f32, mat4.w.y as f32];

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
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float32x2,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 2]>() as wgpu::BufferAddress,
                    shader_location: 3,
                    format: wgpu::VertexFormat::Float32x2,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 4]>() as wgpu::BufferAddress,
                    shader_location: 4,
                    format: wgpu::VertexFormat::Float32x2,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 6]>() as wgpu::BufferAddress,
                    shader_location: 5,
                    format: wgpu::VertexFormat::Uint32,
                },
            ],
        }
    }
}

const VERTEX_SCALE: f32 = 1.;
#[rustfmt::skip]
pub const VERTICES: &[Vertex] = &[
    Vertex { position: [0. * VERTEX_SCALE, -2. * VERTEX_SCALE]},
    Vertex { position: [2. * VERTEX_SCALE, -2. * VERTEX_SCALE]},
    Vertex { position: [2. * VERTEX_SCALE, 0. * VERTEX_SCALE]},
    Vertex { position: [0. * VERTEX_SCALE, 0. * VERTEX_SCALE]},
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
