use wgpu::{util::DeviceExt, Device};
use winit::event::{ElementState, KeyboardInput, VirtualKeyCode, WindowEvent};

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
    is_right_pressed: bool,
    is_left_pressed: bool,
    is_up_pressed: bool,
    is_down_pressed: bool,
}
impl Camera {
    const SPEED: f32 = 0.003;
    pub fn new() -> Self {
        Self {
            uniform: CameraUniform::new(),
            is_right_pressed: false,
            is_left_pressed: false,
            is_up_pressed: false,
            is_down_pressed: false,
        }
    }
    pub fn process_events(&mut self, event: &WindowEvent) -> bool {
        match event {
            WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        state,
                        virtual_keycode: Some(keycode),
                        ..
                    },
                ..
            } => {
                let is_pressed = *state == ElementState::Pressed;
                match keycode {
                    VirtualKeyCode::W | VirtualKeyCode::Up => {
                        self.is_up_pressed = is_pressed;
                        true
                    }
                    VirtualKeyCode::A | VirtualKeyCode::Left => {
                        self.is_left_pressed = is_pressed;
                        true
                    }
                    VirtualKeyCode::S | VirtualKeyCode::Down => {
                        self.is_down_pressed = is_pressed;
                        true
                    }
                    VirtualKeyCode::D | VirtualKeyCode::Right => {
                        self.is_right_pressed = is_pressed;
                        true
                    }
                    _ => false,
                }
            }
            _ => false,
        }
    }
    pub fn update(&mut self) {
        if self.is_right_pressed {
            self.uniform.pos[0] += Camera::SPEED;
        }
        if self.is_left_pressed {
            self.uniform.pos[0] -= Camera::SPEED;
        }
        if self.is_up_pressed {
            self.uniform.pos[1] += Camera::SPEED;
        }
        if self.is_down_pressed {
            self.uniform.pos[1] -= Camera::SPEED;
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
        layout: &camera_bind_group_layout,
        entries: &[wgpu::BindGroupEntry {
            binding: 0,
            resource: camera_buffer.as_entire_binding(),
        }],
        label: Some("camera_bind_group"),
    })
}
