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
    minor_types::{Input, Manager},
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
    tex_bind_group_indexes: HashMap<String, Vec<usize>>,
    texture_bind_groups: HashMap<String, wgpu::BindGroup>,
    camera: Camera,
    camera_bind_group: wgpu::BindGroup,
    last_frame: Instant,
    target_fps: Option<u32>,
    //pub target_tps: u32,
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
                self.update();
                manager.update(self.get_frame_time(), &self.input);

                if self.input.is_left_mouse_button_pressed() {
                    println!("{}", self.get_average_tps());
                }
                self.input.reset_buttons();

                self.update_time();
                match self.get_target_fps() {
                    Some(fps) => {
                        if self.get_time_since_last_render() > 1. / fps as f64 {
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

        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_vertex_buffer(1, self.instance_buffer.slice(..));
        render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);

        /*let x = Instant::now();
        let b = self.texture_bind_groups.get("block.png").unwrap();
        render_pass.set_bind_group(0, b, &[]);


                    render_pass.draw_indexed(
                        0..INDICES.len() as u32,
                        0,
                        0..self.instances.len() as u32,
                    );
                    let x = x.elapsed().as_secs_f64(); //3000000 7 digits   37037037 7 digits
                    println!("{}", (1. / x).round()); //7 digits in total
                    */

        //let x = Instant::now();
        for (bind_group_label, tex_bind_group) in &self.texture_bind_groups {
            if let Some(inst_vec) = self.tex_bind_group_indexes.get_mut(bind_group_label) {
                render_pass.set_bind_group(0, tex_bind_group, &[]);
                for i in inst_vec.drain(..) {
                    // let x = Instant::now();
                    render_pass.draw_indexed(
                        0..INDICES.len() as u32,
                        0,
                        (i as u32)..(i + 1) as u32,
                    );
                    //let x = x.elapsed().as_secs_f64(); //37037037
                    //println!("{}", (1. / x).round());
                }
                //println!();
                //println!();
                //println!();
            }
        }
        //let x = x.elapsed().as_secs_f64();
        //println!("{}", (1. / x).round());

        /*
        foreach tex
            if a instance uses tex
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

    pub fn draw_texture(&mut self, mut rect: Rect, texture: &Texture) {
        // rect.x = rect.x / (self.window.inner_size().width as f64 * 0.5) - 1.; // Vertex buffer * 1 / (self.window.inner_size().width as f64 * 0.5)
        // rect.y = rect.y / (self.window.inner_size().height as f64 * 0.5) - 1.;
        // rect.w /= self.window.inner_size().width as f64;
        // rect.h /= self.window.inner_size().height as f64;

        let inst = Instance::new(rect);
        if self.instances[self.instances_drawn] != inst {
            self.instances[self.instances_drawn] = inst;
            self.instances_raw[self.instances_drawn] = inst.to_raw();
        }

        match self.tex_bind_group_indexes.get_mut(&texture.label) {
            Some(index_vec) => index_vec.push(self.instances_drawn),
            None => {
                self.tex_bind_group_indexes
                    .insert(texture.label.to_string(), vec![self.instances_drawn]);
            }
        }
        self.instances_drawn += 1;
    }

    pub fn initialize_instances(&mut self, rects: Vec<Rect>) {
        self.instances = rects
            .iter()
            .map(|rect| Instance::new(*rect / 350. - 1.))
            .collect();
        self.instances_raw = self
            .instances
            .iter()
            .map(Instance::to_raw)
            .collect::<Vec<_>>();

        self.instance_buffer = instances::create_buffer(&self.device, &self.instances_raw);
    }

    fn update_time(&mut self) {
        let time_since_last_frame = self.last_frame.elapsed().as_secs_f64();
        self.last_frame = Instant::now();

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
            self.camera.update(&self.input);
            self.queue.write_buffer(
                &self.camera_buffer,
                0,
                bytemuck::cast_slice(&[self.camera.uniform]),
            );
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
