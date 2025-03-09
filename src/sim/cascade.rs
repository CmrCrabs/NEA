use crate::engine::util::{bind_group_descriptor, Texture};
use glam::{Vec2, Vec4};
use rand::prelude::*;
use shared::Constants;

pub struct Cascade {
    pub bind_group: wgpu::BindGroup,
    pub layout: wgpu::BindGroupLayout,
    pub h_displacement: Texture,
    pub h_slope: Texture,
    pub v_displacement: Texture,
    pub jacobian: Texture,
}

impl Cascade {
    pub fn new(device: &wgpu::Device, consts: &Constants) -> Self {
        let wave_texture = Texture::new_storage(
            consts.sim.size,
            consts.sim.size,
            wgpu::TextureFormat::Rgba32Float,
            &device,
            "Waves",
        );
        let initial_spectrum_texture = Texture::new_storage(
            consts.sim.size,
            consts.sim.size,
            wgpu::TextureFormat::Rgba32Float,
            &device,
            "Initial Spectrum",
        );
        let evolved_spectrum_texture = Texture::new_storage(
            consts.sim.size,
            consts.sim.size,
            wgpu::TextureFormat::Rgba32Float,
            &device,
            "Evolved Spectrum",
        );
        let displacement_map = Texture::new_storage(
            consts.sim.size,
            consts.sim.size,
            wgpu::TextureFormat::Rgba32Float,
            &device,
            "Displacement Map",
        );
        let normal_map = Texture::new_storage(
            consts.sim.size,
            consts.sim.size,
            wgpu::TextureFormat::Rgba32Float,
            &device,
            "Normal Map",
        );
        let foam_map = Texture::new_storage(
            consts.sim.size,
            consts.sim.size,
            wgpu::TextureFormat::Rgba32Float,
            &device,
            "Foam",
        );

        let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                bind_group_descriptor(0, wgpu::TextureFormat::Rgba32Float),
                bind_group_descriptor(1, wgpu::TextureFormat::Rgba32Float),
                bind_group_descriptor(2, wgpu::TextureFormat::Rgba32Float),
                bind_group_descriptor(3, wgpu::TextureFormat::Rgba32Float),
                bind_group_descriptor(4, wgpu::TextureFormat::Rgba32Float),
                bind_group_descriptor(5, wgpu::TextureFormat::Rgba32Float),
            ],
            label: Some("Storage Textures Layout"),
        });
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&wave_texture.view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::TextureView(&initial_spectrum_texture.view),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: wgpu::BindingResource::TextureView(&evolved_spectrum_texture.view),
                },
                wgpu::BindGroupEntry {
                    binding: 3,
                    resource: wgpu::BindingResource::TextureView(&displacement_map.view),
                },
                wgpu::BindGroupEntry {
                    binding: 4,
                    resource: wgpu::BindingResource::TextureView(&normal_map.view),
                },
                wgpu::BindGroupEntry {
                    binding: 5,
                    resource: wgpu::BindingResource::TextureView(&foam_map.view),
                },
            ],
            label: Some("Storage Textures"),
        });

        let h_displacement = Texture::new_storage(
            consts.sim.size,
            consts.sim.size,
            wgpu::TextureFormat::Rgba32Float,
            &device,
            "h_displacement",
        );
        let h_slope = Texture::new_storage(
            consts.sim.size,
            consts.sim.size,
            wgpu::TextureFormat::Rgba32Float,
            &device,
            "h_slope",
        );
        let jacobian = Texture::new_storage(
            consts.sim.size,
            consts.sim.size,
            wgpu::TextureFormat::Rgba32Float,
            &device,
            "jacobian",
        );
        let v_displacement = Texture::new_storage(
            consts.sim.size,
            consts.sim.size,
            wgpu::TextureFormat::Rgba32Float,
            &device,
            "v_displacment",
        );

        Self {
            layout,
            bind_group,
            h_slope,
            h_displacement,
            v_displacement,
            jacobian,
        }
    }
}
