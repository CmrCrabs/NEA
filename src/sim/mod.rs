use crate::{renderer::Renderer, util::Texture};
use shared::Constants;

pub mod compute;
pub mod util;

pub struct Cascade {
    pub wave_texture: Texture,
    pub initial_spectrum_texture: Texture,
    pub evolved_spectrum_texture: Texture,
    pub height_map: Texture,
    pub tangent_map: Texture,
}

impl Cascade {
    pub fn new(renderer: &Renderer, consts: &Constants) -> Self {
        let wave_texture = Texture::new_storage(
            consts.sim.size,
            consts.sim.size,
            wgpu::TextureFormat::Rgba32Float,
            &renderer,
            "Waves"
        );
        let initial_spectrum_texture = Texture::new_storage(
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
        let evolved_spectrum_texture = Texture::new_storage(
            consts.sim.size,
            consts.sim.size,
            wgpu::TextureFormat::Rgba32Float,
            &renderer,
            "Storage"
        );

        Self {
            wave_texture,
            initial_spectrum_texture,
            height_map,
            tangent_map,
            evolved_spectrum_texture
        }
    }
}
