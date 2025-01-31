use spirv_std::{
    spirv,
    num_traits::Float,
};
use core::f32::consts::{self, PI};
use crate::StorageImage;
use shared::Constants;
use spirv_std::glam::{UVec3, Vec3Swizzles, Vec2, Vec4, Vec4Swizzles};

#[spirv(compute(threads(1,8)))]
pub fn precompute_butterfly(
    #[spirv(global_invocation_id)] id: UVec3,
    #[spirv(uniform, descriptor_set = 0, binding = 0)] consts: &Constants,
    #[spirv(descriptor_set = 1, binding = 1)] butterfly_tex: &StorageImage,
) {
    let k = id.y as f32 * (consts.sim.size as f32 / 2.0_f32.powf(id.x as f32 + 1.0)) % consts.sim.size as f32;
    let exp = 2.0 * consts::PI * k / consts.sim.size as f32;
    let twiddle = Vec2::new(exp.cos(), exp.sin());
    let butterfly_step = 2.0_f32.powf(id.x as f32);
    let butterfly_wing = if id.y as f32 % 2.0_f32.powf(id.x as f32 + 1.0) < butterfly_step {
        1.0
    } else {
        -1.0
    };

    let mut yt: f32 = id.y as f32;
    let mut yb: f32 = id.y as f32;
    if butterfly_wing == 1.0 {
        yb += butterfly_step;
    } else {
        yt -= butterfly_step;
    }

    if id.x == 0 {
        yt = bit_reverse(y1 as u32, consts.sim.size.ilog2()) as f32;
        yb = bit_reverse(y2 as u32, consts.sim.size.ilog2()) as f32;
    }

    unsafe {
        butterfly_tex.write(id.xy(), Vec4::new(twiddle.x, twiddle.y, yt, yb));
    }
}

// credit to
fn bit_reverse(mut x: u32, size: u32) -> u32 {
    let mut n: u32 = 0;
    let mask: u32 = 0x1;
    for i in 0..size {
        n <<= 1;
        n |= x & mask;
        x >>= 1;
    }
    n
}
