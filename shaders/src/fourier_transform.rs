use spirv_std::{
    spirv,
    num_traits::Float,
};
use spirv_std::glam::{UVec3, UVec2, Vec3Swizzles, Vec2, Vec4, Vec4Swizzles};
use shared::Constants;
use crate::evolve_spectra::ComplexMult;
use crate::StorageImage;


#[spirv(compute(threads(8,8)))]
pub fn main(
    #[spirv(global_invocation_id)] id: UVec3,
    #[spirv(uniform, descriptor_set = 0, binding = 0)] consts: &Constants,
    #[spirv(descriptor_set = 1, binding = 0)] wave_tex: &StorageImage,
    #[spirv(descriptor_set = 2, binding = 0)] height_map: &StorageImage,
    #[spirv(descriptor_set = 3, binding = 0)] tangent_map: &StorageImage,
) {
    let id = id.xy();
    let offset = consts.sim.size as f32 * 0.5 * consts.sim.mesh_step;
    let mut y = 0.0;
    for n in 0..consts.sim.size {
        for m in 0..consts.sim.size {
            let wave = wave_tex.read(UVec2::new(n,m));
            let x = Vec2::new(
                n as f32 * consts.sim.mesh_step - offset,
                m as f32 * consts.sim.mesh_step - offset
            );
            let exp = wave.xy().dot(x);
            let euler = Vec2::new(exp.cos(), exp.sin());
            y += ComplexMult(
                height_map.read(UVec2::new(n,m)).xy(),
                euler,
            ).x;
        }
    }
    unsafe {
        height_map.write(id, Vec4::new(0.0, y, 0.0, 1.0));
    }
}
