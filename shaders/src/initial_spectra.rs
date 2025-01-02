use spirv_std::{
    spirv, 
    image::{Image, Image2d}, 
    Sampler,
    num_traits::FloatConst,
};
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
    let dk = 2 * FloatConst.PI() / consts.lengthscale;
    let n = id.x;
    let m = id.y;
    let x = Vec2::new(id.x,id.y);
    let k = Vec2::new(n,m) * dk;
    let mag_k = k.magnitude();
    // when multiple cascades implement cutoff check here if / else
    let theta = 1.0;


    //// wave
    // create k_x, k_z, store x,y
    // create dispersion relation, store z
    // create dw/dk, store w


    unsafe {
        wave_tex.write(id.xy(), consts.shader.base_color);
        spectrum_tex.write(id.xy(), Vec4::new(1.0,1.0,1.0,1.0));
    }
}
