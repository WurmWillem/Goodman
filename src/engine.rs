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
    instances::INDICES,
    instances::{self, Instance, InstanceRaw},
    math::rect,
    math::Rect,
    minor_types::{DrawParams, TimeManager},
    minor_types::{Feature, Features, GoodManUI, Input, InstIndex, Layer, Manager, TexIndex},
    texture::{self, Texture},
};

mod engine_manager;

pub struct Engine {
    input: Input,
    window: Window,
    background_color: wgpu::Color,
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    win_size: winit::dpi::PhysicalSize<u32>,
    render_pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    instance_buffer: wgpu::Buffer,
    camera_buffer: wgpu::Buffer,

    instances: Vec<Instance>,
    instances_raw: Vec<InstanceRaw>,
    instances_rendered: usize,

    tex_index_hash_bind: HashMap<TexIndex, wgpu::BindGroup>,
    layer_hash_inst_vec: HashMap<Layer, Vec<InstIndex>>,
    inst_hash_tex_index: HashMap<InstIndex, TexIndex>,
    texture_amt_created: u32,

    camera: Camera,
    camera_bind_group: wgpu::BindGroup,
    window_bind_group: wgpu::BindGroup,

    time: TimeManager,

    platform: Platform,
    egui_rpass: egui_wgpu_backend::RenderPass,

    features: Features,

    game_ui: Option<GoodManUI>,
}

impl Engine {
    pub fn enter_loop<T>(mut self, mut manager: T, event_loop: EventLoop<()>)
    where
        T: Manager + 'static,
    {
        env_logger::init();
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
                    manager.update(self.time.average_delta_t, &self.input);

                    if self.input.is_right_mouse_button_pressed() {
                        println!("{}", 1. / self.time.average_delta_t);
                    }
                    self.input.reset_buttons();

                    match self.time.target_fps {
                        Some(fps) => {
                            if self.time.time_since_last_render >= 0.995 / fps as f64 {
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
                    load: wgpu::LoadOp::Clear(self.background_color),
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

        //let x = Instant::now();
        for layer in Layer::iterator().rev() {
            if let Some(inst_vec) = self.layer_hash_inst_vec.get_mut(layer) {
                for i in inst_vec.drain(..) {
                    //Check if into_iter() is better
                    if let Some(tex_index) = self.inst_hash_tex_index.get(&i) {
                        if let Some(bind) = self.tex_index_hash_bind.get(tex_index) {
                            render_pass.set_bind_group(0, bind, &[]);
                            render_pass.draw_indexed(0..INDICES.len() as u32, 0, i..(i + 1));
                        }
                    }
                }
            }
        }
        // let x = x.elapsed().as_micros(); //~400 micro
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

        self.instances_rendered = 0;
        self.time.time_since_last_render = 0.;
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
            ui.label(format!(
                "window size: {:?}x{:?}",
                self.win_size.width, self.win_size.height
            ));
            let fps = match self.time.target_fps {
                Some(fps_) => fps_,
                None => self.get_average_tps(),
            };
            ui.label(format!("FPS: {:?}", fps));
            ui.label(format!("TPS: {:?}", self.get_average_tps()));
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
        let inst = Instance::new(rect, rotation);

        if self.instances_rendered < self.instances.len() {
            if self.instances[self.instances_rendered] != inst {
                self.instances[self.instances_rendered] = inst;
                self.instances_raw[self.instances_rendered] = inst.to_raw();
            }
        } else {
            self.instances.push(inst);
            self.instances_raw.push(inst.to_raw());
        }

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
        if new_size.width > 0 && new_size.height > 0 {
            self.win_size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
        }
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
