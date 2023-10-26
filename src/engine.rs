use wgpu::{BindGroup, Buffer};
use winit::{event::Event, event_loop::EventLoop, window::Window};

use crate::{
    camera::Camera,
    input::Input,
    math::Rect32,
    minor_types::{DrawParams, Manager},
    prelude::Sound,
    texture::Texture,
    time::TimeManager,
    ui::Ui,
    vert_buffers::{self, Instance, TexCoords},
};

mod engine_manager;

pub struct Engine {
    input: Input,
    time: TimeManager,
    ui: Ui,
    sound: Sound,

    window: Window,
    win_size: winit::dpi::PhysicalSize<u32>,
    win_background_color: wgpu::Color,
    win_bind_group: BindGroup,

    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,

    render_pipeline: wgpu::RenderPipeline,
    vertex_buffer: Buffer,
    index_buffer: Buffer,
    tex_coords_buffer: Buffer,

    instances: Vec<Instance>,
    tex_coords: Vec<TexCoords>,
    instances_rendered: usize,
    instance_buffer: Buffer,

    tex_bind: Option<BindGroup>,
    texture_amt_created: u32,
    use_near_filter_mode: bool,

    camera: Camera,
    camera_bind_group: BindGroup,
    camera_buffer: Buffer,

    target_fps: Option<u32>,
}
impl Engine {
    pub fn start_loop<T>(mut self, mut manager: T, event_loop: EventLoop<()>)
    where
        T: Manager + 'static,
    {
        env_logger::init();
        manager.start();

        event_loop.run(move |event, _, control_flow| {
            self.ui.platform.handle_event(&event);

            match event {
                Event::WindowEvent {
                    ref event,
                    window_id,
                } if window_id == self.window.id() => {
                    if !self.input.process_events(event) {
                        self.handle_window_event(event, control_flow);
                    }
                }
                Event::MainEventsCleared => {
                    self.time.update(&mut self.ui);

                    self.update_cam();
                    manager.update(
                        self.time.get_relevant_delta_t(),
                        &self.input,
                        &mut self.sound,
                    );

                    if self
                        .input
                        .is_button_pressed(crate::prelude::Button::RightMouse)
                    {
                        println!("{}", self.time.get_avg_tps());
                    }
                    self.input.reset_buttons();

                    match self.target_fps {
                        Some(fps) => {
                            if self.time.get_time_since_last_render() >= 0.95 / fps as f64 {
                                self.window.request_redraw();
                            }
                        }
                        None => {
                            self.window.request_redraw();
                        }
                    }
                }
                Event::RedrawRequested(window_id) if window_id == self.window.id() => {
                    self.handle_rendering(&mut manager, control_flow);
                }
                _ => {}
            }
        });
    }

    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(self.win_background_color),
                    store: true,
                },
            })],
            depth_stencil_attachment: None,
        });

        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.set_bind_group(1, &self.camera_bind_group, &[]);
        render_pass.set_bind_group(2, &self.win_bind_group, &[]);

        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_vertex_buffer(1, self.tex_coords_buffer.slice(..));
        render_pass.set_vertex_buffer(2, self.instance_buffer.slice(..));
        render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);

        if let Some(tex_bind) = &self.tex_bind {
            render_pass.set_bind_group(0, tex_bind, &[]);
            render_pass.draw_indexed(0..6, 0, 0..self.instances_rendered as u32);
        }

        if self.ui.should_render() {
            // Begin to draw the UI frame.
            self.ui.platform.begin_frame();

            self.ui.render_engine(
                self.win_size,
                &self.time,
                self.target_fps,
                self.instances_rendered,
            );
            self.ui.render_game();

            let (paint_jobs, screen_descriptor) =
                self.ui
                    .update_egui_rpass(&self.window, &self.config, &self.device, &self.queue);

            self.ui
                .egui_rpass
                .execute_with_renderpass(&mut render_pass, &paint_jobs, &screen_descriptor)
                .unwrap();
        }

        drop(render_pass);
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        self.instances = Vec::with_capacity(self.instances_rendered);
        self.tex_coords = Vec::with_capacity(self.instances_rendered);
        self.instances_rendered = 0;
        self.time.enable_prev_iter_was_render();
        Ok(())
    }

    pub fn render_texture(&mut self, rect: Rect32, texture: &Texture) {
        self.render_tex(rect, texture, 0., TexCoords::default());
    }
    pub fn render_texture_ex(&mut self, rect: Rect32, texture: &Texture, draw_params: DrawParams) {
        let tex_coords = match draw_params.source {
            Some(rect) => TexCoords::from_rect_tex(rect, texture),
            None => TexCoords::default(),
        };
        self.render_tex(rect, texture, draw_params.rotation, tex_coords);
    }
    fn render_tex(&mut self, rect: Rect32, tex: &Texture, rotation: f32, tex_coords: TexCoords) {
        let inst = Instance::new(rect, rotation, tex.index);

        self.instances.push(inst);
        self.tex_coords.push(tex_coords);

        self.instances_rendered += 1;
    }

    fn update_instance_buffer(&mut self) {
        if self.instance_buffer.size() == self.instances.len() as u64 * 28 {
            self.queue.write_buffer(
                &self.instance_buffer,
                0,
                bytemuck::cast_slice(&self.instances),
            );
        } else {
            self.instance_buffer = vert_buffers::create_inst_buffer(&self.device, &self.instances);
        }

        if self.tex_coords_buffer.size() == self.tex_coords.len() as u64 * 32 {
            self.queue.write_buffer(
                &self.tex_coords_buffer,
                0,
                bytemuck::cast_slice(&self.tex_coords),
            );
        } else {
            self.tex_coords_buffer =
                vert_buffers::create_tex_coords_buffer(&self.device, &self.tex_coords);
        }
    }
}
