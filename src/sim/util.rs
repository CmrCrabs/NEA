use std::f32::consts::PI;

use glam::{Vec4, Vec2};
use shared::Constants;
use crate::{renderer::Renderer, util::Texture};
use rand::prelude::*;
use crate::util::bind_group_descriptor;

pub struct SimData {
    pub gaussian_noise: Vec<Vec4>,
    pub butterfly_data: Vec<Vec4>,
    pub gaussian_tex: Texture,
    pub butterfly_tex: Texture,
    pub layout: wgpu::BindGroupLayout,
    pub bind_group: wgpu::BindGroup,
}

impl SimData {
    pub fn new(renderer: &Renderer, consts: &Constants) -> Self {
        let gaussian_tex = Texture::new_storage(
            consts.sim.size,
            consts.sim.size,
            wgpu::TextureFormat::Rgba32Float,
            &renderer,
            "Gaussian"
        );
        let gaussian_noise = Self::guassian_noise(consts);

        let butterfly_tex = Texture::new_storage(
            consts.sim.size.ilog2(),
            consts.sim.size,
            wgpu::TextureFormat::Rgba32Float,
            &renderer,
            "Butterfly"
        );
        let butterfly_data = Self::butterfly_data(consts);

        let layout = renderer
            .device
            .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[ 
                   bind_group_descriptor(0),
                   bind_group_descriptor(1) 
                ],
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
                    }
                ],
                label: Some("Sim Data Textures"),
            });
        
        Self {
            gaussian_tex,
            gaussian_noise,
            butterfly_tex,
            butterfly_data,
            bind_group,
            layout,
        }
    }

    //TODO: seed with wavenumber?
    fn guassian_noise(consts: &Constants) -> Vec<Vec4> {
        let mut rng = rand::thread_rng();
        let mut data = vec![];
        for _ in 0..(consts.sim.size * consts.sim.size) {
            let gaussian_pair = Self::gaussian_number(
                rng.gen_range(0.0..1.0),
                rng.gen_range(0.0..1.0),
            );
            data.push(Vec4::new(
                gaussian_pair.x,
                gaussian_pair.y,
                0.0,
                1.0,
            ));
        }
        data
    }
    fn gaussian_number(u1: f32, u2: f32) -> Vec2 {
       Vec2::new( 
           (-2.0 * u1.ln()).sqrt() * (2.0 * std::f32::consts::PI * u2).cos(),
           (-2.0 * u1.ln()).sqrt() * (2.0 * std::f32::consts::PI * u2).sin()
       )
    }

    fn butterfly_data(consts: &Constants) -> Vec<Vec4> {
        let mut data = vec![];
        let n: f32 = consts.sim.size as f32;
        for y in 0..consts.sim.size {
            for x in 0..consts.sim.size.ilog2() {
                let k = (y as f32 * n / 2.0_f32.powf(x as f32 + 1.0)) % n;
                let w_n = Vec2::new(
                    ((2.0 * PI * k) / n).cos(),
                    ((2.0 * PI * k) / n).sin(),
                );
                let top = y as f32 % 2.0_f32.powf(x as f32 + 1.0) < 2.0_f32.powf(x as f32);
                data.push(Vec4::new(
                    w_n.x,
                    w_n.y,
                    0.0,
                    0.0,
                ));
            }
        }
        data
    }
}
