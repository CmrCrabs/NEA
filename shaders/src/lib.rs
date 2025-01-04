#![no_std]
//#![deny(warnings)]
pub mod initial_spectra;
pub mod evolve_spectra;
pub mod fourier_transform;

use spirv_std::glam::{Vec4,Vec2,UVec2, Vec4Swizzles};
use spirv_std::{spirv, image::{Image, Image2d}, Sampler};
use shared::Constants;

type StorageImage = Image!(2D, format = rgba32f, sampled = false);

#[spirv(vertex)]
pub fn main_vs(
    pos: Vec4,
    id: UVec2,
    #[spirv(uniform, descriptor_set = 0, binding = 0)] consts: &Constants,
    #[spirv(descriptor_set = 1, binding = 0)] height_map: &StorageImage,
    #[spirv(position)] out_pos: &mut Vec4,
) {
    let length = consts.sim.size as f32 * consts.sim.mesh_step;
    let mut resultant_pos = pos + height_map.read(id);
    resultant_pos.w = 1.0;
    *out_pos = consts.camera_proj * resultant_pos;
}

#[inline(never)]
#[spirv(fragment)]
pub fn main_fs(
    #[spirv(uniform, descriptor_set = 0, binding = 0)] consts: &Constants,
    output: &mut Vec4,
) {    
    *output = consts.shader.base_color;
}

#[spirv(vertex)]
pub fn ui_vs(
    pos: Vec2,
    uv: Vec2,
    col: Vec4,
    #[spirv(uniform, descriptor_set = 0, binding = 0)] consts: &Constants,
    #[spirv(position)] out_pos: &mut Vec4,
    out_uv: &mut Vec2,
    out_col: &mut Vec4,
) {
    *out_pos = Vec4::new(
        2.0 * pos.x / consts.width - 1.0,
        1.0 - 2.0 * pos.y / consts.height,
        0.0,
        1.0,
    );
    *out_uv = uv;
    *out_col = col;
}

#[spirv(fragment)]
pub fn ui_fs(
    uv: Vec2,
    col: Vec4,
    #[spirv(descriptor_set = 1, binding = 0)] tex: &Image2d,
    #[spirv(descriptor_set = 2, binding = 0)] sampler: &Sampler,
    out_col: &mut Vec4,

) {    
    *out_col = tex.sample(*sampler, uv) * col.powf(1.2);
}
