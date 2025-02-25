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
    #[spirv(descriptor_set = 1, binding = 0)] wave_tex: &StorageImage,
    #[spirv(descriptor_set = 1, binding = 1)] initial_spectrum_tex: &StorageImage,
    #[spirv(descriptor_set = 2, binding = 0)] h_displacement: &StorageImage,
    #[spirv(descriptor_set = 3, binding = 0)] h_slope: &StorageImage,
) {
    // Evolving spectra
    let wave = wave_tex.read(id.xy());
    let spectrum = initial_spectrum_tex.read(id.xy());
    let h0 = spectrum.xy();
    let h0c = spectrum.zw();
    let phase = wave.w * consts.time;
    let exponent = euler(phase);
    let negative_exponent = Vec2::new(exponent.x, -exponent.y);

    // Precalculating Amplitudes
    let h = complex_mult(h0, exponent) + complex_mult(h0c, negative_exponent);
    let ih = Vec2::new(-h.y, h.x);

    let dx = -ih * wave.x * wave.z;
    let dy = h;
    let dz = -ih * wave.y * wave.z;

    unsafe {
        h_displacement.write(id.xy(), Vec4::new(dy.x, dy.y, dx.x, dx.y));
        h_slope.write(id.xy(), Vec4::new(dz.x, dz.y, 0.0, 0.0));
    }
}

pub fn complex_mult(a: Vec2, b: Vec2) -> Vec2 {
    Vec2::new(a.x * b.x - a.y * b.y, a.x * b.y + a.y * b.x)
}

pub fn euler(exp: f32) -> Vec2 {
    Vec2::new(exp.cos(), exp.sin())
}

pub fn complex_exp(a: Vec2) -> Vec2 {
    Vec2::new(a.y.cos(), a.y.sin()) * a.x.exp()
}
