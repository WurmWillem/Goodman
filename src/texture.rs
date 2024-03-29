use cgmath::vec2;
use image::GenericImageView;
use wgpu::Device;

pub struct Texture {
    #[allow(dead_code)]
    pub(crate) texture: wgpu::Texture,
    pub(crate) view: wgpu::TextureView,
    pub(crate) sampler: wgpu::Sampler,
    pub(crate) index: u32,
    inv_size: cgmath::Vector2<f32>,
}
impl Texture {
    pub(crate) fn get_inv_width(&self) -> f32 {
        self.inv_size.x
    }
    pub(crate) fn get_inv_height(&self) -> f32 {
        self.inv_size.y
    }

    pub(crate) fn from_bytes(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        index: u32,
        bytes: &[u8],
        use_near_filter_mode: bool,
    ) -> Result<Self, String> {
        let img = match image::load_from_memory(bytes) {
            Ok(img) => img,
            Err(_) => return Err("could not load image from memory".to_string()),
        };
        Ok(Self::from_image(
            device,
            queue,
            &img,
            index,
            None,
            use_near_filter_mode,
        ))
    }

    /*pub(crate) fn from_path(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        index: u32,
        label: &str,
    ) -> Result<Self> {
        let img = image::open(label)?;
        Ok(Self::from_image(device, queue, &img, index, Some(label)))
    }*/

    pub(crate) fn from_image(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        img: &image::DynamicImage,
        index: u32,
        label: Option<&str>,
        use_near_filter_mode: bool,
    ) -> Self {
        let rgba = img.to_rgba8();
        let dimensions = img.dimensions();

        let size = wgpu::Extent3d {
            width: dimensions.0,
            height: dimensions.1,
            depth_or_array_layers: 1,
        };
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label,
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });

        queue.write_texture(
            wgpu::ImageCopyTexture {
                aspect: wgpu::TextureAspect::All,
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
            },
            &rgba,
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(4 * dimensions.0),
                rows_per_image: Some(dimensions.1),
            },
            size,
        );

        let filter_mode = if use_near_filter_mode {
            wgpu::FilterMode::Nearest
        } else {
            wgpu::FilterMode::Linear
        };
        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: filter_mode,
            min_filter: filter_mode,
            mipmap_filter: filter_mode,
            ..Default::default()
        });

        let inv_size = vec2(1. / texture.width() as f32, 1. / texture.height() as f32);

        Self {
            texture,
            view,
            sampler,
            index,
            inv_size,
        }
    }
}

#[macro_export]
macro_rules! create_textures {
    ($engine: expr, $textures: expr, $($name: expr)*) => {
        $(
            let tex_bytes = include_bytes!($name);
            $textures.push($engine.create_texture(tex_bytes).unwrap());
        )*
       $engine.use_textures(&$textures);
    };
}

pub fn create_bind_group_layout(device: &Device, tex_amt: u32) -> wgpu::BindGroupLayout {
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        entries: &[
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::FRAGMENT,
                // This should match the filterable field of the
                // corresponding Texture entry above.
                ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 1,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Texture {
                    multisampled: false,
                    view_dimension: wgpu::TextureViewDimension::D2,
                    sample_type: wgpu::TextureSampleType::Float { filterable: true },
                },
                count: std::num::NonZeroU32::new(tex_amt),
            },
        ],
        label: Some("texture_bind_group_layout"),
    })
}

pub fn create_bind_group(
    device: &Device,
    tex_bind_group_layout: &wgpu::BindGroupLayout,
    textures: &Vec<Texture>,
) -> wgpu::BindGroup {
    let mut views = vec![];
    for tex in textures {
        views.push(&tex.view);
    }

    device.create_bind_group(&wgpu::BindGroupDescriptor {
        layout: tex_bind_group_layout,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::Sampler(&textures[0].sampler),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: wgpu::BindingResource::TextureViewArray(&views),
            },
        ],
        label: Some("texture_bind_group"),
    })
}

pub fn create_bind_group_single_tex(
    device: &Device,
    tex_bind_group_layout: &wgpu::BindGroupLayout,
    tex: &Texture,
) -> wgpu::BindGroup {
    device.create_bind_group(&wgpu::BindGroupDescriptor {
        layout: tex_bind_group_layout,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::Sampler(&tex.sampler),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: wgpu::BindingResource::TextureViewArray(&vec![&tex.view]),
            },
        ],
        label: Some("texture_bind_group"),
    })
}
