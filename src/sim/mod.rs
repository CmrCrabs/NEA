use crate::{
    renderer::Renderer,
    util::{bind_group_descriptor, StorageTexture, Texture},
};
use glam::{Vec2, Vec4};
use rand::prelude::*;
use shared::Constants;

pub mod compute;
pub mod fft;

pub struct Cascade {
    pub stg_bind_group: wgpu::BindGroup,
    pub stg_layout: wgpu::BindGroupLayout,
    pub displacement_map: StorageTexture,
    pub normal_map: StorageTexture,
    pub foam_map: StorageTexture,
    pub h_displacement: StorageTexture,
    pub h_slope: StorageTexture,
    pub v_displacement: StorageTexture,
    pub jacobian: StorageTexture,
}

impl Cascade {
    pub fn new(renderer: &Renderer, consts: &Constants) -> Self {
        let wave_texture = StorageTexture::new(
            consts.sim.size,
            consts.sim.size,
            wgpu::TextureFormat::Rgba32Float,
            &renderer,
            "Waves",
        );
        let initial_spectrum_texture = StorageTexture::new(
            consts.sim.size,
            consts.sim.size,
            wgpu::TextureFormat::Rgba32Float,
            &renderer,
            "Initial Spectrum",
        );
        let evolved_spectrum_texture = StorageTexture::new(
            consts.sim.size,
            consts.sim.size,
            wgpu::TextureFormat::Rgba32Float,
            &renderer,
            "Evolved Spectrum",
        );
        let displacement_map = StorageTexture::new(
            consts.sim.size,
            consts.sim.size,
            wgpu::TextureFormat::Rgba32Float,
            &renderer,
            "Displacement Map",
        );
        let normal_map = StorageTexture::new(
            consts.sim.size,
            consts.sim.size,
            wgpu::TextureFormat::Rgba32Float,
            &renderer,
            "Normal Map",
        );
        let foam_map = StorageTexture::new(
            consts.sim.size,
            consts.sim.size,
            wgpu::TextureFormat::Rgba32Float,
            &renderer,
            "Foam",
        );

        let stg_layout =
            renderer
                .device
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
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
        let stg_bind_group = renderer
            .device
            .create_bind_group(&wgpu::BindGroupDescriptor {
                layout: &stg_layout,
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

        //TODO: optimise into 2 per rola
        let h_displacement = StorageTexture::new(
            consts.sim.size,
            consts.sim.size,
            wgpu::TextureFormat::Rgba32Float,
            &renderer,
            "h_displacement",
        );
        let h_slope = StorageTexture::new(
            consts.sim.size,
            consts.sim.size,
            wgpu::TextureFormat::Rgba32Float,
            &renderer,
            "h_slope",
        );
        let jacobian = StorageTexture::new(
            consts.sim.size,
            consts.sim.size,
            wgpu::TextureFormat::Rgba32Float,
            &renderer,
            "jacobian",
        );
        let v_displacement = StorageTexture::new(
            consts.sim.size,
            consts.sim.size,
            wgpu::TextureFormat::Rgba32Float,
            &renderer,
            "v_displacment",
        );


        Self {
            stg_layout,
            stg_bind_group,
            displacement_map,
            normal_map,
            foam_map,
            h_slope,
            h_displacement,
            v_displacement,
            jacobian
        }
    }
}

pub struct SimData {
    pub gaussian_noise: Vec<Vec4>,
    pub gaussian_tex: StorageTexture,
    pub layout: wgpu::BindGroupLayout,
    pub bind_group: wgpu::BindGroup,
}

impl SimData {
    pub fn new(renderer: &Renderer, consts: &Constants) -> Self {
        let gaussian_tex = StorageTexture::new(
            consts.sim.size,
            consts.sim.size,
            wgpu::TextureFormat::Rgba32Float,
            &renderer,
            "Gaussian",
        );
        let gaussian_noise = Self::guassian_noise(consts);

        let butterfly_tex = StorageTexture::new(
            consts.sim.size.ilog2(),
            consts.sim.size,
            wgpu::TextureFormat::Rgba32Float,
            &renderer,
            "Butterfly",
        );

        let layout = renderer
            .device
            .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[bind_group_descriptor(0, wgpu::TextureFormat::Rgba32Float), bind_group_descriptor(1, wgpu::TextureFormat::Rgba32Float)],
                label: Some("Sim Data Layout"),
            });
        let bind_group = renderer
            .device
            .create_bind_group(&wgpu::BindGroupDescriptor {
                layout: &layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(&gaussian_tex.view),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::TextureView(&butterfly_tex.view),
                    },
                ],
                label: Some("Sim Data Textures"),
            });

        Self {
            gaussian_tex,
            gaussian_noise,
            bind_group,
            layout,
        }
    }

    //TODO: seed with wavenumber?
    fn guassian_noise(consts: &Constants) -> Vec<Vec4> {
        let mut rng = rand::thread_rng();
        let mut data = vec![];
        for _ in 0..(consts.sim.size * consts.sim.size) {
            let gaussian_pair =
                Self::gaussian_number(rng.gen_range(0.0..1.0), rng.gen_range(0.0..1.0));
            data.push(Vec4::new(gaussian_pair.x, gaussian_pair.y, 0.0, 1.0));
        }
        data
    }
    fn gaussian_number(u1: f32, u2: f32) -> Vec2 {
        Vec2::new(
            (-2.0 * u1.ln()).sqrt() * (2.0 * std::f32::consts::PI * u2).cos(),
            (-2.0 * u1.ln()).sqrt() * (2.0 * std::f32::consts::PI * u2).sin(),
        )
    }
}
