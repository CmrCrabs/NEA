#![no_std]
// TODO: clean warnings
//#![deny(warnings)]
pub mod initial_spectra;
pub mod evolve_spectra;
pub mod fft;
pub mod ui;
pub mod process_deltas;

use core::f32::consts;
use core::ops::{Add,Mul};

use spirv_std::glam::{Vec4, Vec3, UVec2, Vec2};
use spirv_std::Sampler;
use spirv_std::{spirv, image::Image};
use spirv_std::num_traits::Float;
use shared::Constants;

type StorageImage = Image!(2D, format = rgba32f, sampled = false);
type SampledStorageImage = Image!(2D, format = rgba32f, sampled = true);

#[spirv(vertex)]
pub fn main_vs(
    pos: Vec4,
    uv: UVec2,
    #[spirv(uniform, descriptor_set = 0, binding = 0)] consts: &Constants,
    #[spirv(descriptor_set = 2, binding = 0)] displacement_map: &StorageImage,
    #[spirv(position)] out_pos: &mut Vec4,
    out_uv: &mut UVec2,
) { let offset = 0.5 * consts.sim.size as f32 * consts.sim.mesh_step;
    let offset = Vec4::new(offset, 0.0, offset, 0.0);
    let displacement = displacement_map.read(UVec2::new(uv.x as _, uv.y as _));
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
    #[spirv(descriptor_set = 3, binding = 0)] normal_map: &StorageImage,
    #[spirv(descriptor_set = 4, binding = 0)] foam_map: &StorageImage,
    output: &mut Vec4,
    ) {
    // TODO: fix vectors
    let n = normal_map.read(UVec2::new(uv.x as u32, uv.y as u32)).truncate();
    //let n = normal_map.sample(*sampler, Vec2::new(uv.x as _, uv.y as _)).truncate();
    let l = (consts.shader.light - pos).truncate().normalize();
    let v = (consts.eye - pos).truncate().normalize();
    let h = (l + v).normalize();
 
    //let foam = foam_map.sample(*sampler, Vec2::new(uv.x as _, uv.y as _)).x;
    let foam = foam_map.read(UVec2::new(uv.x as _, uv.y as _)).x;
    
    let roughness = consts.shader.roughness + foam * consts.shader.foam_roughness;

    let fresnel = fresnel(h, v, &consts);
    let l_scatter = subsurface_scattering(l, v, n, pos.y, roughness, consts);
    let l_env_reflected = Vec3::ZERO;
    // TODO check h as microfacet normal vs halfway
    let l_specular = match consts.shader.pbr {
        1 => pbr_specular(l, h, n, v, consts, roughness),
        _ => blinn_phong(n, h, consts),
    };

    let l_eye = lerp(
        (1.0 - fresnel) * l_scatter + l_specular + fresnel * l_env_reflected,
        consts.shader.foam_color.truncate(),
        foam,
    );

    *output = l_eye.extend(1.0);
}

fn fresnel(n: Vec3, v: Vec3, consts: &Constants) -> f32 {
    let f0 = ((consts.shader.air_ri - consts.shader.water_ri)
        / (consts.shader.air_ri + consts.shader.water_ri)).powf(2.0);
    f0 + (1.0 - f0) * (1.0 - n.dot(v)).powf(5.0)
}

fn subsurface_scattering(l: Vec3, v: Vec3, n: Vec3, height: f32, roughness: f32, consts: &Constants) -> Vec3 {
    let height_factor = consts.shader.ss_height * height.max(0.0) * l.dot(-v).max(0.0).powf(4.0)
        * (0.5 - 0.5 * l.dot(n)).powf(3.0);
    let reflection_factor = consts.shader.ss_reflected * v.dot(n).max(0.0).powf(2.0);
    let lambert_factor = consts.shader.ss_lambert * l.dot(n).max(0.0) * consts.shader.scatter_color.truncate() * consts.shader.sun_color.truncate();
    let ambient_factor = consts.shader.ss_ambient * consts.shader.bubble_density * consts.shader.bubble_color.truncate() * consts.shader.sun_color.truncate();

    ((height_factor + reflection_factor) * consts.shader.scatter_color.truncate() * consts.shader.sun_color.truncate()) / (1.0 + lambda_ggx(roughness))
        + lambert_factor + ambient_factor
}

fn pbr_specular(l: Vec3, h: Vec3, n: Vec3, v: Vec3, consts: &Constants, roughness: f32) -> Vec3 {
    consts.shader.sun_color.truncate() * microfacet_brdf(l, h, n, v, consts, roughness)
}

fn microfacet_brdf(l: Vec3, h: Vec3, n: Vec3, v: Vec3, consts: &Constants, roughness: f32) -> f32 {
    let f = fresnel(n, v, consts);
    let g = smith_g2(h, l, v, roughness);
    let d = ggx(n, h, roughness);
    f * g * d / (4.0 * n.dot(l) * n.dot(v))
}

fn ggx(n: Vec3, h: Vec3, roughness: f32) -> f32 {
    roughness * roughness / (consts::PI * 
    ((roughness * roughness - 1.0) * n.dot(h).powf(2.0) + 1.0).powf(2.0))
}

fn smith_g2(h: Vec3, l: Vec3, v: Vec3, roughness: f32) -> f32 {
    1.0 / (1.0 + smith_g1(h, l, roughness) + smith_g1(h, v, roughness))
}

// TODO: h as microfacet normal
fn smith_g1(h: Vec3, s: Vec3, roughness: f32) -> f32 {
    let alpha = roughness * roughness;
    let hs = h.dot(s);
    let a = hs / (alpha * (1.0 - hs * hs).sqrt());
    // TODO multiply ggx by a?
    1.0 / (1.0 + lambda_ggx(a))
}

fn lambda_ggx(a: f32) -> f32 {
    ((1.0 + 1.0 / (a * a)).sqrt() - 1.0) * 0.5
}

fn lerp<T: Add<Output = T> + Mul<f32, Output = T>>(a: T, b: T, t: f32) -> T { 
    a * (1.0 - t) + b * t
}

fn blinn_phong(n: Vec3, h: Vec3, consts: &Constants) -> Vec3 {
    n.dot(h).max(0.0).powf(consts.shader.shininess) * consts.shader.sun_color.truncate()
}
