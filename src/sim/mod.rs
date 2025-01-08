use crate::{renderer::Renderer, util::Texture};
use glam::{Vec4, Vec2};
use rand::prelude::*;
use shared::Constants;

pub mod compute;

pub struct Cascade {
    pub gaussian_texture: Texture,
    pub gaussian_noise: Vec<Vec4>,
    pub wave_texture: Texture,
    pub spectrum_texture: Texture,
    pub storage_texture: Texture,
    pub height_map: Texture,
    pub tangent_map: Texture,
}

impl Cascade {
    pub fn new(renderer: &Renderer, consts: &Constants) -> Self {
        let gaussian_texture = Texture::new_storage(
            consts.sim.size,
            consts.sim.size,
            wgpu::TextureFormat::Rgba32Float,
            &renderer,
            "Gaussian"
        );
        let gaussian_noise = Cascade::guassian_noise(consts);

        let wave_texture = Texture::new_storage(
            consts.sim.size,
            consts.sim.size,
            wgpu::TextureFormat::Rgba32Float,
            &renderer,
            "Waves"
        );
        let spectrum_texture = Texture::new_storage(
            consts.sim.size,
            consts.sim.size,
            wgpu::TextureFormat::Rgba32Float,
            &renderer,
            "Spectrum"
        );
        let height_map = Texture::new_storage(
            consts.sim.size,
            consts.sim.size,
            wgpu::TextureFormat::Rgba32Float,
            &renderer,
            "Height Map"
        );
        let tangent_map = Texture::new_storage(
            consts.sim.size,
            consts.sim.size,
            wgpu::TextureFormat::Rgba32Float,
            &renderer,
            "Tangents"
        );
        let storage_texture = Texture::new_storage(
            consts.sim.size,
            consts.sim.size,
            wgpu::TextureFormat::Rgba32Float,
            &renderer,
            "Storage"
        );

        Self {
            gaussian_texture,
            gaussian_noise,
            wave_texture,
            spectrum_texture,
            height_map,
            tangent_map,
            storage_texture
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
}
