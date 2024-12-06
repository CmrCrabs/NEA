#![no_std]

use spirv_std::glam::{Vec3, Vec4,Vec2,Vec4Swizzles, Mat4};
use spirv_std::spirv;
use spirv_std::num_traits::Float;
use shared::SceneConstants;

#[spirv(vertex)]
pub fn main_vs(
    pos: Vec3,
    #[spirv(uniform, descriptor_set = 0, binding = 0)] camera_view_proj: &Mat4,

    #[spirv(position)] out_pos: &mut Vec4,
) {
    *out_pos = *camera_view_proj * pos.extend(1.0);
}

#[spirv(fragment)]
pub fn main_fs(
    output: &mut Vec4,
) {    
    *output = Vec4::new(1.0,1.0,1.0,1.0);
}
