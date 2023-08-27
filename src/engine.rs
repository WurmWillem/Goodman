use egui_wgpu_backend::ScreenDescriptor;
use egui_winit_platform::Platform;
use winit::{event::Event, event_loop::EventLoop, window::Window};

use crate::{
    camera::Camera,
    input::Input,
    math::Rect,
    minor_types::{DrawParams, TimeManager},
    minor_types::{Features, GoodManUI, Manager, Sound},
    prelude::Vec2,
    texture::Texture,
    vert_buffers::INDICES,
    vert_buffers::{self, Instance},
};

#[allow(unused_imports)]
use std::time::Instant;

mod engine_manager;

pub struct Engine {
    input: Input,

    window: Window,
    win_size: winit::dpi::PhysicalSize<u32>,
    inv_win_size: Vec2,
    win_background_color: wgpu::Color,
    win_bind_group: wgpu::BindGroup,

    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,

    render_pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    instance_buffer: wgpu::Buffer,
    camera_buffer: wgpu::Buffer,

    instances: Vec<Instance>,
    instances_rendered: usize,

    tex_bind: Option<wgpu::BindGroup>,
    texture_amt_created: u32,

    camera: Camera,
    camera_bind_group: wgpu::BindGroup,

    time: TimeManager,

    target_fps: Option<u32>,
    target_tps: Option<u32>,

    platform: Platform,
    egui_rpass: egui_wgpu_backend::RenderPass,
    game_ui: Option<GoodManUI>,

    features: Features,

    sound: Sound,
}
impl Engine {
    pub fn enter_loop<T>(mut self, mut manager: T, event_loop: EventLoop<()>)
    where
        T: Manager + 'static,
    {
        env_logger::init();

        let report_interval = self.features.average_tps.unwrap_or(0.1);
        // If target_fps in some and target_tps is None than the loop helper will run at fps
        let fps = match self.target_fps {
            Some(fps) => {
                self.time.set_use_target_tps(true);
                fps
            }
            None => 1000, // Doesn't matter because if target_fps is None and target_tps is None than use_target_tps is false
        };

        let target_tps = match self.target_tps {
            Some(tps) => tps,
            None => fps,
        };

        self.time.replace_loop_helper(report_interval, target_tps);
        manager.start();

        event_loop.run(move |event, _, control_flow| {
            self.platform.handle_event(&event);

            match event {
                Event::WindowEvent {
                    ref event,
                    window_id,
                } if window_id == self.window.id() => {
                    if !self.input(event) {
                        self.handle_window_event(event, control_flow);
                    }
                }
                Event::MainEventsCleared => {
                    self.time.update(&mut self.platform);

                    self.update();
                    manager.update(self.time.get_relevant_delta_t(), &self.input, &self.sound);

                    if self
                        .input
                        .is_button_pressed(crate::prelude::Button::RightMouse)
                    {
                        println!("{}", self.time.get_average_tps());
                    }
                    self.input.reset_buttons();

                    match self.target_fps {
                        Some(fps) => {
                            if self.target_tps.is_some() {
                                if self.time.get_time_since_last_render() >= 0.995 / fps as f64 {
                                    self.window.request_redraw();
                                }
                            } else {
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
        render_pass.set_vertex_buffer(1, self.instance_buffer.slice(..));
        render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);

        // println!("{}", )
        if let Some(tex_bind) = &self.tex_bind {
            render_pass.set_bind_group(0, tex_bind, &[]);
        }
        render_pass.draw_indexed(
            0..INDICES.len() as u32,
            0,
            0..self.instances_rendered as u32,
        );

        if self.features.engine_ui_enabled || self.features.game_ui_enabled {
            self.time.update_graph();

            // Begin to draw the UI frame.
            self.platform.begin_frame();

            self.render_ui();
            if let Some(game_ui) = &self.game_ui {
                self.render_game_ui(game_ui);
            }

            let full_output = self.platform.end_frame(Some(&self.window));
            let paint_jobs = self.platform.context().tessellate(full_output.shapes);

            // Upload all resources for the GPU.
            let screen_descriptor = ScreenDescriptor {
                physical_width: self.config.width,
                physical_height: self.config.height,
                scale_factor: self.window.scale_factor() as f32,
            };
            let tdelta: egui::TexturesDelta = full_output.textures_delta;
            self.egui_rpass
                .add_textures(&self.device, &self.queue, &tdelta)
                .expect("add texture ok");

            self.egui_rpass.update_buffers(
                &self.device,
                &self.queue,
                &paint_jobs,
                &screen_descriptor,
            );

            self.egui_rpass
                .remove_textures(tdelta)
                .expect("remove texture ok");

            self.egui_rpass
                .execute_with_renderpass(&mut render_pass, &paint_jobs, &screen_descriptor)
                .unwrap();
        }

        drop(render_pass);
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        self.instances = Vec::with_capacity(self.instances_rendered);
        self.instances_rendered = 0;
        self.time.reset_time_since_last_render();
        Ok(())
    }

    pub fn render_texture(&mut self, rect: &Rect, texture: &Texture) {
        self.render_tex(rect, texture, 0.);
    }
    pub fn render_texture_ex(&mut self, rect: &Rect, texture: &Texture, draw_params: DrawParams) {
        self.render_tex(rect, texture, draw_params.rotation);
    }
    fn render_tex(&mut self, rect: &Rect, texture: &Texture, rotation: f64) {
        let width = rect.w * self.inv_win_size.x;
        let height = rect.h * self.inv_win_size.y;
        let inst = Instance::new(rect.x, rect.y, width, height, rotation, texture.index);

        self.instances.push(inst);
        self.instances_rendered += 1;
    }

    fn update_instance_buffer(&mut self) {
        if self.instance_buffer.size() == self.instances.len() as u64 * 24 {
            self.queue.write_buffer(
                &self.instance_buffer,
                0,
                bytemuck::cast_slice(&self.instances),
            );
        } else {
            self.instance_buffer = vert_buffers::create_buffer(&self.device, &self.instances);
        }
    }
}
