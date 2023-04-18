use std::{collections::HashMap, time::Instant};

use engine_manager::Color;
use winit::{
    dpi::PhysicalSize,
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

use crate::{
    camera::{self, Camera},
    engine_manager::{self, Input, Manager, Vec2},
    instances::{self, Instance, InstanceRaw},
    math::Rect,
    object_data::{self, INDICES},
    texture::{self, Texture},
};

pub struct Engine {
    pub input: Input,
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
    bind_group_indexes: HashMap<String, Vec<usize>>,
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
    pub async fn new(size: Vec2, event_loop: &EventLoop<()>) -> Self {
        let window = WindowBuilder::new() //350 - 1;
            .with_inner_size(PhysicalSize::new(size.x, size.y))
            .build(event_loop)
            .expect("Failed to build window");

        let size = window.inner_size();

        // Backends::all => Vulkan + Metal + DX12 + Browser WebGPU
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            dx12_shader_compiler: Default::default(), // sudo sysctl dev.i915.perf_stream_paranoid=0
        });

        // State owns the window so this should be safe.
        let surface = unsafe { instance.create_surface(&window) }.expect("Failed to init surface");

        let adapter = engine_manager::create_adapter(&instance, &surface).await;
        let (device, queue) = engine_manager::create_device_and_queue(&adapter).await;

        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = engine_manager::create_surface_format(&surface_caps);

        let config = engine_manager::create_config(&surface_format, size, &surface_caps);
        surface.configure(&device, &config);

        let texture_bind_group_layout = texture::create_bind_group_layout(&device);
        let texture_bind_groups = HashMap::new();

        let camera = Camera::new(false);
        let camera_buffer = camera::create_buffer(&device, camera.uniform);
        let camera_bind_group_layout = camera::create_bind_group_layout(&device);
        let camera_bind_group =
            camera::create_bind_group(&device, &camera_buffer, &camera_bind_group_layout);

        let instances = vec![];
        let instance_data = instances.iter().map(Instance::to_raw).collect::<Vec<_>>();
        let instance_buffer = instances::create_buffer(&device, &instance_data);

        let shader = device.create_shader_module(wgpu::include_wgsl!("shader.wgsl"));

        let render_pipeline_layout = engine_manager::create_render_pipeline_layout(
            &device,
            &texture_bind_group_layout,
            &camera_bind_group_layout,
        );
        let render_pipeline = engine_manager::create_render_pipeline(
            &device,
            &render_pipeline_layout,
            &shader,
            &config,
        );

        let (vertex_buffer, index_buffer) = object_data::create_buffers(&device);

        let background_color = wgpu::Color {
            r: 0.,
            g: 0.,
            b: 0.,
            a: 1.,
        };

        Self {
            window,
            background_color,
            surface,
            device,
            queue,
            config,
            size,
            render_pipeline,
            vertex_buffer,
            index_buffer,
            camera,
            camera_bind_group,
            camera_buffer,
            instance_buffer,
            instances,
            instances_raw: instance_data,
            input: Input::new(),
            last_frame: Instant::now(),
            frame_time_this_sec: 0.,
            frames_passed_this_sec: 0,
            time_since_last_render: 0.,
            target_fps: None,
            //target_tps: 5700,
            instances_drawn: 0,
            bind_group_indexes: HashMap::new(),
            texture_bind_groups,
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

        for (bind_group_label, bind_group) in &self.texture_bind_groups {
            if let Some(inst_vec) = self.bind_group_indexes.get_mut(bind_group_label) {
                render_pass.set_bind_group(0, bind_group, &[]);
                for i in inst_vec.drain(..) {
                    render_pass.draw_indexed(
                        0..INDICES.len() as u32,
                        0,
                        (i as u32)..(i + 1) as u32,
                    );
                }
            }
        }

        drop(render_pass);
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        self.instances_drawn = 0;
        self.time_since_last_render = 0.;
        Ok(())
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

    pub fn draw_texture(&mut self, mut rect: Rect, texture: &Texture) {
        rect.x = rect.x / (self.window.inner_size().width as f64 * 0.5) - 1.;
        rect.y = rect.y / (self.window.inner_size().height as f64 * 0.5) - 1.;
        rect.w /= self.window.inner_size().width as f64;
        rect.h /= self.window.inner_size().height as f64;

        let inst = Instance::new(rect);
        if self.instances[self.instances_drawn] != inst {
            self.instances[self.instances_drawn] = inst;
            self.instances_raw[self.instances_drawn] = inst.to_raw();
        }

        if self.bind_group_indexes.contains_key(&texture.label) {
            for (label, index_vec) in &mut self.bind_group_indexes {
                if *label == texture.label {
                    index_vec.push(self.instances_drawn);
                    break;
                }
            }
        } else {
            self.bind_group_indexes
                .insert(texture.label.to_string(), vec![self.instances_drawn]);
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

    pub fn update_instance_buffer(&mut self) {
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

    pub fn enter_loop<T>(mut self, mut manager: T, event_loop: EventLoop<()>)
    where
        T: Manager + 'static,
    {
        env_logger::init();
        event_loop.run(move |event, _, control_flow| {
            match event {
                Event::WindowEvent {
                    ref event,
                    window_id,
                } if window_id == self.window.id() => {
                    if !self.input(event) {
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

                Event::MainEventsCleared => {
                    self.update();
                    manager.update(&self);

                    if self.input.left_mouse_button_pressed {
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
                    self.update_textures_and_render(&mut manager, control_flow);
                }
                _ => {}
            }
        });
    }

    fn update_textures_and_render<T>(&mut self, manager: &mut T, control_flow: &mut ControlFlow)
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

    pub fn create_texture(&mut self, bytes: &[u8], label: &str) -> Texture {
        let tex = Texture::from_bytes(&self.device, &self.queue, bytes, label)
            .unwrap_or_else(|_| panic!("Could not create {label} texture"));

        let texture_bind_group_layout = texture::create_bind_group_layout(&self.device);
        let texture_bind_group =
            texture::create_bind_group(&self.device, &texture_bind_group_layout, &tex);

        self.texture_bind_groups
            .insert(tex.label.clone(), texture_bind_group);
        tex
    }

    pub fn get_frame_time(&self) -> f64 {
        self.last_frame.elapsed().as_secs_f64()
    }
    pub fn get_average_tps(&mut self) -> u32 {
        (self.frames_passed_this_sec as f64 / self.frame_time_this_sec) as u32
    }
    pub fn get_target_fps(&self) -> Option<u32> {
        self.target_fps
    }
    pub fn get_size(&self) -> winit::dpi::PhysicalSize<u32> {
        self.size
    }
    pub fn get_time_since_last_render(&self) -> f64 {
        self.time_since_last_render
    }

    pub fn set_fps(&mut self, fps: Option<u32>) {
        self.target_fps = fps;
    }
    pub fn set_background_color(&mut self, color: Color) {
        self.background_color = wgpu::Color {
            r: color.r,
            g: color.g,
            b: color.b,
            a: color.a,
        }
    }
}
