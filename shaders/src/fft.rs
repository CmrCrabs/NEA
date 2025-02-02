use spirv_std::{
    spirv,
    num_traits::Float,
};
use core::f32::consts;
use crate::StorageImage;
use shared::Constants;
use spirv_std::glam::{UVec3, Vec3Swizzles, Vec2, Vec4};

#[spirv(compute(threads(1,8)))]
pub fn precompute_butterfly(
    #[spirv(global_invocation_id)] id: UVec3,
    #[spirv(uniform, descriptor_set = 0, binding = 0)] consts: &Constants,
    #[spirv(descriptor_set = 1, binding = 1)] butterfly_tex: &StorageImage,
) {
    let k = (id.y as f32 * consts.sim.size as f32 / 2.0_f32.powf(id.x as f32 + 1.0)) % consts.sim.size as f32;
    let exp = -2.0 * consts::PI * k / consts.sim.size as f32;
    let twiddle = Vec2::new(exp.cos(), exp.sin());

    let step = 2.0_f32.powf(id.x as f32);
    let wing = id.y as f32 % 2.0_f32.powf(id.x as f32 + 1.0) < step; //here

    let mut yt = id.y;
    let mut yb = id.y;
    if id.x == 0 {
        yt = bit_reverse(yt, consts.sim.logsize);
        yb = bit_reverse(yb, consts.sim.logsize);
        if wing {
            yb += 1;
        } else {
            yt -= 1;
        }
    } else {
        if wing {
            yb += step as u32;
        } else {
            yt -= step as u32;
        }
    }

    unsafe {
        butterfly_tex.write(id.xy(), Vec4::new(twiddle.x, twiddle.y, yt as f32, yb as f32));
    }
}

// algorithm from https://stackoverflow.com/questions/746171/efficient-algorithm-for-bit-reversal-from-msb-lsb-to-lsb-msb-in-c
fn bit_reverse(mut x: u32, size: u32) -> u32 {
    let mut n: u32 = 0;
    let mask: u32 = 0x1;
    for _ in 0..size {
        n <<= 1;
        n |= x & mask;
        x >>= 1;
    }
    n
}
