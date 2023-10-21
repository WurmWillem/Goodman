use egui_winit_platform::{Platform, PlatformDescriptor};
use wgpu::util::DeviceExt;
use winit::dpi::PhysicalSize;
use winit::event_loop::EventLoop;
use winit::window::WindowBuilder;

use crate::camera::{self, Camera};
use crate::engine::Engine;
use crate::minor_types::WindowUniform;
use crate::prelude::{Sound, Vec32};
use crate::texture::{self};
use crate::time::TimeManager;
use crate::ui::Ui;
use crate::vert_buffers::{Instance, TexCoords, Vertex};

pub struct EngineBuilder {
    win_size: Vec32,
    win_background_color: wgpu::Color,
    win_resizable: bool,
    win_title: String,

    show_engine_ui: bool,
    use_sound: bool,

    reset_rate: Option<f64>,
    target_fps: Option<u32>,
    target_tps: Option<u32>,
}
impl EngineBuilder {
    pub fn new(win_size: Vec32) -> Self {
        Self {
            win_size,
            win_background_color: wgpu::Color::BLACK,
            win_resizable: false,
            win_title: "Goodman".to_string(),

            show_engine_ui: false,
            use_sound: true,

            reset_rate: None,
            target_fps: None,
            target_tps: None,
        }
    }
    pub fn disable_sound(mut self) -> Self {
        self.use_sound = false;
        self
    }
    pub fn set_window_to_be_resizable(mut self) -> Self {
        self.win_resizable = true;
        self
    }
    pub fn show_engine_ui(mut self) -> Self {
        self.show_engine_ui = true;
        self
    }
    pub fn enable_average_tps_and_set_reset_rate(mut self, reset_rate: Option<f64>) -> Self {
        self.reset_rate = reset_rate;
        self
    }
    pub fn with_target_fps(mut self, target_fps: u32) -> Self {
        self.target_fps = Some(target_fps);
        self
    }
    pub fn with_target_tps(mut self, target_tps: u32) -> Self {
        self.target_tps = Some(target_tps);
        self
    }
    pub fn with_window_title(mut self, win_title: String) -> Self {
        self.win_title = win_title;
        self
    }
    pub fn with_background_color(mut self, color: crate::minor_types::Color) -> Self {
        self.win_background_color = wgpu::Color {
            r: color.r / 255.,
            g: color.g / 255.,
            b: color.b / 255.,
            a: color.a / 255.,
        };
        self
    }

    pub async fn build(&mut self, event_loop: &EventLoop<()>) -> Engine {
        // Engine::new(event_loop, self.win_size, self.win_resizable).await
        let window = WindowBuilder::new()
            .with_title(self.win_title.clone())
            .with_resizable(self.win_resizable)
            .with_inner_size(PhysicalSize::new(self.win_size.x, self.win_size.y))
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

        let adapter = create_adapter(&instance, &surface).await;
        let (device, queue) = create_device_and_queue(&adapter).await;

        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = create_surface_format(&surface_caps);

        let config = create_config(&surface_format, win_size, &surface_caps);
        surface.configure(&device, &config);

        // Swap target_fps and target_tps because this way we use loop_helper which is more consistent
        if self.target_fps.is_some() && self.target_tps.is_none() {
            let temp = self.target_fps;
            self.target_fps = self.target_tps;
            self.target_tps = temp;
        }

        // If target_fps is Some and target_tps is None then target_tps is fps
        let fps = self.target_fps.unwrap_or(144); // Doesn't matter because if target_fps is None and target_tps is None than use_target_tps is false
        let target_tps = self.target_tps.unwrap_or(fps);

        let time =
            crate::time::TimeManager::new(self.reset_rate, target_tps, self.target_tps.is_some());

        let tex_bind_layout = texture::create_bind_group_layout(&device, 0);
        let camera = Camera::new(false);
        let camera_buffer = camera::create_buffer(&device, camera.uniform);
        let camera_bind_group_layout = camera::create_bind_group_layout(&device);
        let camera_bind_group =
            camera::create_bind_group(&device, &camera_buffer, &camera_bind_group_layout);

        let window_size_uniform = WindowUniform {
            size: [1. / self.win_size.x as f32, 1. / self.win_size.y as f32],
        };
        let window_size_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("window size buffer"),
            contents: bytemuck::cast_slice(&[window_size_uniform]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });
        let window_bind_group_layout = create_win_layout(&device);
        let window_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &window_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: window_size_buffer.as_entire_binding(),
            }],
            label: Some("camera_bind_group"),
        });

        let instances = vec![];
        let instance_buffer = super::vert_buffers::create_inst_buffer(&device, &instances);
        let shader = device.create_shader_module(wgpu::include_wgsl!("shader.wgsl"));

        let render_pipeline_layout = create_render_pipeline_layout(
            &device,
            &tex_bind_layout,
            &camera_bind_group_layout,
            &window_bind_group_layout,
        );
        let render_pipeline =
            create_render_pipeline(&device, &render_pipeline_layout, &shader, &config);

        let (vertex_buffer, index_buffer) = super::vert_buffers::create_buffers(&device);
        let tex_coords_buffer =
            super::vert_buffers::create_tex_coords_buffer(&device, &[TexCoords::default()]);

        // We use the egui_winit_platform crate as the platform.
        let platform = Platform::new(PlatformDescriptor {
            physical_width: self.win_size.y as u32,
            physical_height: self.win_size.x as u32,
            scale_factor: window.scale_factor(),
            font_definitions: egui::FontDefinitions::default(),
            style: Default::default(),
        });

        // We use the egui_wgpu_backend crate as the render backend.
        let egui_rpass = egui_wgpu_backend::RenderPass::new(&device, surface_format, 1);

        let ui = Ui::new(platform, egui_rpass, self.show_engine_ui);

        let all_fields = AllFields {
            input: crate::prelude::Input::new(),
            window,
            win_bind_group: window_bind_group,
            win_size,

            win_background_color: self.win_background_color,
            surface,
            device,
            queue,
            config,
            tex_coords: vec![],

            render_pipeline,
            vertex_buffer,
            index_buffer,
            tex_coords_buffer,

            camera,
            camera_bind_group,
            camera_buffer,

            instance_buffer,
            instances,
            instances_rendered: 0,

            time,

            tex_bind: None,

            texture_amt_created: 0,

            ui,

            target_fps: self.target_fps,

            sound: Sound::new(self.use_sound),
        };
        Engine::new(all_fields)
    }
}

pub fn create_shader(device: &wgpu::Device) -> wgpu::ShaderModule {
    device.create_shader_module(wgpu::include_wgsl!("shader.wgsl"))
}

pub fn create_win_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
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
    })
}

async fn create_adapter(instance: &wgpu::Instance, surface: &wgpu::Surface) -> wgpu::Adapter {
    instance
        .request_adapter(&wgpu::RequestAdapterOptionsBase {
            power_preference: wgpu::PowerPreference::HighPerformance,
            force_fallback_adapter: false,
            compatible_surface: Some(surface),
        })
        .await
        .expect("Failed to create adapter")
}

async fn create_device_and_queue(adapter: &wgpu::Adapter) -> (wgpu::Device, wgpu::Queue) {
    let limits = wgpu::Limits {
        max_sampled_textures_per_shader_stage: 512,
        ..Default::default()
    };

    let mut features = wgpu::Features::TEXTURE_BINDING_ARRAY;
    features.extend(wgpu::Features::SAMPLED_TEXTURE_AND_STORAGE_BUFFER_ARRAY_NON_UNIFORM_INDEXING);

    adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                features,
                limits,
                label: None,
            },
            None,
        )
        .await
        .expect("failed to create device or queue")
}

fn create_surface_format(surface_caps: &wgpu::SurfaceCapabilities) -> wgpu::TextureFormat {
    surface_caps
        .formats
        .iter()
        .copied()
        .find(|f| f.is_srgb())
        .unwrap_or(surface_caps.formats[0])
}

fn create_config(
    surface_format: &wgpu::TextureFormat,
    size: PhysicalSize<u32>,
    surface_caps: &wgpu::SurfaceCapabilities,
) -> wgpu::SurfaceConfiguration {
    wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format: *surface_format,
        width: size.width,
        height: size.height,
        present_mode: wgpu::PresentMode::Immediate, //Used to be surface_caps.present_modes[0]
        alpha_mode: surface_caps.alpha_modes[0],
        view_formats: vec![],
    }
}

pub fn create_render_pipeline_layout(
    device: &wgpu::Device,
    tex_layout: &wgpu::BindGroupLayout,
    cam_layout: &wgpu::BindGroupLayout,
    window_layout: &wgpu::BindGroupLayout,
) -> wgpu::PipelineLayout {
    device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("Render Pipeline Layout"),
        bind_group_layouts: &[tex_layout, cam_layout, window_layout],
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
            buffers: &[Vertex::desc(), TexCoords::desc(), Instance::desc()],
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

pub struct AllFields {
    pub input: crate::prelude::Input,

    pub window: winit::window::Window,
    pub win_size: winit::dpi::PhysicalSize<u32>,
    pub win_background_color: wgpu::Color,
    pub win_bind_group: wgpu::BindGroup,

    pub surface: wgpu::Surface,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub config: wgpu::SurfaceConfiguration,

    pub render_pipeline: wgpu::RenderPipeline,
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub instance_buffer: wgpu::Buffer,
    pub camera_buffer: wgpu::Buffer,
    pub tex_coords_buffer: wgpu::Buffer,

    pub tex_coords: Vec<TexCoords>,
    pub instances: Vec<Instance>,
    pub instances_rendered: usize,

    pub tex_bind: Option<wgpu::BindGroup>,
    pub texture_amt_created: u32,

    pub camera: Camera,
    pub camera_bind_group: wgpu::BindGroup,

    pub time: TimeManager,

    pub target_fps: Option<u32>,

    pub ui: Ui,

    pub sound: Sound,
}

#[macro_export]
macro_rules! create_Engine_from_AllFields {
    ($all_fields: expr, $($field_name: ident)*) => {
        Engine {
            $($field_name: $all_fields.$field_name,)*
        }
    };
}
