use crate::{
    renderer::Renderer,
    util::{StorageTexture, Texture},
};
use glam::Vec4;
use rand::prelude::*;
use shared::Constants;
use std::f32::consts::{E, PI};

pub mod compute;

pub struct Cascade {
    gaussian_texture: StorageTexture,
    gaussian_noise: Vec<Vec4>,
    wave_texture: StorageTexture,
    spectrum_texture: StorageTexture,
}

impl Cascade {
    pub fn new(renderer: &Renderer, consts: &Constants) -> Self {
        let gaussian_texture = StorageTexture::new(
            consts.sim.lengthscale,
            consts.sim.lengthscale,
            wgpu::TextureFormat::Rg32Float,
            &renderer,
        );
        let gaussian_noise = Cascade::guassian_noise(consts);

        let wave_texture = StorageTexture::new(
            consts.sim.lengthscale,
            consts.sim.lengthscale,
            wgpu::TextureFormat::Rgba32Float,
            &renderer,
        );
        let spectrum_texture = StorageTexture::new(
            consts.sim.lengthscale,
            consts.sim.lengthscale,
            wgpu::TextureFormat::Rg32Float,
            &renderer,
        );
        Self {
            gaussian_texture,
            gaussian_noise,
            wave_texture,
            spectrum_texture,
        }
    }

    //TODO: seed with wavenumber?
    fn guassian_noise(consts: &Constants) -> Vec<Vec4> {
        let mut rng = rand::thread_rng();
        let mut data = vec![];
        for _ in 0..(consts.sim.lengthscale * consts.sim.lengthscale) {
            data.push(Vec4::new(
                Self::gaussian_number(rng.gen_range(-1.0..1.0), consts),
                Self::gaussian_number(rng.gen_range(-1.0..1.0), consts),
                0.0,
                0.0,
            ));
        }
        data
    }
    fn gaussian_number(x: f32, consts: &shared::Constants) -> f32 {
        1.0 / (2.0 * PI * consts.sim.standard_deviation * consts.sim.standard_deviation)
            * E.powf(
                -1.0 * (x - consts.sim.mean * consts.sim.mean)
                    / (2.0 * consts.sim.standard_deviation * consts.sim.standard_deviation),
            )
    }
}
