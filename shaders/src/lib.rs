#![no_std]
// TODO: clean warnings
//#![deny(warnings)]
pub mod initial_spectra;
pub mod evolve_spectra;
pub mod fft;
pub mod ui;
pub mod process_deltas;

use core::f32::consts;

use spirv_std::glam::{Vec4, Vec3, UVec2, Vec2};
use spirv_std::Sampler;
use spirv_std::{spirv, image::Image};
use spirv_std::num_traits::Float;
use shared::Constants;

type StorageImage = Image!(2D, format = rgba32f, sampled = false);
type SampledStorageImage = Image!(2D, format = rgba32f, sampled = false);

#[spirv(vertex)]
pub fn main_vs(
    pos: Vec4,
    uv: UVec2,
    #[spirv(uniform, descriptor_set = 0, binding = 0)] consts: &Constants,
    #[spirv(descriptor_set = 2, binding = 3)] displacement_map: &StorageImage,
    #[spirv(position)] out_pos: &mut Vec4,
    out_uv: &mut UVec2,
) {
    let offset = 0.5 * consts.sim.size as f32 * consts.sim.mesh_step;
    let offset = Vec4::new(offset, 0.0, offset, 0.0);
    let displacement = displacement_map.read(UVec2::new(uv.x as u32, uv.y as u32));
    let mut resultant_pos = pos + displacement - offset;
    resultant_pos.w = 1.0;
    *out_pos = consts.camera_proj * resultant_pos;
    *out_uv = uv;
}

#[inline(never)]
#[spirv(fragment)]
pub fn main_fs(
    #[spirv(position)] pos: Vec4,
    #[spirv(flat)] uv: UVec2,
    #[spirv(uniform, descriptor_set = 0, binding = 0)] consts: &Constants,
    #[spirv(descriptor_set = 1, binding = 0)] sampler: &Sampler,
    #[spirv(descriptor_set = 2, binding = 4)] normal_map: &SampledStorageImage,
    #[spirv(descriptor_set = 2, binding = 5)] foam_map: &SampledStorageImage,
    output: &mut Vec4,
    ) {
    let n = normal_map.read(UVec2::new(uv.x as u32, uv.y as u32)).truncate();
    //let n = normal_map.sample(*sampler, uv).truncate();
    let l = (consts.shader.light - pos).truncate().normalize();
    let v = (consts.view - pos).truncate().normalize();
    let h = (l + v).normalize();
   
    // TODO: adjust roughness based on foam density
    let roughness = consts.shader.roughness * consts.shader.roughness;

    let l_scatter = subsurface_scattering(l, v, n, pos.y, roughness, consts);
    //let l_scatter = Vec3::ZERO;
    let l_specular = Vec3::ZERO;
    let l_env_reflected = Vec3::ZERO;
    let fresnel = fresnel(h, v, &consts);
   
    // TODO: lerp foam
    let l_eye = (1.0 - fresnel) * l_scatter + l_specular + fresnel * l_env_reflected;

    *output = l_eye.extend(1.0);
}

fn fresnel(h: Vec3, v: Vec3, consts: &Constants) -> f32 {
    let f0 = ((consts.shader.air_ri - consts.shader.water_ri)
        / (consts.shader.air_ri + consts.shader.water_ri)).powf(2.0);
    f0 + (1.0 - f0) * (1.0 - h.dot(v)).powf(5.0)
}

fn subsurface_scattering(l: Vec3, v: Vec3, n: Vec3, h: f32, roughness: f32, consts: &Constants) -> Vec3 {
    // TODO: make v -
    let height_factor = consts.shader.ss_height * h.max(0.0) * l.dot(v).max(0.0).powf(4.0)
        * (0.5 - 0.5 * l.dot(n)).powf(3.0);
    let reflection_factor = consts.shader.ss_reflected * v.dot(n).max(0.0).powf(2.0);
    let lambert_factor = consts.shader.ss_lambert * l.dot(n).max(0.0) * consts.shader.scatter_color.truncate() * consts.shader.sun_color.truncate();
    let ambient_factor = consts.shader.ss_ambient * consts.shader.bubble_density * consts.shader.bubble_color.truncate() * consts.shader.sun_color.truncate();

    ((height_factor + reflection_factor) * consts.shader.scatter_color.truncate() * consts.shader.sun_color.truncate()) / (1.0 + lambda_ggx(roughness))
        + lambert_factor + ambient_factor
}

fn lambda_ggx(a: f32) -> f32 {
    ((1.0 + 1.0 / (a * a)).sqrt() - 1.0) * 0.5
}

fn geometric_attenuation(n: Vec3, h: Vec3, a: f32) -> f32 {
    let a2 = a * a;
    let nh = n.dot(h);
    a2 / (consts::PI * ((a2 - 1.0) * nh * nh + 1.0).powf(2.0))
}
