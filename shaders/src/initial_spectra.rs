use spirv_std::{
    spirv,
num_traits::Float,
};
use crate::StorageImage;
use core::f32::consts::{self, PI};
use spirv_std::glam::{UVec3, UVec2, Vec3Swizzles, Vec2, Vec4, Vec4Swizzles};
use shared::{Constants, SimConstants};

#[spirv(compute(threads(8,8)))]
pub fn main(
#[spirv(global_invocation_id)] id: UVec3,
#[spirv(uniform, descriptor_set = 0, binding = 0)] consts: &Constants,
#[spirv(descriptor_set = 1, binding = 0)] gaussian_tex: &StorageImage,
#[spirv(descriptor_set = 2, binding = 0)] wave_tex: &StorageImage,
#[spirv(descriptor_set = 2, binding = 1)] spectrum_tex: &StorageImage
) {
let dk: f32 = 2.0 * consts::PI / consts.sim.lengthscale as f32;
let n = id.x as f32 - 0.5 *  consts.sim.size as f32;
let m = id.y as f32 - 0.5 *  consts.sim.size as f32;
let k: Vec2 = Vec2::new(n, m) * dk;
let k_length = k.length();

if k_length <= 6.0 && k_length >= 0.000001 {
    let theta = k.y.atan2(k.x) - consts.sim.wind_offset;
    let omega = dispersion_relation(k_length, &consts.sim);
    let domega_dk = dispersion_derivative(k_length, &consts.sim); //Derivative
    let omega_peak = 22.0 * ((consts.sim.gravity * consts.sim.gravity) / (consts.sim.wind_speed * consts.sim.fetch)).powf(1.0 / 3.0);
    let jonswap = jonswap(omega, omega_peak, &consts.sim);
    let depth_attenuation = depth_attenuation(omega, &consts.sim);
    let tma = jonswap * depth_attenuation;
    let spectrum = 2.0 * tma * donelan_banner(omega, omega_peak, theta) * domega_dk.abs() * dk * dk / k_length;
    let h0 = 1.0 / 2.0_f32.sqrt() * gaussian_tex.read(id.xy()).xy() * spectrum.sqrt();
    
    unsafe {
        wave_tex.write(id.xy(), Vec4::new(k.x, k.y, 1.0 / k_length, omega));
        spectrum_tex.write(id.xy(), Vec4::new(h0.x, h0.y, 0.0, 1.0));
    }
} else {
    unsafe {
        wave_tex.write(id.xy(), Vec4::new(k.x, k.y, 0.0, 1.0));
        spectrum_tex.write(id.xy(), Vec4::ZERO);
        }
    }
}

fn dispersion_relation(k: f32, consts: &SimConstants) -> f32 {
    (consts.gravity * k * (k * consts.depth).min(20.0).tanh()).sqrt()
}

fn dispersion_derivative(k: f32, consts: &SimConstants) -> f32 {
    let tanh = (consts.depth * k).min(20.0).tanh();
    let sech = 1.0 / (consts.depth * k).cosh();
    (consts.gravity * (tanh + consts.depth * k * sech * sech)) / (2.0 * (consts.gravity * k * tanh).sqrt())
}

fn jonswap(omega: f32,omega_p: f32, consts: &SimConstants) -> f32 {
    let sigma: f32;
    if omega <= omega_p {
        sigma = 0.07;
    } else {
        sigma = 0.09;
    }
    let alpha = 0.076 * (
        (consts.wind_speed * consts.wind_speed) 
        / (consts.fetch * consts.gravity)
    ).powf(0.22);
    let r = (
        -1.0 * (omega - omega_p) * (omega - omega_p)
        / (2.0 * omega_p * omega_p * sigma * sigma)
    ).exp();
    alpha * consts.gravity * consts.gravity / 
    (omega * omega * omega * omega * omega) * (-consts.beta * (omega_p / omega).powf(4.0)).exp() * consts.gamma.powf(r)
}

fn depth_attenuation(omega: f32, consts: &SimConstants) -> f32 {
    let omega_h = omega * (consts.depth / consts.gravity).sqrt();
    if omega_h <= 1.0 {
        0.5 * omega_h * omega_h 
    } else if omega_h < 2.0 {
        1.0 - 0.5 * (2.0 - omega_h) * (2.0 - omega_h)
    } else {
        1.0
    }
} 

fn donelan_banner(omega: f32,omega_p: f32, theta: f32) -> f32 {
    let k = omega / omega_p; // arbitrary shorthand
    let beta_s: f32;
    if k < 0.95 {
        beta_s = 2.61 * k.abs().powf(1.3);
    } else if k <= 1.6 && k >= 0.95 { 
        beta_s = 2.28 * k.abs().powf(-1.3);
    } else {
        beta_s = 10.0_f32.powf(-0.4 + 0.8393 * (-0.567 * (k * k).ln()).exp())
    }
    let sech = 1.0 / (beta_s * theta).cosh();
    beta_s / (2.0 * (beta_s * PI).tanh()) * sech * sech
}

// TODO: explained in biebras -> explain
#[spirv(compute(threads(8,8)))]
pub fn pack_conjugates(
    #[spirv(global_invocation_id)] id: UVec3,
    #[spirv(uniform, descriptor_set = 0, binding = 0)] consts: &Constants,
    #[spirv(descriptor_set = 1, binding = 1)] spectrum_tex: &StorageImage,
) {
    let h0 = spectrum_tex.read(id.xy());
    let h0c = spectrum_tex.read(UVec2::new(
        (consts.sim.size - id.x) % consts.sim.size,
        (consts.sim.size - id.y) % consts.sim.size
    )).xy();
    unsafe {
        spectrum_tex.write(id.xy(), Vec4::new(h0.x, h0.y, h0c.x, -h0c.y));
    }
}
