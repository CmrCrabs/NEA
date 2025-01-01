#![no_std]
//#![deny(warnings)]
pub mod initial_spectra;

use spirv_std::glam::{Vec4,Vec2};
use spirv_std::{spirv, image::Image2d, Sampler};
use shared::Constants;


#[spirv(vertex)]
pub fn main_vs(
    pos: Vec4,
    #[spirv(uniform, descriptor_set = 0, binding = 0)] consts: &Constants,
    #[spirv(position)] out_pos: &mut Vec4,
) {
    *out_pos = consts.camera_proj * pos;
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
    #[spirv(descriptor_set = 1, binding = 1)] sampler: &Sampler,
    out_col: &mut Vec4,

) {    
    *out_col = tex.sample(*sampler, uv) * col.powf(1.2);
}
