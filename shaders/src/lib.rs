#![no_std]
// TODO: clean warnings
//#![deny(warnings)]
pub mod initial_spectra;
pub mod evolve_spectra;
pub mod fourier_transform;
pub mod fft;
pub mod ui;

use spirv_std::glam::{Vec4,UVec2};
use spirv_std::{spirv, image::Image};
use shared::Constants;

type StorageImage = Image!(2D, format = rgba32f, sampled = false);

#[spirv(vertex)]
pub fn main_vs(
    pos: Vec4,
    uv: UVec2,
    #[spirv(uniform, descriptor_set = 0, binding = 0)] consts: &Constants,
    #[spirv(descriptor_set = 1, binding = 3)] height_map: &StorageImage,
    #[spirv(position)] out_pos: &mut Vec4,
    out_h: &mut f32,
) {
    let offset = 0.5 * consts.sim.size as f32 * consts.sim.mesh_step;
    let offset = Vec4::new(offset, 0.0, offset, 0.0);
    let displacement = height_map.read(uv);
    let mut resultant_pos = pos + displacement - offset;
    resultant_pos.w = 1.0;
    *out_pos = consts.camera_proj * resultant_pos;
    *out_h = resultant_pos.y;
}

#[inline(never)]
#[spirv(fragment)]
pub fn main_fs(
    h: f32,
    #[spirv(uniform, descriptor_set = 0, binding = 0)] consts: &Constants,
    output: &mut Vec4,
) { 
    let mut c = consts.shader.base_color * h.abs(); 
    c.w = 1.0;
    *output = c;
}
