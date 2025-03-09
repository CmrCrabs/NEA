#![no_std]
// TODO: clean warnings
//#![deny(warnings)]
pub mod sim;
pub mod ui;
pub mod skybox;

use core::f32::consts;
use core::ops::{Add, Mul};

use spirv_std::glam::{Vec4, Vec3, UVec2, Vec2};
use spirv_std::image::Image2d;
use spirv_std::Sampler;
use spirv_std::{spirv, image::Image};
use spirv_std::num_traits::Float;
use shared::Constants;

type StorageImage = Image!(2D, format = rgba32f, sampled = false);

#[spirv(vertex)]
pub fn main_vs(
    pos: Vec4,
    uv: UVec2,
    #[spirv(instance_index)] instance_index: u32,
    #[spirv(uniform, descriptor_set = 0, binding = 0)] consts: &Constants,
    #[spirv(descriptor_set = 3, binding = 3)] displacement_map0: &StorageImage,
    #[spirv(descriptor_set = 3, binding = 4)] normal_map0: &StorageImage,
    #[spirv(descriptor_set = 3, binding = 5)] foam_map0: &StorageImage,
    #[spirv(descriptor_set = 4, binding = 3)] displacement_map1: &StorageImage,
    #[spirv(descriptor_set = 4, binding = 4)] normal_map1: &StorageImage,
    #[spirv(descriptor_set = 4, binding = 5)] foam_map1: &StorageImage,
    #[spirv(descriptor_set = 5, binding = 3)] displacement_map2: &StorageImage,
    #[spirv(descriptor_set = 5, binding = 4)] normal_map2: &StorageImage,
    #[spirv(descriptor_set = 5, binding = 5)] foam_map2: &StorageImage,
    #[spirv(position)] out_pos: &mut Vec4, out_normal: &mut Vec3,
    out_foam: &mut Vec3,
    out_world_pos: &mut Vec4,
) {
    let mut displacement = displacement_map0.read(uv) * consts.sim.lengthscale0_sf;
    displacement += displacement_map1.read(uv) * consts.sim.lengthscale1_sf;
    displacement += displacement_map2.read(uv) * consts.sim.lengthscale2_sf;
    let mut normal = normal_map0.read(uv) * consts.sim.lengthscale0_sf;
    normal += normal_map1.read(uv) * consts.sim.lengthscale1_sf;
    normal += normal_map2.read(uv) * consts.sim.lengthscale2_sf;
    let mut foam = foam_map0.read(uv) * consts.sim.lengthscale0_sf;
    foam += foam_map1.read(uv) * consts.sim.lengthscale1_sf;
    foam += foam_map2.read(uv) * consts.sim.lengthscale2_sf;

    let width = consts.sim.size as f32 * consts.sim.mesh_step;
    let x = instance_index % consts.sim.instances;
    let z  = instance_index / consts.sim.instances;
    let tiling_offset = Vec4::new(x as f32 * width, 0.0, z as f32 * width, 0.0) * 0.96;
    let positive_offset = Vec4::new(width * 0.5, 0.0, width * 0.5, 0.0) * (consts.sim.instances as f32 - 1.0);

    let centring_offset = Vec4::new(0.5 * width, consts.sim.height_offset, 0.5 * width, 0.0);
    let mut resultant_pos = pos + displacement - centring_offset + tiling_offset - positive_offset;
    resultant_pos.w = 1.0;
    *out_pos = consts.camera_viewproj * resultant_pos;
    *out_normal = normal.truncate();
    *out_foam = foam.truncate();
    *out_world_pos = resultant_pos;
}

#[inline(never)]
#[spirv(fragment)]
pub fn main_fs(
    normal: Vec3,
    foam: Vec3,
    world_pos: Vec4,
    #[spirv(uniform, descriptor_set = 0, binding = 0)] consts: &Constants,
    #[spirv(descriptor_set = 1, binding = 0)] sampler: &Sampler,
    #[spirv(descriptor_set = 2, binding = 0)] hdri: &Image2d,
    output: &mut Vec4,
    ) {
    let pos = world_pos.truncate();
    let n = normal.normalize();
    let l = (consts.shader.light.truncate() - pos).normalize();
    let v = (consts.eye.truncate() - pos).normalize();
    let h = (l + v).normalize();

    let dist = (consts.eye - world_pos).length();
    let max_dist = (consts.eye - 0.5 * consts.sim.size as f32 * consts.sim.mesh_step * consts.sim.instances as f32).length();
    let t = ((dist - consts.shader.fog_offset) / (max_dist - consts.shader.fog_offset)).clamp(0.0, 1.0);
    let fog = t.powf(consts.shader.fog_falloff) * consts.shader.fog_density;

    let foam = foam.x.max(0.0).min(1.0);
    
    let roughness = consts.shader.roughness + foam * consts.shader.foam_roughness;

    let fresnel = fresnel(n, v, &consts) * consts.shader.fresnel_sf;
    let l_scatter = subsurface_scattering(l, v, n, pos.y, roughness, consts);
    let l_env_reflected = hdri.sample(*sampler, equirectangular_to_uv(reflect(n, v))).truncate() * consts.shader.reflection_sf;
    let l_specular = match consts.shader.pbr {
        1 => pbr_specular(l, h, n, v, consts, roughness) * consts.shader.pbr_sf,
        _ => blinn_phong(n, h, consts) * fresnel,
    };
    let l_eye = lerp(
        (1.0 - fresnel) * l_scatter + l_specular + fresnel * l_env_reflected,
        consts.shader.foam_color.truncate(),
        foam,
    );
    let l_eye = lerp(
        l_eye,
        consts.shader.fog_color.truncate(),
        fog,
    );
    
    *output = reinhard_tonemap(l_eye).extend(1.0);
}

fn fresnel(n: Vec3, v: Vec3, consts: &Constants) -> f32 {
    let fresnel_n = Vec3::new(n.x * consts.shader.fresnel_normal_sf, n.y, n.z * consts.shader.fresnel_normal_sf); let f0 = ((consts.shader.air_ri - consts.shader.water_ri)
        / (consts.shader.air_ri + consts.shader.water_ri)).powf(2.0);
    f0 + (1.0 - f0) * (1.0 - fresnel_n.dot(v)).powf(consts.shader.fresnel_shine)
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
    let f = fresnel(h, v, consts) * consts.shader.fresnel_pbr_sf;
    let g = smith_g2(h, l, v, roughness);
    let d = ggx(n, h, roughness);
    let div = (4.0 * n.dot(l) * n.dot(v)).max(consts.shader.pbr_cutoff);
    f * g * d / div
}

fn ggx(n: Vec3, h: Vec3, roughness: f32) -> f32 {
    let nh = n.dot(h).max(0.0001).min(0.9999);
    roughness * roughness / (consts::PI * 
    ((roughness * roughness - 1.0) * nh.powf(2.0) + 1.0).powf(2.0))
}

fn smith_g2(h: Vec3, l: Vec3, v: Vec3, roughness: f32) -> f32 {
    1.0 / (1.0 + smith_g1(h, l, roughness) + smith_g1(h, v, roughness))
}

fn smith_g1(h: Vec3, s: Vec3, roughness: f32) -> f32 {
    let alpha = roughness * roughness;
    let hs = h.dot(s);
    let a = hs / (alpha * (1.0 - hs * hs).sqrt());
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

fn reflect(n: Vec3, v: Vec3) -> Vec3 {
    2.0 * (n * n.dot(v)) - v
}

fn equirectangular_to_uv(v: Vec3) -> Vec2 {
    Vec2::new(
        (v.z.atan2(v.x) + consts::PI) / consts::TAU,
        v.y.acos() / consts::PI,
    )
}

fn reinhard_tonemap(c: Vec3) -> Vec3 {
    c / (c + Vec3::splat(1.0))
}
