use cgmath::vec2;
use winit::{event::WindowEvent, window::Window};

use crate::{
    camera::{self, Camera},
    instances::{self, CircleInstance, InstanceRaw, SquareInstance},
    object_data::{self, INDICES, VERTEX_SCALE},
    state_manager::{self, Input},
    texture::{self},
};

pub struct State {
    pub size: winit::dpi::PhysicalSize<u32>,
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    render_pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    diffuse_bind_group_0: wgpu::BindGroup,
    diffuse_bind_group_1: wgpu::BindGroup,
    //diffuse_texture: Texture,
    camera: Camera,
    camera_bind_group: wgpu::BindGroup,
    camera_buffer: wgpu::Buffer,
    pub square_instances: Vec<SquareInstance>,
    pub circle_instances: Vec<CircleInstance>,
    square_instance_buffer: wgpu::Buffer,
    circle_instance_buffer: wgpu::Buffer,
    pub input: Input,
    window: Window,
    ball_vel: cgmath::Vector2<f64>,
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

        let diffuse_bytes = include_bytes!("paddle.png");
        let diffuse_texture_0 =
            texture::Texture::from_bytes(&device, &queue, diffuse_bytes, "paddle.png").unwrap();

        let diffuse_bytes = include_bytes!("ball.png");
        let diffuse_texture_1 =
            texture::Texture::from_bytes(&device, &queue, diffuse_bytes, "ball.png").unwrap();

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

        //let instances = create_instances();
        /*let pos_0 = vec2(-0.8, 0.);
        let pos_1 = vec2(0.8, 0.);
        let scale_0 = vec2(1., 3.);

        let square_instance_0 = SquareInstance::new(pos_0, scale_0);
        let square_instance_1 = SquareInstance::new(pos_1, scale_0);*/

        let square_instances = vec![]; //square_instance_0, square_instance_1

        //let pos = vec2(0., 0.);
        //let circle_instance = CircleInstance::new(pos, 1.);

        let circle_instances = vec![]; //vec![circle_instance]

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
            //diffuse_texture: diffuse_texture_1,
            camera,
            camera_bind_group,
            camera_buffer,
            square_instance_buffer,
            circle_instance_buffer,
            square_instances,
            circle_instances,
            input: Input::new(),
            ball_vel: vec2(0.003, 0.0017),
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
        /*if self.camera.movement_enabled {
            self.camera.update(&self.input);
            self.queue.write_buffer(
                &self.camera_buffer,
                0,
                bytemuck::cast_slice(&[self.camera.uniform]),
            );
        }

        let mut paddle_0 = &mut self.square_instances[0];
        let speed = 0.005;
        let size_scaled_y = paddle_0.size.y * VERTEX_SCALE as f64 * 0.5 + speed + 0.07;

        if self.input.is_w_pressed && paddle_0.pos.y + size_scaled_y < 1. {
            paddle_0.pos.y += speed;
        }
        if self.input.is_s_pressed && paddle_0.pos.y - size_scaled_y > -1. {
            paddle_0.pos.y -= speed;
        }

        let mut paddle_1 = &mut self.square_instances[1];
        if self.input.is_up_pressed && paddle_1.pos.y + size_scaled_y < 1. {
            paddle_1.pos.y += speed;
        }
        if self.input.is_down_pressed && paddle_1.pos.y - size_scaled_y > -1. {
            paddle_1.pos.y -= speed;
        }

        let ball = &self.circle_instances[0];
        let radius_scaled = ball.radius * (VERTEX_SCALE as f64);

        let new_pos = vec2(ball.pos.x + self.ball_vel.x, ball.pos.y + self.ball_vel.y);
        if new_pos.x + radius_scaled > 1. || new_pos.x - radius_scaled < -1. {
            let pos_0 = vec2(-0.8, 0.);
            let pos_1 = vec2(0.8, 0.);
            let scale_0 = vec2(1., 3.);

            let square_instance_0 = SquareInstance::new(pos_0, scale_0);
            let square_instance_1 = SquareInstance::new(pos_1, scale_0);

            self.square_instances = vec![square_instance_0, square_instance_1];

            let pos = vec2(0., 0.);
            let circle_instance = CircleInstance::new(pos, 1.);

            self.circle_instances = vec![circle_instance];
        }

        let mut ball = &mut self.circle_instances[0];
        if new_pos.y + radius_scaled > 1. {
            self.ball_vel.y *= -1.;
            ball.pos.y = 1. - radius_scaled;
        }
        if new_pos.y - radius_scaled < -1. {
            self.ball_vel.y *= -1.;
            ball.pos.y = -1. + radius_scaled;
        }
        let paddle_0 = &self.square_instances[0];
        let paddle_1 = &self.square_instances[1];
        let size_scaled_x = paddle_0.size.x * VERTEX_SCALE as f64 * 0.5 + 0.02;
        let size_scaled_y = paddle_0.size.y * VERTEX_SCALE as f64 * 0.5 + 0.02;

        if (new_pos.x + radius_scaled > paddle_1.pos.x - size_scaled_x
            && new_pos.y + radius_scaled > paddle_1.pos.y - size_scaled_y
            && new_pos.y - radius_scaled < paddle_1.pos.y + size_scaled_y
            && self.ball_vel.x > 0.)
            || (new_pos.x - radius_scaled < paddle_0.pos.x + size_scaled_x
                && new_pos.y + radius_scaled > paddle_0.pos.y - size_scaled_y
                && new_pos.y - radius_scaled < paddle_0.pos.y + size_scaled_y
                && self.ball_vel.x < 0.)
        {
            self.ball_vel.x *= -1.;
        }

        ball.pos.x += self.ball_vel.x;
        ball.pos.y += self.ball_vel.y;

        let square_instance_data = self
            .square_instances
            .iter()
            .map(SquareInstance::to_raw)
            .collect::<Vec<_>>();
        self.queue.write_buffer(
            &self.square_instance_buffer,
            0,
            bytemuck::cast_slice(&square_instance_data),
        );

        let circle_instance_data = self
            .circle_instances
            .iter()
            .map(CircleInstance::to_raw)
            .collect::<Vec<_>>();

        self.queue.write_buffer(
            &self.circle_instance_buffer,
            0,
            bytemuck::cast_slice(&circle_instance_data),
        );*/
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
}
