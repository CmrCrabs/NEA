use shared::Constants;
use glam::{Vec4, Vec2};
use rand::prelude::*;
use crate::{util::Texture, renderer::Renderer};
use std::f32::consts::{E, PI};

mod compute;

pub struct Ocean {
    gaussian_texture: Texture,
    gaussian_noise: Vec<Vec4>,

    //wave_texture: Texture,
    //spectrum_texture: Texture,
}

impl Ocean {
    pub fn new(renderer: &Renderer, consts: &Constants) -> Self {
        let gaussian_texture = Texture::new(
                consts.sim.lengthscale,
                consts.sim.lengthscale,
                wgpu::TextureFormat::Rg16Snorm,
                &renderer,
        );
        let gaussian_noise = Ocean::guassian_noise(consts);

        Self {
            gaussian_texture,
            gaussian_noise,
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
                0.0
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

