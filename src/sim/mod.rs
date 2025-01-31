use crate::{
    renderer::Renderer,
    util::{bind_group_descriptor, Texture},
};
use shared::Constants;

pub mod compute;
pub mod fft;
pub mod util;

pub struct Cascade {
    pub bind_group: wgpu::BindGroup,
    pub layout: wgpu::BindGroupLayout,
}

impl Cascade {
    pub fn new(renderer: &Renderer, consts: &Constants) -> Self {
        let wave_texture = Texture::new_storage(
            consts.sim.size,
            consts.sim.size,
            wgpu::TextureFormat::Rgba32Float,
            &renderer,
            "Waves",
        );
        let initial_spectrum_texture = Texture::new_storage(
            consts.sim.size,
            consts.sim.size,
            wgpu::TextureFormat::Rgba32Float,
            &renderer,
            "Spectrum",
        );
        let height_map = Texture::new_storage(
            consts.sim.size,
            consts.sim.size,
            wgpu::TextureFormat::Rgba32Float,
            &renderer,
            "Height Map",
        );
        let tangent_map = Texture::new_storage(
            consts.sim.size,
            consts.sim.size,
            wgpu::TextureFormat::Rgba32Float,
            &renderer,
            "Tangents",
        );
        let evolved_spectrum_texture = Texture::new_storage(
            consts.sim.size,
            consts.sim.size,
            wgpu::TextureFormat::Rgba32Float,
            &renderer,
            "Storage",
        );

        let layout = renderer
            .device
            .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[
                    bind_group_descriptor(0),
                    bind_group_descriptor(1),
                    bind_group_descriptor(2),
                    bind_group_descriptor(3),
                    bind_group_descriptor(4),
                ],
                label: Some("Storage Textures Layout"),
            });
        let bind_group = renderer
            .device
            .create_bind_group(&wgpu::BindGroupDescriptor {
                layout: &layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(&wave_texture.view),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::TextureView(
                            &initial_spectrum_texture.view,
                        ),
                    },
                    wgpu::BindGroupEntry {
                        binding: 2,
                        resource: wgpu::BindingResource::TextureView(
                            &evolved_spectrum_texture.view,
                        ),
                    },
                    wgpu::BindGroupEntry {
                        binding: 3,
                        resource: wgpu::BindingResource::TextureView(&height_map.view),
                    },
                    wgpu::BindGroupEntry {
                        binding: 4,
                        resource: wgpu::BindingResource::TextureView(&tangent_map.view),
                    },
                ],
                label: Some("Storage Textures"),
            });

        Self { layout, bind_group }
    }
}
