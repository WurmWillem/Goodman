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
