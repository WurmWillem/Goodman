use cgmath::{vec3, Deg, Matrix4, Vector4};
use wgpu::{util::DeviceExt, Device};

//use crate::object_data::VERTEX_SCALE;

//const INSTANCES_PER_ROW: u32 = 5;
//const INSTANCE_DISPLACEMENT: f64 = 1.;

type Vec2 = cgmath::Vector2<f64>;
type Vec3 = cgmath::Vector3<f64>;

/*pub fn create_instances() -> Vec<SquareInstance> {
    (0..INSTANCES_PER_ROW)
        .flat_map(|y| {
            (0..INSTANCES_PER_ROW).map(move |x| {
                let position = cgmath::Vector3 {
                    x: x as f64 * VERTEX_SCALE as f64 * 2.3 - INSTANCE_DISPLACEMENT,
                    y: y as f64 * VERTEX_SCALE as f64 * 4.6 - INSTANCE_DISPLACEMENT,
                    z: 0.,
                };
                let rotation = 0.;
                let scale = vec2(1., 1.);
                SquareInstance {
                    pos: position,
                    rotation,
                    size: scale,
                }
            })
        })
        .collect::<Vec<_>>()
}*/

pub struct CircleInstance {
    pub pos: Vec3,
    pub radius: f64,
}
impl CircleInstance {
    pub fn new(pos: Vec2, radius: f64) -> Self {
        Self {
            pos: vec3(pos.x, pos.y, 1.),
            radius,
        }
    }

    pub fn to_raw(&self) -> InstanceRaw {
        let matrix4 = Matrix4::from_translation(self.pos) * Matrix4::from_scale(self.radius);

        let x = get_f32_array_from_vec4_f64(matrix4.x);
        let y = get_f32_array_from_vec4_f64(matrix4.y);
        let z = get_f32_array_from_vec4_f64(matrix4.z);
        let w = get_f32_array_from_vec4_f64(matrix4.w);

        InstanceRaw {
            model: [x, y, z, w],
        }
    }
}

pub struct SquareInstance {
    pub pos: Vec3,
    pub size: Vec2,
    pub rotation: f64,
}
impl SquareInstance {
    pub fn new(pos: Vec2, size: Vec2) -> Self {
        Self {
            pos: vec3(pos.x, pos.y, 1.),
            size,
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

pub trait SquareInstanceT {
    fn to_square_instance(&self) -> SquareInstance;
}
pub trait CircleInstanceT {
    fn to_circle_instance(&self) -> CircleInstance;
}
