use glam::{Vec4, Vec2};
use shared::Constants;
use crate::{renderer::Renderer, util::Texture};
use rand::prelude::*;

pub struct SimData {
    pub gaussian_noise: Vec<Vec4>,
    pub butterfly_data: Vec<Vec4>,
    pub gaussian_tex: Texture,
    pub butterfly_tex: Texture,
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
            consts.sim.size,
            consts.sim.size.ilog2(),
            wgpu::TextureFormat::Rgba32Float,
            &renderer,
            "Butterfly"
        );

        let butterfly_data = Self::butterfly_data(consts);
        Self {
            gaussian_tex,
            gaussian_noise,
            butterfly_tex,
            butterfly_data,
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
        for y in 0..consts.sim.size {
            for x in 0..consts.sim.size.ilog2() {
                data.push(Vec4::new(
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                ));
            }
        }
        data
    }
}
