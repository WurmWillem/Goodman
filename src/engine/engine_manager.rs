use std::collections::HashMap;

use wgpu::util::DeviceExt;
use winit::dpi::PhysicalSize;
use winit::event_loop::EventLoop;
use winit::window::WindowBuilder;

use crate::camera::{self, Camera};
use crate::engine::Engine;
use crate::instances::InstanceRaw;
use crate::instances::{Instance, Vertex};
use crate::minor_types::{Time, Windowniform};
use crate::prelude::{Color, Input, Vec2};
use crate::texture::{self, Texture};

impl Engine {
    pub fn create_texture(&mut self, bytes: &[u8], label: &str) -> Result<Texture, &'static str> {
        let tex = match Texture::from_bytes(
            &self.device,
            &self.queue,
            bytes,
            label,
            self.texture_amt_created,
        ) {
            Ok(tex) => tex,
            Err(_) => return Err("failed to create texture"),
        };

        let texture_bind_group_layout = super::texture::create_bind_group_layout(&self.device);
        let texture_bind_group =
            texture::create_bind_group(&self.device, &texture_bind_group_layout, &tex);

        self.tex_index_hash_bind
            .insert(tex.index, texture_bind_group);

        self.texture_amt_created += 1;
        Ok(tex)
    }

    /*fn get_delta_time(&self) -> f64 {
        self.delta_time.elapsed().as_secs_f64()
    }*/
    pub fn get_average_tps(&self) -> u32 {
        (self.time.ticks_passed_this_sec as f64 / self.time.tick_time_this_sec) as u32
    }
    pub fn get_target_fps(&self) -> Option<u32> {
        self.time.target_fps
    }
    pub fn get_size(&self) -> winit::dpi::PhysicalSize<u32> {
        self.win_size
    }
    pub fn get_time_since_last_render(&self) -> f64 {
        self.time.time_since_last_render
    }

    pub fn set_target_fps(&mut self, fps: Option<u32>) {
        self.time.target_fps = fps;
    }
    pub fn set_target_tps(&mut self, tps: Option<u32>) {
        self.time.target_tps = tps;
    }
    pub fn set_background_color(&mut self, color: Color) {
        self.background_color = wgpu::Color {
            r: color.r,
            g: color.g,
            b: color.b,
            a: color.a,
        }
    }

    pub async fn new(size: Vec2, event_loop: &EventLoop<()>, win_resizable: bool) -> Self {
        let window = WindowBuilder::new()
            .with_resizable(win_resizable)
            .with_inner_size(PhysicalSize::new(size.x, size.y))
            .build(event_loop)
            .expect("Failed to build window");

        let win_size = window.inner_size();

        // Backends::all => Vulkan + Metal + DX12 + Browser WebGPU
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            dx12_shader_compiler: Default::default(), // sudo sysctl dev.i915.perf_stream_paranoid=0
        });

        // State owns the window so this should be safe.
        let surface = unsafe { instance.create_surface(&window) }.expect("Failed to init surface");

        let adapter = super::engine_manager::create_adapter(&instance, &surface).await;
        let (device, queue) = super::engine_manager::create_device_and_queue(&adapter).await;

        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = super::engine_manager::create_surface_format(&surface_caps);

        let config = super::engine_manager::create_config(&surface_format, win_size, &surface_caps);
        surface.configure(&device, &config);

        let texture_bind_group_layout = super::texture::create_bind_group_layout(&device);

        let camera = Camera::new(false);
        let camera_buffer = camera::create_buffer(&device, camera.uniform);
        let camera_bind_group_layout = camera::create_bind_group_layout(&device);
        let camera_bind_group =
            camera::create_bind_group(&device, &camera_buffer, &camera_bind_group_layout);

        let window_size_uniform = Windowniform {
            size: [1. / size.x as f32, 1. / size.y as f32],
        };
        let window_size_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("window size buffer"),
            contents: bytemuck::cast_slice(&[window_size_uniform]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });
        let window_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
                label: Some("camera_bind_group_layout"),
            });
        let window_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &window_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: window_size_buffer.as_entire_binding(),
            }],
            label: Some("camera_bind_group"),
        });

        let instances = vec![];
        let instances_raw = instances.iter().map(Instance::to_raw).collect::<Vec<_>>();
        let instance_buffer = super::instances::create_buffer(&device, &instances_raw);
        let shader = device.create_shader_module(wgpu::include_wgsl!("shader.wgsl"));

        let render_pipeline_layout = super::engine_manager::create_render_pipeline_layout(
            &device,
            &texture_bind_group_layout,
            &camera_bind_group_layout,
            &window_bind_group_layout,
        );
        let render_pipeline = super::engine_manager::create_render_pipeline(
            &device,
            &render_pipeline_layout,
            &shader,
            &config,
        );

        let (vertex_buffer, index_buffer) = super::instances::create_buffers(&device);

        let background_color = wgpu::Color {
            r: 0.,
            g: 0.,
            b: 0.,
            a: 1.,
        };

        Self {
            input: Input::new(),
            window,
            window_bind_group,

            background_color,
            surface,
            device,
            queue,
            config,
            win_size,

            render_pipeline,
            vertex_buffer,
            index_buffer,

            camera,
            camera_bind_group,
            camera_buffer,

            instance_buffer,
            instances,
            instances_raw,
            instances_rendered: 0,

            time: Time::new(),

            texture_amt_created: 0,
            layer_hash_inst_vec: HashMap::new(),
            tex_index_hash_bind: HashMap::new(),
            inst_hash_tex_index: HashMap::new(),
        }
    }
}

pub async fn create_adapter(instance: &wgpu::Instance, surface: &wgpu::Surface) -> wgpu::Adapter {
    instance
        .request_adapter(&wgpu::RequestAdapterOptionsBase {
            power_preference: wgpu::PowerPreference::default(),
            force_fallback_adapter: false,
            compatible_surface: Some(surface),
        })
        .await
        .expect("Failed to create adapter")
}

pub async fn create_device_and_queue(adapter: &wgpu::Adapter) -> (wgpu::Device, wgpu::Queue) {
    adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                features: wgpu::Features::empty(),
                limits: wgpu::Limits::default(),
                label: None,
            },
            None,
        )
        .await
        .expect("failed to create device or queue")
}

pub fn create_surface_format(surface_caps: &wgpu::SurfaceCapabilities) -> wgpu::TextureFormat {
    surface_caps
        .formats
        .iter()
        .copied()
        .find(|f| f.describe().srgb)
        .unwrap_or(surface_caps.formats[0])
}

pub fn create_config(
    surface_format: &wgpu::TextureFormat,
    size: PhysicalSize<u32>,
    surface_caps: &wgpu::SurfaceCapabilities,
) -> wgpu::SurfaceConfiguration {
    wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format: *surface_format,
        width: size.width,
        height: size.height,
        present_mode: surface_caps.present_modes[0],
        alpha_mode: surface_caps.alpha_modes[0],
        view_formats: vec![],
    }
}

pub fn create_render_pipeline_layout(
    device: &wgpu::Device,
    texture_bind_group_layout: &wgpu::BindGroupLayout,
    camera_bind_group_layout: &wgpu::BindGroupLayout,
    window_bind_group_layout: &wgpu::BindGroupLayout,
) -> wgpu::PipelineLayout {
    device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("Render Pipeline Layout"),
        bind_group_layouts: &[
            texture_bind_group_layout,
            camera_bind_group_layout,
            window_bind_group_layout,
        ],
        push_constant_ranges: &[],
    })
}

pub fn create_render_pipeline(
    device: &wgpu::Device,
    render_pipeline_layout: &wgpu::PipelineLayout,
    shader: &wgpu::ShaderModule,
    config: &wgpu::SurfaceConfiguration,
) -> wgpu::RenderPipeline {
    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("Render Pipeline"),
        layout: Some(render_pipeline_layout),
        vertex: wgpu::VertexState {
            module: shader,
            entry_point: "vs_main",
            buffers: &[Vertex::desc(), InstanceRaw::desc()],
        },
        fragment: Some(wgpu::FragmentState {
            module: shader,
            entry_point: "fs_main",
            targets: &[Some(wgpu::ColorTargetState {
                format: config.format,
                blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                write_mask: wgpu::ColorWrites::ALL,
            })],
        }),
        primitive: wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleList,
            strip_index_format: None,
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: Some(wgpu::Face::Back),
            // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
            polygon_mode: wgpu::PolygonMode::Fill,
            // Requires Features::DEPTH_CLIP_CONTROL
            unclipped_depth: false,
            // Requires Features::CONSERVATIVE_RASTERIZATION
            conservative: false,
        },
        depth_stencil: None,
        multisample: wgpu::MultisampleState {
            count: 1,
            mask: !0,
            alpha_to_coverage_enabled: false,
        },
        multiview: None,
    })
}
