use spirv_std::{
    spirv,
    num_traits::Float,
};
use spirv_std::glam::{UVec3, UVec2, Vec3Swizzles, Vec2, Vec4, Vec4Swizzles};
use shared::Constants;
use crate::evolve_spectra::complex_mult;
use crate::StorageImage;

#[spirv(compute(threads(8,8)))]
pub fn main(
    #[spirv(global_invocation_id)] id: UVec3,
    #[spirv(uniform, descriptor_set = 0, binding = 0)] consts: &Constants,
    #[spirv(descriptor_set = 1, binding = 0)] wave_tex: &StorageImage,
    #[spirv(descriptor_set = 2, binding = 0)] butterfly_tex: &StorageImage,
    #[spirv(descriptor_set = 3, binding = 0)] storage_tex: &StorageImage,
    #[spirv(descriptor_set = 4, binding = 0)] height_map: &StorageImage,
    #[spirv(descriptor_set = 5, binding = 0)] tangent_map: &StorageImage,
) {
    let id = id.xy();
    let x = Vec2::new(id.x as f32 * consts.sim.mesh_step, id.y as f32 * consts.sim.mesh_step);

    let mut y = 0.0;
    let mut dx = 0.0;
    let mut dz = 0.0;
    for n in 0..consts.sim.size {
        for m in 0..consts.sim.size {
            let pos = UVec2::new(m,n);
            let k = wave_tex.read(pos).xy();
            let exponent = k.dot(x);
            let euler = Vec2::new(exponent.cos(), exponent.sin());
            y += complex_mult(
                storage_tex.read(pos).xy(),
                euler
            ).x;
            dx += complex_mult(
                tangent_map.read(pos).xy(), 
                euler
            ).x;
            dz += complex_mult(
                tangent_map.read(pos).zw(), 
                euler
            ).x;
        }
    }
    unsafe {
        height_map.write(id, Vec4::new(dx * consts.sim.choppiness, y, dz * consts.sim.choppiness, 1.0));
    }
}
