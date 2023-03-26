use std::time::Instant;

use winit::{event::WindowEvent, window::Window};

use crate::{
    camera::{self, Camera},
    instances::{self, CircleInstance, SquareInstance},
    object_data::{self, INDICES},
    state_manager::{self, Input},
    texture::{self},
};

pub struct State {
    pub size: winit::dpi::PhysicalSize<u32>,
    pub square_instances: Vec<SquareInstance>,
    pub circle_instances: Vec<CircleInstance>,
    pub input: Input,
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    render_pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    diffuse_bind_group_0: wgpu::BindGroup,
    diffuse_bind_group_1: wgpu::BindGroup,
    camera: Camera,
    camera_bind_group: wgpu::BindGroup,
    camera_buffer: wgpu::Buffer,
    square_instance_buffer: wgpu::Buffer,
    circle_instance_buffer: wgpu::Buffer,
    window: Window,
    last_frame: Instant,
    frames_passed: u32,
    total_frame_time: f32,
}

impl State {
    pub async fn new(window: Window) -> Self {
        let size = window.inner_size();

        // Backends::all => Vulkan + Metal + DX12 + Browser WebGPU
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            dx12_shader_compiler: Default::default(),
        });

        // State owns the window so this should be safe.
        let surface = unsafe { instance.create_surface(&window) }.expect("Failed to init surface");

        let adapter = state_manager::create_adapter(&instance, &surface).await;
        let (device, queue) = state_manager::create_device_and_queue(&adapter).await;

        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = state_manager::create_surface_format(&surface_caps);

        let config = state_manager::create_config(&surface_format, size, &surface_caps);
        surface.configure(&device, &config);

        let diffuse_bytes = include_bytes!("assets/paddle.png");
        let diffuse_texture_0 =
            texture::Texture::from_bytes(&device, &queue, diffuse_bytes, "assets/paddle.png")
                .unwrap();

        let diffuse_bytes = include_bytes!("assets/ball.png");
        let diffuse_texture_1 =
            texture::Texture::from_bytes(&device, &queue, diffuse_bytes, "assets/ball.png")
                .unwrap();

        let texture_bind_group_layout = texture::create_bind_group_layout(&device);

        let diffuse_bind_group_0 =
            texture::create_bind_group(&device, &texture_bind_group_layout, &diffuse_texture_0);
        let diffuse_bind_group_1 =
            texture::create_bind_group(&device, &texture_bind_group_layout, &diffuse_texture_1);

        let camera = Camera::new(false);

        let camera_buffer = camera::create_buffer(&device, camera.uniform);
        let camera_bind_group_layout = camera::create_bind_group_layout(&device);
        let camera_bind_group =
            camera::create_bind_group(&device, &camera_buffer, &camera_bind_group_layout);

        let square_instances = vec![];
        let circle_instances = vec![];

        let square_instance_data = square_instances
            .iter()
            .map(SquareInstance::to_raw)
            .collect::<Vec<_>>();
        let square_instance_buffer = instances::create_buffer(&device, &square_instance_data);

        let circle_instance_data = circle_instances
            .iter()
            .map(CircleInstance::to_raw)
            .collect::<Vec<_>>();
        let circle_instance_buffer = instances::create_buffer(&device, &circle_instance_data);

        let shader = device.create_shader_module(wgpu::include_wgsl!("shader.wgsl"));

        let render_pipeline_layout = state_manager::create_render_pipeline_layout(
            &device,
            &texture_bind_group_layout,
            &camera_bind_group_layout,
        );

        let render_pipeline = state_manager::create_render_pipeline(
            &device,
            &render_pipeline_layout,
            &shader,
            &config,
        );

        let (vertex_buffer, index_buffer) = object_data::create_buffers(&device);

        Self {
            window,
            surface,
            device,
            queue,
            config,
            size,
            render_pipeline,
            vertex_buffer,
            index_buffer,
            diffuse_bind_group_0,
            diffuse_bind_group_1,
            camera,
            camera_bind_group,
            camera_buffer,
            square_instance_buffer,
            circle_instance_buffer,
            square_instances,
            circle_instances,
            input: Input::new(),
            last_frame: Instant::now(),
            total_frame_time: 0.,
            frames_passed: 0,
        }
    }

    pub fn window(&self) -> &Window {
        &self.window
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }

    pub fn input(&mut self, event: &WindowEvent) -> bool {
        self.input.process_events(event)
    }

    pub fn update(&mut self) {
        if self.camera.movement_enabled {
            self.camera.update(&self.input);
            self.queue.write_buffer(
                &self.camera_buffer,
                0,
                bytemuck::cast_slice(&[self.camera.uniform]),
            );
        }
    }

    pub fn render(&self) -> Result<(), wgpu::SurfaceError> {
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
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: 0.1,
                        g: 0.2,
                        b: 0.3,
                        a: 1.0,
                    }),
                    store: true,
                },
            })],
            depth_stencil_attachment: None,
        });

        render_pass.set_pipeline(&self.render_pipeline);

        render_pass.set_bind_group(0, &self.diffuse_bind_group_0, &[]);
        render_pass.set_bind_group(1, &self.camera_bind_group, &[]);

        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_vertex_buffer(1, self.square_instance_buffer.slice(..));
        render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        render_pass.draw_indexed(
            0..INDICES.len() as u32,
            0,
            0..self.square_instances.len() as u32,
        );

        render_pass.set_bind_group(0, &self.diffuse_bind_group_1, &[]);
        render_pass.set_vertex_buffer(1, self.circle_instance_buffer.slice(..));
        render_pass.draw_indexed(
            0..INDICES.len() as u32,
            0,
            0..self.circle_instances.len() as u32,
        );

        drop(render_pass);
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }

    pub fn update_square_instances(&mut self) {
        let square_instance_data = self
            .square_instances
            .iter()
            .map(SquareInstance::to_raw)
            .collect::<Vec<_>>();

        let data_size = square_instance_data.len() as u64 * 16;
        if self.square_instance_buffer.size() != data_size {
            self.square_instance_buffer =
                instances::create_buffer(&self.device, &square_instance_data);
        }

        self.queue.write_buffer(
            &self.square_instance_buffer,
            0,
            bytemuck::cast_slice(&square_instance_data),
        );
    }
    pub fn update_circle_instances(&mut self) {
        let circle_instance_data = self
            .circle_instances
            .iter()
            .map(CircleInstance::to_raw)
            .collect::<Vec<_>>();

        let data_size = circle_instance_data.len() as u64 * 16;
        if self.circle_instance_buffer.size() != data_size {
            self.circle_instance_buffer =
                instances::create_buffer(&self.device, &circle_instance_data);
        }

        self.queue.write_buffer(
            &self.circle_instance_buffer,
            0,
            bytemuck::cast_slice(&circle_instance_data),
        );
    }

    pub fn update_time(&mut self) {
        self.total_frame_time += self.last_frame.elapsed().as_secs_f32();
        self.last_frame = std::time::Instant::now();
        self.frames_passed += 1;
    }

    pub fn get_frame_time(&self) -> f32 {
        self.last_frame.elapsed().as_secs_f32()
    }

    pub fn get_fps(&mut self) -> f32 {
        let fps = self.frames_passed as f32 / self.total_frame_time;
        self.frames_passed = 0;
        self.total_frame_time = 0.;
        fps
    }
}
