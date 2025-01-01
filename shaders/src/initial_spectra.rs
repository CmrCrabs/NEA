use spirv_std::{spirv, image::{Image, Image2d}, Sampler};
use spirv_std::glam::{UVec3, Vec3Swizzles, Vec4Swizzles, Vec2, Vec4};
use shared::Constants;

#[spirv(compute(threads(8,8)))]
pub fn main(
    #[spirv(workgroup_id)] id: UVec3,
    #[spirv(uniform, descriptor_set = 0, binding = 0)] consts: &Constants,
    #[spirv(descriptor_set = 1, binding = 0)] gaussian_tex: &Image2d,
    #[spirv(descriptor_set = 1, binding = 1)] sampler: &Sampler,
    #[spirv(descriptor_set = 2, binding = 0)] wave_tex: &Image!(2D, format = rgba32f, sampled = false),
    #[spirv(descriptor_set = 3, binding = 0)] spectrum_tex: &Image!(2D, format = rg32f, sampled = false),
) {
    unsafe {
        wave_tex.write(id.xy(), consts.shader.base_color);
        spectrum_tex.write(id.xy(), Ve4::new(1.0,1.0,1.0,1.0));
    }
}
