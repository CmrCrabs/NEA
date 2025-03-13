use image::io::Reader;
use image::GenericImageView;
use std::fs::File;
use std::io::Cursor;
use std::io::Read;
use wgpu::util::DeviceExt;
use wgpu::Queue;

pub struct Texture {
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
    pub bind_group: wgpu::BindGroup,
    pub layout: wgpu::BindGroupLayout,
}

impl Texture {
    pub fn new_sampled(
        width: u32,
        height: u32,
        format: wgpu::TextureFormat,
        device: &wgpu::Device,
        label: &str,
    ) -> Self {
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            size: wgpu::Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
            label: Some(label),
        });
        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[sampled_bind_group_descriptor(0)],
            label: Some(label),
        });
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::TextureView(&view),
            }],
            label: Some(label),
        });

        Self {
            texture,
            view,
            bind_group,
            layout,
        }
    }

    pub fn new_storage(
        width: u32,
        height: u32,
        format: wgpu::TextureFormat,
        device: &wgpu::Device,
        label: &str,
    ) -> Self {
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            size: wgpu::Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format,
            usage: wgpu::TextureUsages::STORAGE_BINDING
                | wgpu::TextureUsages::COPY_DST
                | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
            label: Some(label),
        });
        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[bind_group_descriptor(0, format)],
            label: Some(label),
        });
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::TextureView(&view),
            }],
            label: Some(label),
        });

        Self {
            texture,
            view,
            layout,
            bind_group,
        }
    }

    pub fn write(&self, queue: &Queue, data: &[u8], size: u32) {
        queue.write_texture(
            wgpu::ImageCopyTexture {
                texture: &self.texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            data,
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(size * self.texture.width()),
                rows_per_image: Some(self.texture.height()),
            },
            self.texture.size(),
        );
    }

    pub fn from_file(device: &wgpu::Device, queue: &Queue, label: &str, file: &str) -> Self {
        // cant include_bytes! as filepath is non static
        let mut file_data = Vec::new();
        File::open(file)
            .expect("failed to open file")
            .read_to_end(&mut file_data)
            .expect("failed to read file");

        let diffuse_image = Reader::new(Cursor::new(file_data))
            .with_guessed_format()
            .expect("failed to guess format")
            .decode()
            .expect("failed to decode image");
        let diffuse_rgba = diffuse_image.to_rgba32f();
        let dimensions = diffuse_image.dimensions();
        let texture_size = wgpu::Extent3d {
            width: dimensions.0,
            height: dimensions.1,
            depth_or_array_layers: 1,
        };

        let texture = device.create_texture_with_data(
            queue,
            &wgpu::TextureDescriptor {
                size: texture_size,
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::Rgba32Float,
                usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
                label: None,
                view_formats: &[],
            },
            wgpu::util::TextureDataOrder::default(),
            super::cast_slice(diffuse_rgba.as_raw()),
        );
        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[sampled_bind_group_descriptor(0)],
            label: Some(label),
        });
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::TextureView(&view),
            }],
            label: Some(label),
        });
        Self {
            texture,
            view,
            layout,
            bind_group,
        }
    }
}


// debatably redundant as not app agnositc, but reduces LOC significantly
pub fn bind_group_descriptor(
    binding: u32,
    format: wgpu::TextureFormat,
) -> wgpu::BindGroupLayoutEntry {
    wgpu::BindGroupLayoutEntry {
        binding,
        visibility: wgpu::ShaderStages::COMPUTE | wgpu::ShaderStages::VERTEX_FRAGMENT,
        ty: wgpu::BindingType::StorageTexture {
            access: wgpu::StorageTextureAccess::ReadWrite,
            format,
            view_dimension: wgpu::TextureViewDimension::D2,
        },
        count: None,
    }
}

pub fn sampled_bind_group_descriptor(binding: u32) -> wgpu::BindGroupLayoutEntry {
    wgpu::BindGroupLayoutEntry {
        binding,
        visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
        ty: wgpu::BindingType::Texture {
            sample_type: wgpu::TextureSampleType::Float { filterable: false },
            view_dimension: wgpu::TextureViewDimension::D2,
            multisampled: false,
        },
        count: None,
    }
}
