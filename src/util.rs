use crate::renderer::Renderer;
use wgpu::Queue;

pub struct Texture {
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
    pub smp_bind_group: wgpu::BindGroup,
    pub smp_layout: wgpu::BindGroupLayout,
}

impl Texture {
    pub fn new(
        width: u32,
        height: u32,
        format: wgpu::TextureFormat,
        renderer: &Renderer,
        label: &str,
    ) -> Self {
        let texture = renderer.device.create_texture(&wgpu::TextureDescriptor {
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
        let smp_layout = renderer
            .device
            .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[sampled_bind_group_descriptor(0)],
                label: Some(label),
            });
        let smp_bind_group = renderer
            .device
            .create_bind_group(&wgpu::BindGroupDescriptor {
                layout: &smp_layout,
                entries: &[wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&view),
                }],
                label: Some(label),
            });

        Self {
            texture,
            view,
            smp_bind_group,
            smp_layout,
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
}

pub struct StorageTexture {
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
    pub stg_bind_group: wgpu::BindGroup,
    pub stg_layout: wgpu::BindGroupLayout,
    pub smp_bind_group: wgpu::BindGroup,
    pub smp_layout: wgpu::BindGroupLayout,
}

impl StorageTexture {
    pub fn new(
        width: u32,
        height: u32,
        format: wgpu::TextureFormat,
        renderer: &Renderer,
        label: &str,
    ) -> Self {
        let texture = renderer.device.create_texture(&wgpu::TextureDescriptor {
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
        let stg_layout = renderer
            .device
            .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[bind_group_descriptor(0, format)],
                label: Some(label),
            });
        let stg_bind_group = renderer
            .device
            .create_bind_group(&wgpu::BindGroupDescriptor {
                layout: &stg_layout,
                entries: &[wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&view),
                }],
                label: Some(label),
            });
        let smp_layout = renderer
            .device
            .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[sampled_bind_group_descriptor(0)],
                label: Some(label),
            });
        let smp_bind_group = renderer
            .device
            .create_bind_group(&wgpu::BindGroupDescriptor {
                layout: &smp_layout,
                entries: &[wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&view),
                }],
                label: Some(label),
            });

        Self { 
            texture, 
            view,
            stg_bind_group,
            stg_layout,
            smp_layout,
            smp_bind_group,
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
}

pub fn bind_group_descriptor(binding: u32, format: wgpu::TextureFormat) -> wgpu::BindGroupLayoutEntry {
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
        visibility: wgpu::ShaderStages::FRAGMENT,
        ty: wgpu::BindingType::Texture {
            sample_type: wgpu::TextureSampleType::Float { filterable: false },
            view_dimension: wgpu::TextureViewDimension::D2,
            multisampled: false,
        },
        count: None,
    }
}
