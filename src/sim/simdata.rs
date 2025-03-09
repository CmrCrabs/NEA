use glam::{Vec2, Vec4};
use rand::prelude::*;
use crate::engine::util::{bind_group_descriptor, Texture};
use shared::Constants;

pub struct SimData {
    pub gaussian_noise: Vec<Vec4>,
    pub gaussian_tex: Texture,
    pub layout: wgpu::BindGroupLayout,
    pub bind_group: wgpu::BindGroup,
}

impl SimData {
    pub fn new(device: &wgpu::Device, consts: &Constants) -> Self {
        let gaussian_tex = Texture::new_storage(
            consts.sim.size,
            consts.sim.size,
            wgpu::TextureFormat::Rgba32Float,
            device,
            "Gaussian",
        );
        let gaussian_noise = Self::guassian_noise(consts);

        let butterfly_tex = Texture::new_storage(
            consts.sim.size.ilog2(),
            consts.sim.size,
            wgpu::TextureFormat::Rgba32Float,
            device,
            "Butterfly",
        );

        let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                bind_group_descriptor(0, wgpu::TextureFormat::Rgba32Float),
                bind_group_descriptor(1, wgpu::TextureFormat::Rgba32Float),
            ],
            label: Some("Sim Data Layout"),
        });
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
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
