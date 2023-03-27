use std::{time::Instant, collections::HashMap};

use winit::{event::WindowEvent, window::Window};

use crate::{
    camera::{self, Camera},
    instances::{self, SquareInstance},
    object_data::{self, INDICES},
    state_manager::{self, Input, Vec2},
    texture::{self, Texture},
};

pub struct State {
    pub size: winit::dpi::PhysicalSize<u32>,
    pub instances: Vec<SquareInstance>,
    pub input: Input,
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    render_pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    camera: Camera,
    camera_bind_group: wgpu::BindGroup,
    camera_buffer: wgpu::Buffer,
    instance_buffer: wgpu::Buffer,
    window: Window,
    last_frame: Instant,
    frames_passed: u32,
    total_frame_time: f64,
    pub time_since_last_render: f64,
    pub target_fps: u32,
    instances_drawn: usize,
    instance_indexes: HashMap<String, Vec<usize>>,
    texture_bind_groups: HashMap<String, wgpu::BindGroup>,
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

        let texture_bind_group_layout = texture::create_bind_group_layout(&device);

        let texture_bind_groups = HashMap::new();

        let camera = Camera::new(false);

        let camera_buffer = camera::create_buffer(&device, camera.uniform);
        let camera_bind_group_layout = camera::create_bind_group_layout(&device);
        let camera_bind_group =
            camera::create_bind_group(&device, &camera_buffer, &camera_bind_group_layout);

        let instances = vec![];

        let instance_data = instances
            .iter()
            .map(SquareInstance::to_raw)
            .collect::<Vec<_>>();
        let instance_buffer = instances::create_buffer(&device, &instance_data);

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
            camera,
            camera_bind_group,
            camera_buffer,
            instance_buffer,
            instances,
            input: Input::new(),
            last_frame: Instant::now(),
            total_frame_time: 0.,
            frames_passed: 0,
            time_since_last_render: 0.,
            target_fps: 144,
            instances_drawn: 0,
            instance_indexes: HashMap::new(),
            texture_bind_groups,
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

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
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
        
        //render_pass.set_bind_group(0, &self.texture_bind_groups[0], &[]);
        render_pass.set_bind_group(1, &self.camera_bind_group, &[]);

        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        
        render_pass.set_vertex_buffer(1, self.instance_buffer.slice(..));

        //let mut x = 0;
        for (label, bind_group) in &self.texture_bind_groups {
            
            if self.instance_indexes.contains_key(label) {
                //dbg!(bind_group);
                /*let t = &self.texture_bind_groups.get("ball.png");
                let t = match *t {
                    Some(f) => f,
                    _ => panic!("ds")
                };*/
                render_pass.set_bind_group(0, bind_group, &[]);
                //x +=1;
                
                for (_, inst_vec) in &mut self.instance_indexes {
                     
                    inst_vec.into_iter().for_each(|i| {
                        
                        let i = *i as u64;

                        //println!("yes");
                        //println!("{}", self.instance_buffer.size());
                        //let i = 3;
                        //render_pass.set_vertex_buffer(1, self.instance_buffer.slice((i * 192)..((i + 1)*192)));
                        //println!("{}", i);
                        
                        render_pass.draw_indexed(
                            0..INDICES.len() as u32,
                            0,
                            (i as u32)..(i + 1) as u32,
                        );
                        //println!("maybe")
                    });
                    inst_vec.clear();
                }
            }
        }
        //dbg!(x);



        /*render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        render_pass.draw_indexed(
            0..INDICES.len() as u32,
            0,
            0..self.square_instances.len() as u32,
        );*/

        //render_pass.set_bind_group(0, &self.texture_bind_groups[1], &[]);
        /*render_pass.set_vertex_buffer(1, self.circle_instance_buffer.slice(..));
        render_pass.draw_indexed(
            0..INDICES.len() as u32,
            0,
            0..self.circle_instances.len() as u32,
        );*/

        drop(render_pass);
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        self.instances_drawn = 0;

        Ok(())
    }

    pub fn draw_texture(&mut self, pos: Vec2, size: Vec2, texture: &Texture) {
        let inst = SquareInstance::new(pos, size);

        self.instances[self.instances_drawn] = inst;
        

        if self.instance_indexes.contains_key(&texture.label) {
            for (label, index_vec) in &mut self.instance_indexes {
                if *label == texture.label {
                    index_vec.push(self.instances_drawn);
                    //dbg!(texture.label.to_string());
                }
            }
        } else {
            //dbg!(texture.label.to_string());
            self.instance_indexes.insert(texture.label.to_string(), vec![self.instances_drawn]);
        }

        //self.instance_textures[self.square_instances_drawn] 
        self.instances_drawn += 1;
        self.update_square_instances();
    }

    pub fn update_square_instances(&mut self) {
        let square_instance_data = self
            .instances
            .iter()
            .map(SquareInstance::to_raw)
            .collect::<Vec<_>>();

        let data_size = square_instance_data.len() as u64 * 16;
        if self.instance_buffer.size() != data_size {
            self.instance_buffer =
                instances::create_buffer(&self.device, &square_instance_data);
        }

        self.queue.write_buffer(
            &self.instance_buffer,
            0,
            bytemuck::cast_slice(&square_instance_data),
        );
    }

    pub fn update_time(&mut self) {
        let time_since_last_frame = self.last_frame.elapsed().as_secs_f64();
        self.last_frame = std::time::Instant::now();

        self.total_frame_time += time_since_last_frame;
        self.time_since_last_render += time_since_last_frame;

        if self.total_frame_time > 3. && self.total_frame_time < 3.1 {
            self.get_average_fps();
        }

        self.frames_passed += 1;
    }

    pub fn get_frame_time(&self) -> f64 {
        self.last_frame.elapsed().as_secs_f64()
    }

    pub fn get_average_fps(&mut self) -> u32 {
        let fps = (self.frames_passed as f32 / self.total_frame_time as f32) as u32;
        self.frames_passed = 0;
        self.total_frame_time = 0.;
        fps
    }

    pub fn create_texture(&mut self, bytes: &[u8], label: &str) -> Texture {
        let tex = Texture::from_bytes(&self.device, &self.queue, bytes, label)
            .expect(&format!("Could not create {} texture", label));

        let texture_bind_group_layout = texture::create_bind_group_layout(&self.device);
        let texture_bind_group =
            texture::create_bind_group(&self.device, &texture_bind_group_layout, &tex);

        self.texture_bind_groups.insert(tex.label.clone(), texture_bind_group);
        tex
    }
}
