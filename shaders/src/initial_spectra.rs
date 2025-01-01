use spirv_std::{spirv, image::Image};
use spirv_std::glam::{UVec3, Vec3Swizzles, Vec4Swizzles};
use shared::Constants;

#[spirv(compute(threads(128, 128)))]
pub fn main(
    #[spirv(workgroup_id)] id: UVec3,
    #[spirv(descriptor_set = 0, binding = 0)] consts: Constants,
    #[spirv(descriptor_set = 1, binding = 0)] wave_tex: &Image!(2D, format = rgba32f, sampled = false),
    #[spirv(descriptor_set = 2, binding = 0)] spectrum_tex: &Image!(2D, format = rg32f, sampled = false),
) {
    unsafe {
        wave_tex.write(id.xy(), consts.shader.base_color);
        spectrum_tex.write(id.xy(), consts.shader.base_color.xy());
    }
}
