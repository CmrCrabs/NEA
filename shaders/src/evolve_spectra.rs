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
    #[spirv(descriptor_set = 2, binding = 0)] spectrum_tex: &StorageImage,
    #[spirv(descriptor_set = 3, binding = 0)] height_map: &StorageImage,
    #[spirv(descriptor_set = 4, binding = 0)] tangent_map: &StorageImage,
) {
    // Evolving spectra, calculating amplitudes
    let wave = wave_tex.read(id.xy());
    let spectrum = spectrum_tex.read(id.xy());
    let h0 = spectrum.xy();
    let h0c = spectrum.zw();
    let phase = wave.w * consts.time;
    let exponent = Vec2::new(phase.cos(), phase.sin());
    let negative_exponent = Vec2::new(exponent.x, -exponent.y);

    let h = complex_mult(h0, exponent) + complex_mult(h0c, negative_exponent);
    let ih = Vec2::new(-h.y, h.x);
    let x_d = -ih * wave.x * wave.z;
    let z_d = -ih * wave.y * wave.z;

    unsafe {
        height_map.write(id.xy(), Vec4::new(h.x, h.y, ih.x, ih.y));
        tangent_map.write(id.xy(), Vec4::new(x_d.x, x_d.y, z_d.x, z_d.y));
    }
}

pub fn complex_mult(a: Vec2, b: Vec2) -> Vec2 {
    Vec2::new(a.x * b.x - a.y * b.y, a.x * b.y + a.y * b.x)
}
