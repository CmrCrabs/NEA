use spirv_std::{
    spirv,
    num_traits::Float,
};
use spirv_std::glam::{UVec3, Vec3Swizzles, Vec2, Vec4, Vec4Swizzles};
use shared::Constants;
use crate::StorageImage;

#[spirv(compute(threads(8,8)))]
pub fn main(
    #[spirv(global_invocation_id)] id: UVec3,
    #[spirv(uniform, descriptor_set = 0, binding = 0)] consts: &Constants,
    #[spirv(descriptor_set = 1, binding = 0)] h_displacement: &StorageImage,
    #[spirv(descriptor_set = 2, binding = 0)] h_slope: &StorageImage,
    #[spirv(descriptor_set = 3, binding = 3)] displacement_map: &StorageImage,
    #[spirv(descriptor_set = 3, binding = 4)] normal_map: &StorageImage,
    #[spirv(descriptor_set = 3, binding = 5)] foam_map: &StorageImage,
) {
    let dy = h_displacement.read(id.xy()).x;
    let dx = h_displacement.read(id.xy()).y;
    let dz = h_slope.read(id.xy()).x;
    unsafe {
        displacement_map.write(id.xy(), Vec4::new(
            consts.sim.choppiness * dx,
            dy, 
            consts.sim.choppiness * dz, 
            1.0
        ));
    }
}
