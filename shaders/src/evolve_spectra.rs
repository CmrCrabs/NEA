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
    #[spirv(descriptor_set = 2, binding = 0)] dx_dz: &StorageImage,
    #[spirv(descriptor_set = 3, binding = 0)] dy_dxz: &StorageImage,
    #[spirv(descriptor_set = 4, binding = 0)] dyx_dyz: &StorageImage,
    #[spirv(descriptor_set = 5, binding = 0)] dxx_dzz: &StorageImage,
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

    let dx = ih * wave.x * wave.z;
    let dy = h;
    let dz = ih * wave.y * wave.z;

    let dx_dx = -h * wave.x * wave.x * wave.z;
    let dy_dx = ih * wave.x;
    let dz_dx = -h * wave.y * wave.y * wave.z;

    let dy_dz = ih * wave.y;
    let dz_dz = -h * wave.y * wave.y * wave.z;

    unsafe {
        // portially adapted from gasgiant TODO: proper credit
        // TODO: optimise into 2
        dx_dz.write(id.xy(), Vec4::new(dx.x - dz.y, dx.y + dz.x, 0.0, 1.0));
        dy_dxz.write(id.xy(), Vec4::new(dy.x - dz_dx.y, dy.y + dz_dx.x, 0.0, 1.0));
        dyx_dyz.write(id.xy(), Vec4::new(dy_dx.x - dy_dz.y, dy_dx.y + dy_dz.x, 0.0, 1.0));
        dxx_dzz.write(id.xy(), Vec4::new(dx_dx.x - dz_dz.y, dx_dx.y + dz_dz.x, 0.0, 1.0));
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
