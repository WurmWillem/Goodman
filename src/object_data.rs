use wgpu::util::DeviceExt;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    position: [f32; 3],
    tex_coords: [f32; 2],
}
impl Vertex {
    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x2,
                },
            ],
        }
    }
}

const VERTEX_SCALED: f32 = 0.05;
pub const VERTEX_SCALE: f64 = VERTEX_SCALED as f64;

#[rustfmt::skip]
pub const VERTICES: &[Vertex] = &[
    Vertex { position: [-1. * VERTEX_SCALED, -1. * VERTEX_SCALED, 0.0], tex_coords: [0.0, 1.0], },
    Vertex { position: [1. * VERTEX_SCALED, -1. * VERTEX_SCALED, 0.0], tex_coords: [1.0, 1.0], },
    Vertex { position: [1. * VERTEX_SCALED, 1. * VERTEX_SCALED, 0.0], tex_coords: [1.0, 0.0], },
    Vertex { position: [-1. * VERTEX_SCALED, 1. * VERTEX_SCALED, 0.0], tex_coords: [0.0, 0.0], },
];

#[rustfmt::skip]
pub const INDICES: &[u16] = &[
    0, 1, 2, 2, 3, 0, 
];

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
