use std::{collections::HashMap, time::Instant};
use winit::{
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::Window,
};

use crate::{
    camera::Camera,
    instances::{self, Instance, InstanceRaw},
    math::Rect,
    minor_types::{Input, Layer, Manager},
    object_data::{self, INDICES},
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
    size: winit::dpi::PhysicalSize<u32>,

    render_pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    instance_buffer: wgpu::Buffer,
    camera_buffer: wgpu::Buffer,

    instances: Vec<Instance>,
    instances_raw: Vec<InstanceRaw>,
    instances_drawn: usize,

    tex_hash_inst: HashMap<u32, Vec<u32>>,
    tex_index_hash_bind: HashMap<u32, wgpu::BindGroup>,
    inst_hash_layer: HashMap<Layer, Vec<u32>>,
    inst_hash_tex_index: HashMap<u32, u32>,
    texture_amt_created: u32,

    camera: Camera,
    camera_bind_group: wgpu::BindGroup,
    window_bind_group: wgpu::BindGroup,

    frame_time: Instant,
    target_fps: Option<u32>,
    target_tps: Option<u32>,
    frames_passed_this_sec: u64,
    frame_time_this_sec: f64,
    time_since_last_render: f64,
}

impl Engine {
    pub fn enter_loop<T>(mut self, mut manager: T, event_loop: EventLoop<()>)
    where
        T: Manager + 'static,
    {
        env_logger::init();
        event_loop.run(move |event, _, control_flow| match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == self.window.id() => {
                if !self.input(event) {
                    self.handle_window_event(event, control_flow);
                }
            }
            Event::MainEventsCleared => {
                /*if let Some(tps) = self.target_tps {
                    if self.get_frame_time() < 1. / tps as f64 {
                        return;
                    }
                }*/
                self.update();
                manager.update(self.get_frame_time(), &self.input);

                if self.input.is_left_mouse_button_pressed() {
                    println!("{}", self.get_average_tps());
                }
                self.input.reset_buttons();

                self.update_time();

                match self.get_target_fps() {
                    Some(fps) => {
                        if self.get_time_since_last_render() >= 1. / fps as f64 {
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
        for layer in Layer::iterator() {
            if let Some(inst_vec) = self.inst_hash_layer.get_mut(&layer) {
                for i in inst_vec.drain(..) {
                    if let Some(tex_index) = self.inst_hash_tex_index.get_mut(&i) {
                        if let Some(bind) = self.tex_index_hash_bind.get(&tex_index) {
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

        /*let x = Instant::now();
        for (tex_index, tex_bind_group) in &self.tex_index_hash_bind {
            if let Some(inst_vec) = self.tex_hash_inst.get_mut(tex_index) {
                render_pass.set_bind_group(0, tex_bind_group, &[]);
                for i in inst_vec.drain(..) {
                    render_pass.draw_indexed(0..INDICES.len() as u32, 0, i..(i + 1));
                }
            }
        }
        let x = x.elapsed().as_micros(); //50-150 micros
        println!("{x}");*/
        /*
        foreach tex
            if an instance uses tex
                foreach instance that uses tex
                    draw(inst)
        */

        drop(render_pass);
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        self.instances_drawn = 0;
        self.time_since_last_render = 0.;
        Ok(())
    }

    pub fn draw_texture(&mut self, rect: &Rect, texture: &Texture, layer: Layer) {
        let inst = Instance::new(*rect);
        if self.instances_drawn < self.instances_raw.len() {
            if self.instances[self.instances_drawn] != inst {
                self.instances[self.instances_drawn] = inst;
                self.instances_raw[self.instances_drawn] = inst.to_raw();
            }
        } else {
            self.instances.push(inst);
            self.instances_raw.push(inst.to_raw());
        }

        self.inst_hash_tex_index
            .insert(self.instances_drawn as u32, texture.index);

        match self.inst_hash_layer.get_mut(&layer) {
            Some(instance_vec) => instance_vec.push(self.instances_drawn as u32),
            None => {
                self.inst_hash_layer
                    .insert(layer, vec![self.instances_drawn as u32]);
            }
        }

        /*match self.tex_hash_inst.get_mut(&texture.index) {
            Some(instance_vec) => instance_vec.push(self.instances_drawn as u32),
            None => {
                self.tex_hash_inst
                    .insert(texture.index, vec![self.instances_drawn as u32]);
            }
        }*/
        self.instances_drawn += 1;
    }

    fn update_time(&mut self) {
        let time_since_last_frame = self.frame_time.elapsed().as_secs_f64();
        self.frame_time = Instant::now();

        self.frame_time_this_sec += time_since_last_frame;
        self.time_since_last_render += time_since_last_frame;
        self.frames_passed_this_sec += 1;

        if self.frame_time_this_sec > 1. {
            self.frames_passed_this_sec = 0;
            self.frame_time_this_sec = 0.;
        }
    }

    fn update_instance_buffer(&mut self) {
        if self.instance_buffer.size() == self.instances_raw.len() as u64 * 64 {
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
        if self.camera.movement_enabled {
            if self.camera.update(&self.input) {
                self.queue.write_buffer(
                    &self.camera_buffer,
                    0,
                    bytemuck::cast_slice(&[self.camera.uniform]),
                );
            }
        }
    }

    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
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
