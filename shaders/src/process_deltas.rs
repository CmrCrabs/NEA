use spirv_std::{
    spirv,
    num_traits::Float,
};
use spirv_std::glam::{UVec3, Vec3Swizzles, Vec3, Vec4};
use shared::Constants;
use crate::StorageImage;

#[spirv(compute(threads(8,8)))]
pub fn main(
    #[spirv(global_invocation_id)] id: UVec3,
    #[spirv(uniform, descriptor_set = 0, binding = 0)] consts: &Constants,
    #[spirv(descriptor_set = 1, binding = 0)] h_displacement: &StorageImage,
    #[spirv(descriptor_set = 2, binding = 0)] v_displacement: &StorageImage,
    #[spirv(descriptor_set = 3, binding = 0)] h_slope: &StorageImage,
    #[spirv(descriptor_set = 4, binding = 0)] jacobian: &StorageImage,
    #[spirv(descriptor_set = 5, binding = 3)] displacement_map: &StorageImage,
    #[spirv(descriptor_set = 5, binding = 4)] normal_map: &StorageImage,
    #[spirv(descriptor_set = 5, binding = 5)] foam_map: &StorageImage,
) {
    let dy = v_displacement.read(id.xy()).x;
    let dx = h_displacement.read(id.xy()).x;
    let dz = h_displacement.read(id.xy()).y;
    let displacement = Vec4::new(dx * consts.sim.choppiness, dy, dz * consts.sim.choppiness, 1.0);
    
    let nx = h_slope.read(id.xy()).x;
    let nz = h_slope.read(id.xy()).y;
    let normal = Vec3::new(-nx, 1.0, -nz).normalize().extend(1.0);

    let jxx = 1.0_f32 + consts.sim.choppiness * jacobian.read(id.xy()).x;
    let jzz = 1.0_f32 + consts.sim.choppiness * jacobian.read(id.xy()).y;
    let jxz = consts.sim.choppiness * v_displacement.read(id.xy()).y;
    let jacobian = -(jxx * jzz - jxz * jxz) + consts.sim.foam_bias;
    let mut accumulation = foam_map.read(id.xy()).x - consts.deltatime * consts.sim.foam_decay / jacobian.max(0.5);
    if jacobian <= 0.0 {
        accumulation += jacobian;
    }
    let foam = Vec3::splat(jacobian.max(accumulation)).extend(1.0);

    unsafe {
        displacement_map.write(id.xy(), displacement);
        normal_map.write(id.xy(), normal);
        foam_map.write(id.xy(), foam);
    }
}
