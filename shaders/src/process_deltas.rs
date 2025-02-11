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
    #[spirv(descriptor_set = 1, binding = 0)] dx_dz: &StorageImage,
    #[spirv(descriptor_set = 2, binding = 0)] dy_dxz: &StorageImage,
    #[spirv(descriptor_set = 3, binding = 0)] dyx_dyz: &StorageImage,
    #[spirv(descriptor_set = 4, binding = 0)] dxx_dzz: &StorageImage,
    #[spirv(descriptor_set = 5, binding = 3)] displacement_map: &StorageImage,
    #[spirv(descriptor_set = 5, binding = 4)] normal_map: &StorageImage,
    #[spirv(descriptor_set = 5, binding = 5)] foam_map: &StorageImage,
) {
    let dxdz = dx_dz.read(id.xy());
    let dydxz = dy_dxz.read(id.xy());
    let dyxdyz = dyx_dyz.read(id.xy());
    let dxxdzz = dxx_dzz.read(id.xy());
    unsafe {
        displacement_map.write(id.xy(), Vec4::new(
            consts.sim.choppiness * dxdz.x, dydxz.x, consts.sim.choppiness * dxdz.y, 1.0
        ));
    }
}
