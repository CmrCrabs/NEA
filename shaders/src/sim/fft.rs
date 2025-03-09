use spirv_std::{
    spirv,
    num_traits::Float,
};
use core::f32::consts;
use crate::{sim::evolve_spectra::complex_mult, StorageImage};
use shared::{Constants, FFTData};
use spirv_std::glam::{UVec3, UVec2, Vec3Swizzles, Vec2, Vec4, Vec4Swizzles};


// algorithm referenced from GPGPU TODO: credit
#[spirv(compute(threads(8,8)))]
pub fn hstep_ifft(
    #[spirv(global_invocation_id)] id: UVec3,
    #[spirv(push_constant)] data: &FFTData,
    #[spirv(descriptor_set = 0, binding = 1)] butterfly_tex: &StorageImage,
    #[spirv(descriptor_set = 1, binding = 0)] pingpong0: &StorageImage,
    #[spirv(descriptor_set = 2, binding = 0)] pingpong1: &StorageImage,
) {
    let butterfly_data: Vec4 = butterfly_tex.read(UVec2::new(data.stage, id.x));
    let twiddle: Vec2 = butterfly_data.xy();
    let indices: UVec2 = UVec2::new(butterfly_data.z as u32, butterfly_data.w as u32);

    if data.pingpong == 0 {
        let top_signal0 = pingpong0.read(UVec2::new(indices.x, id.y)).xy();
        let top_signal1 = pingpong0.read(UVec2::new(indices.x, id.y)).zw();
        let bottom_signal0 = pingpong0.read(UVec2::new(indices.y, id.y)).xy();
        let bottom_signal1 = pingpong0.read(UVec2::new(indices.y, id.y)).zw();

        let h0 = top_signal0 + complex_mult(twiddle, bottom_signal0);
        let h1 = top_signal1 + complex_mult(twiddle, bottom_signal1);

        unsafe {
            pingpong1.write(id.xy(), Vec4::new(h0.x, h0.y, h1.x, h1.y));
        }
    } else if data.pingpong == 1 {
        let top_signal0 = pingpong1.read(UVec2::new(indices.x, id.y)).xy();
        let top_signal1 = pingpong1.read(UVec2::new(indices.x, id.y)).zw();
        let bottom_signal0 = pingpong1.read(UVec2::new(indices.y, id.y)).xy();
        let bottom_signal1 = pingpong1.read(UVec2::new(indices.y, id.y)).zw();

        let h0 = top_signal0 + complex_mult(twiddle, bottom_signal0);
        let h1 = top_signal1 + complex_mult(twiddle, bottom_signal1);

        unsafe {
            pingpong0.write(id.xy(), Vec4::new(h0.x, h0.y, h1.x, h1.y));
        }
    }
}

// algorithm referenced from GPGPU TODO: credit
#[spirv(compute(threads(8,8)))]
pub fn vstep_ifft(
    #[spirv(global_invocation_id)] id: UVec3,
    #[spirv(push_constant)] data: &FFTData,
    #[spirv(descriptor_set = 0, binding = 1)] butterfly_tex: &StorageImage,
    #[spirv(descriptor_set = 1, binding = 0)] pingpong0: &StorageImage,
    #[spirv(descriptor_set = 2, binding = 0)] pingpong1: &StorageImage,
) {
    let butterfly_data: Vec4 = butterfly_tex.read(UVec2::new(data.stage, id.y));
    let twiddle: Vec2 = butterfly_data.xy();
    let indices: UVec2 = UVec2::new(butterfly_data.z as u32, butterfly_data.w as u32);

    if data.pingpong == 0 {
        let top_signal0 = pingpong0.read(UVec2::new(id.x, indices.x)).xy();
        let top_signal1 = pingpong0.read(UVec2::new(id.x, indices.x)).zw();
        let bottom_signal0 = pingpong0.read(UVec2::new(id.x, indices.y)).xy();
        let bottom_signal1 = pingpong0.read(UVec2::new(id.x, indices.y)).zw();

        let h0 = top_signal0 + complex_mult(twiddle, bottom_signal0);
        let h1 = top_signal1 + complex_mult(twiddle, bottom_signal1);

        unsafe {
            pingpong1.write(id.xy(), Vec4::new(h0.x, h0.y, h1.x, h1.y));
        }
    } else if data.pingpong == 1 {
        let top_signal0 = pingpong1.read(UVec2::new(id.x, indices.x)).xy();
        let top_signal1 = pingpong1.read(UVec2::new(id.x, indices.x)).zw();
        let bottom_signal0 = pingpong1.read(UVec2::new(id.x, indices.y)).xy();
        let bottom_signal1 = pingpong1.read(UVec2::new(id.x, indices.y)).zw();

        let h0 = top_signal0 + complex_mult(twiddle, bottom_signal0);
        let h1 = top_signal1 + complex_mult(twiddle, bottom_signal1);

        unsafe {
            pingpong0.write(id.xy(), Vec4::new(h0.x, h0.y, h1.x, h1.y));
        }
    }
}

// algorithm referenced from biebras: credit
#[spirv(compute(threads(8,8)))]
pub fn permute(
    #[spirv(global_invocation_id)] id: UVec3,
    #[spirv(descriptor_set = 1, binding = 0)] pingpong0: &StorageImage,
) {
    let sign = match ((id.x + id.y) % 2) as f32 {
        0.0 => 1.0,
        _ => -1.0,
    };

    let h0 = sign * pingpong0.read(id.xy()).x;
    let h1 = sign * pingpong0.read(id.xy()).z;
    unsafe {
        pingpong0.write(id.xy(), Vec4::new(h0, h1, 0.0, 1.0));
    }
}

// algorithm referenced from GPGPU TODO: credit
#[spirv(compute(threads(1,8)))]
pub fn precompute_butterfly(
    #[spirv(global_invocation_id)] id: UVec3,
    #[spirv(uniform, descriptor_set = 0, binding = 0)] consts: &Constants,
    #[spirv(descriptor_set = 1, binding = 1)] butterfly_tex: &StorageImage,
) {
    let k = (id.y as f32 * consts.sim.size as f32 / 2.0_f32.powf(id.x as f32 + 1.0)) % consts.sim.size as f32;
    // TODO TODO TDODO CHECK MINUS
    let exp = -2.0 * consts::PI * k / consts.sim.size as f32;
    let twiddle = Vec2::new(exp.cos(), exp.sin());

    let step = 2.0_f32.powf(id.x as f32);
    let wing = id.y as f32 % 2.0_f32.powf(id.x as f32 + 1.0) < step;

    let mut yt: u32 = id.y;
    let mut yb: u32 = id.y;

    if id.x == 0 {
        if wing {
            yb += 1;
        } else {
            yt -= 1;
        }
        yt = bit_reverse(yt, consts.sim.logsize);
        yb = bit_reverse(yb, consts.sim.logsize);
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
