use egui_wgpu_backend::ScreenDescriptor;
use egui_winit_platform::Platform;
use std::collections::HashMap;
use winit::{
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::Window,
};

use crate::{
    camera::Camera,
    input::Input,
    instances::INDICES,
    instances::{self, Instance, InstanceRaw},
    math::rect,
    math::Rect,
    minor_types::{DrawParams, TimeManager},
    minor_types::{Feature, Features, GoodManUI, InstIndex, Layer, Manager, Sound, TexIndex},
    texture::{self, Texture},
};

mod engine_manager;

pub struct Engine {
    input: Input,

    window: Window,
    win_size: winit::dpi::PhysicalSize<u32>,
    win_background_color: wgpu::Color,
    window_bind_group: wgpu::BindGroup,

    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,

    render_pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    instance_buffer: wgpu::Buffer,
    camera_buffer: wgpu::Buffer,

    instances_raw: Vec<InstanceRaw>,
    instances_rendered: usize,

    tex_bindgroup_vec: Vec<wgpu::BindGroup>,
    layer_hash_inst_vec: HashMap<Layer, Vec<InstIndex>>,
    inst_hash_tex_index: HashMap<InstIndex, TexIndex>,
    texture_amt_created: u32,

    camera: Camera,
    camera_bind_group: wgpu::BindGroup,

    time: TimeManager,

    target_fps: Option<u32>,
    target_tps: Option<u32>,

    platform: Platform,
    egui_rpass: egui_wgpu_backend::RenderPass,

    features: Features,

    game_ui: Option<GoodManUI>,

    sound: Sound,
}
impl Engine {
    pub fn enter_loop<T>(mut self, mut manager: T, event_loop: EventLoop<()>)
    where
        T: Manager + 'static,
    {
        env_logger::init();

        let report_interval = match self.features.average_tps {
            Some(report_interval) => report_interval,
            None => 0.1,
        };

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
        render_pass.set_bind_group(2, &self.window_bind_group, &[]);

        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_vertex_buffer(1, self.instance_buffer.slice(..));
        render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);

        // let x = std::time::Instant::now();
        for layer in Layer::iterator().rev() {
            if let Some(inst_vec) = self.layer_hash_inst_vec.get_mut(layer) {
                for i in inst_vec.drain(..) {
                    if let Some(tex_bind_index) = self.inst_hash_tex_index.get(&i) {
                        let tex_bind_index = *tex_bind_index as usize;
                        render_pass.set_bind_group(0, &self.tex_bindgroup_vec[tex_bind_index], &[]);
                        render_pass.draw_indexed(0..INDICES.len() as u32, 0, i..(i + 1));
                    }
                }
            }
        }
        // let x = x.elapsed().as_micros(); //~230 micro, 10k tex
        // println!("{x}");
        /*
        foreach layer
            if an instance is in layer
                foreach instance in layer
                    if tex in instance
                        bind(tex)
                        draw(instance)
        */

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

        self.instances_raw = Vec::with_capacity(self.instances_rendered);
        self.instances_rendered = 0;
        self.time.reset_time_since_last_render();
        Ok(())
    }

    pub fn enable_feature(&mut self, feature: Feature) {
        self.features.enable_feature(feature);
    }

    fn render_ui(&self) {
        if !self.features.engine_ui_enabled {
            return;
        }

        egui::Window::new("Engine").show(&self.platform.context(), |ui| {
            let tps_points: egui::plot::PlotPoints = self
                .time
                .graph_vec
                .iter()
                .map(|vec| [vec.x, vec.y])
                .collect();
            let line = egui::plot::Line::new(tps_points);

            egui::plot::Plot::new("sd")
                .view_aspect(2.)
                .include_y(0.)
                .show(ui, |plot_ui| plot_ui.line(line));

            ui.label(format!(
                "window size: {:?}x{:?}",
                self.win_size.width, self.win_size.height
            ));
            let fps = match self.target_fps {
                Some(fps) => fps,
                None => self.get_average_tps(),
            };
            ui.label(format!("FPS: {:?}", fps));
            ui.label(format!("TPS: {:?}", self.get_average_tps()));
            ui.label(format!(
                "textures rendered this frame: {:?}",
                self.instances_rendered
            ));
        });
    }

    pub fn set_game_ui(&mut self, user_ui: GoodManUI) {
        if !self.features.game_ui_enabled {
            println!("game ui is disabled");
            return;
        }
        self.game_ui = Some(user_ui);
    }

    fn render_game_ui(&self, game_ui: &GoodManUI) {
        egui::Window::new(game_ui.title.clone()).show(&self.platform.context(), |ui| {
            for label in &game_ui.labels {
                ui.label(label);
            }
        });
    }

    pub fn render_texture(&mut self, rect: &Rect, texture: &Texture) {
        self.render_tex(rect, texture, 0., Layer::Layer1);
    }
    pub fn render_texture_ex(&mut self, rect: &Rect, texture: &Texture, draw_params: DrawParams) {
        self.render_tex(rect, texture, draw_params.rotation, draw_params.layer);
    }
    fn render_tex(&mut self, rect_: &Rect, texture: &Texture, rotation: f64, layer: Layer) {
        let width = rect_.w / self.win_size.width as f64;
        let height = rect_.h / self.win_size.height as f64;
        let rect = rect(rect_.x, rect_.y, width, height);
        let inst_raw = Instance::new(rect, rotation).to_raw();

        self.instances_raw.push(inst_raw);

        self.inst_hash_tex_index
            .insert(self.instances_rendered as u32, texture.index);

        match self.layer_hash_inst_vec.get_mut(&layer) {
            Some(instance_vec) => instance_vec.push(self.instances_rendered as u32),
            None => {
                self.layer_hash_inst_vec
                    .insert(layer, vec![self.instances_rendered as u32]);
            }
        }

        self.instances_rendered += 1;
    }

    fn update_instance_buffer(&mut self) {
        if self.instance_buffer.size() == self.instances_raw.len() as u64 * 24 {
            self.queue.write_buffer(
                &self.instance_buffer,
                0,
                bytemuck::cast_slice(&self.instances_raw),
            );
        } else {
            self.instance_buffer = instances::create_buffer(&self.device, &self.instances_raw);
        }
    }

    fn handle_rendering<T>(&mut self, manager: &mut T, control_flow: &mut ControlFlow)
    where
        T: Manager + 'static,
    {
        manager.render(self);
        self.update_instance_buffer();
        match self.render() {
            Ok(_) => {}
            // Reconfigure the surface if lost
            Err(wgpu::SurfaceError::Lost) => self.resize(self.get_size()),
            // The system is out of memory, we should probably quit
            Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
            // All other errors (Outdated, Timeout) should be resolved by the next frame
            Err(e) => eprintln!("{e:?}"),
        }
    }

    fn update(&mut self) {
        if self.camera.movement_enabled && self.camera.update(&self.input) {
            self.queue.write_buffer(
                &self.camera_buffer,
                0,
                bytemuck::cast_slice(&[self.camera.uniform]),
            );
        }
    }

    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.win_size = new_size;
        self.config.width = new_size.width;
        self.config.height = new_size.height;
        self.surface.configure(&self.device, &self.config);
    }

    fn input(&mut self, event: &WindowEvent) -> bool {
        self.input.process_events(event)
    }

    fn handle_window_event(&mut self, event: &WindowEvent, control_flow: &mut ControlFlow) {
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
}
