use crate::renderer::Renderer;
use std::f32::consts::{E, PI};
use wgpu::Queue;

pub struct Texture {
    pub texture: wgpu::Texture,
    pub sampler: wgpu::Sampler,
    pub view: wgpu::TextureView,
    pub bind_group: wgpu::BindGroup,
}

impl Texture {
    pub fn new(width: u32, height: u32, format: wgpu::TextureFormat, renderer: &Renderer) -> Self {
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
            label: None,
        });
        let sampler = renderer
            .device
            .create_sampler(&wgpu::SamplerDescriptor::default());
        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        let bind_group = renderer
            .device
            .create_bind_group(&wgpu::BindGroupDescriptor {
                layout: &renderer.tex_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(&view),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::Sampler(&sampler),
                    },
                ],
                label: None,
            });

        Self {
            texture,
            sampler,
            view,
            bind_group,
        }
    }

    pub fn write(&self, queue: &Queue, data: &[u8]) {
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
                bytes_per_row: Some(4 * self.texture.width()),
                rows_per_image: Some(self.texture.height()),
            },
            self.texture.size(),
        );
    }
}

pub fn gaussian_texture(renderer: &Renderer, consts: &shared::Constants) {
    let texture = Texture::new(
        consts.sim.lengthscale as u32,
        consts.sim.lengthscale as u32,
        wgpu::TextureFormat::Rg16Snorm,
        &renderer,
    );
}

fn gaussian_number(x: f32, consts: &shared::Constants) -> f32 {
    1.0 / (2.0 * PI * consts.sim.standard_deviation * consts.sim.standard_deviation)
        * E.powf(
            -1.0 * (x - consts.sim.mean * consts.sim.mean)
                / (2.0 * consts.sim.standard_deviation * consts.sim.standard_deviation),
        )
}
