use crate::create_Engine_from_AllFields;
use crate::engine::Engine;
use crate::engine_builder::AllFields;
use crate::prelude::{Color, Manager};
use crate::texture::{self, Texture};
use winit::{
    event::{ElementState, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::ControlFlow,
};

impl Engine {
    pub(crate) fn handle_rendering<T>(&mut self, manager: &T, control_flow: &mut ControlFlow)
    where
        T: Manager + 'static,
    {
        manager.render(self);
        self.update_instance_buffer();
        match self.render() {
            Ok(_) => {}
            // Reconfigure the surface if lost
            Err(wgpu::SurfaceError::Lost) => self.resize(self.win_size),
            // The system is out of memory, we should probably quit
            Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
            // All other errors (Outdated, Timeout) should be resolved by the next frame
            Err(e) => eprintln!("{e:?}"),
        }
    }

    pub(crate) fn update(&mut self) {
        if self.camera.movement_enabled && self.camera.update(&self.input) {
            self.queue.write_buffer(
                &self.camera_buffer,
                0,
                bytemuck::cast_slice(&[self.camera.uniform]),
            );
        }
    }

    pub(crate) fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.win_size = new_size;
        self.config.width = new_size.width;
        self.config.height = new_size.height;
        self.surface.configure(&self.device, &self.config);
    }

    pub(crate) fn input(&mut self, event: &WindowEvent) -> bool {
        self.input.process_events(event)
    }

    pub(crate) fn handle_window_event(
        &mut self,
        event: &WindowEvent,
        control_flow: &mut ControlFlow,
    ) {
        match event {
            WindowEvent::CloseRequested
            | WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        state: ElementState::Pressed,
                        virtual_keycode: Some(VirtualKeyCode::Escape),
                        ..
                    },
                ..
            } => *control_flow = ControlFlow::Exit,
            WindowEvent::Resized(physical_size) => {
                self.resize(*physical_size);
            }
            WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                self.resize(**new_inner_size);
            }
            _ => {}
        }
    }

    pub(crate) fn new(all_fields: AllFields) -> Engine {
        create_Engine_from_AllFields!(all_fields, input window win_bind_group win_size inv_win_size win_background_color
        surface device queue config render_pipeline vertex_buffer index_buffer camera camera_bind_group
        camera_buffer instance_buffer instances instances_rendered time tex_bind
        texture_amt_created target_fps sound ui)
    }
    pub fn play_sound<S>(&self, source: S) -> Result<(), rodio::PlayError>
    where
        S: rodio::Source<Item = f32> + Send + 'static,
    {
        self.sound.play_sound(source)
    }

    pub fn use_textures(&mut self, textures: &Vec<Texture>, tex_amt: u32) {
        let tex_bind_group_layout = texture::create_bind_group_layout(&self.device, tex_amt);
        self.tex_bind = Some(texture::create_bind_group(
            &self.device,
            &tex_bind_group_layout,
            textures,
        ));
    }

    pub fn create_texture(&mut self, bytes: &[u8]) -> Result<Texture, &'static str> {
        let tex =
            match Texture::from_bytes(&self.device, &self.queue, self.texture_amt_created, bytes) {
                Ok(tex) => tex,
                Err(_) => return Err("failed to create texture"),
            };

        self.texture_amt_created += 1;
        Ok(tex)
    }

    pub fn get_avg_tps(&self) -> u32 {
        self.time.get_avg_tps()
    }
    pub fn get_avg_fps(&self) -> u32 {
        self.time.get_avg_fps()
    }
    pub fn get_time_since_last_render(&self) -> f64 {
        self.time.get_time_since_last_render()
    }

    pub fn set_background_color(&mut self, color: Color) {
        self.win_background_color = wgpu::Color {
            r: color.r,
            g: color.g,
            b: color.b,
            a: color.a,
        }
    }
}
