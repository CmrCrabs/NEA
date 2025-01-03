use spirv_std::{
    image::Image, spirv, Sampler,
    num_traits::Float,
};
use core::f32::consts::{self, PI};
use spirv_std::glam::{UVec3, Vec3Swizzles, Vec2, Vec4};
use shared::{Constants, SimConstants};

#[spirv(compute(threads(8,8)))]
pub fn main(
    #[spirv(global_invocation_id)] id: UVec3,
    #[spirv(uniform, descriptor_set = 0, binding = 0)] consts: &Constants,
    #[spirv(descriptor_set = 1, binding = 0)] gaussian_tex: &Image!(2D, format=rgba32f, sampled = false),
    #[spirv(descriptor_set = 2, binding = 0)] wave_tex: &Image!(2D, format = rgba32f, sampled = false),
    #[spirv(descriptor_set = 3, binding = 0)] spectrum_tex: &Image!(2D, format = rgba32f, sampled = false),
) {
    let dk: f32 = 2.0 * consts::PI / consts.sim.lengthscale as f32;
    let n = id.x as f32 - 0.5 *  consts.sim.size as f32;
    let m = id.y as f32 - 0.5 *  consts.sim.size as f32;
    let k: Vec2 = Vec2::new(n, m) * dk;
    let k_length = k.length();
    let theta = f32::atan(k.y / k.x);

    let omega = dispersion_relation(k_length, &consts.sim);
    let omega_d = dispersion_derivative(k_length, &consts.sim);
    let omega_p = 22.0 * ((consts.sim.gravity * consts.sim.gravity) / (consts.sim.wind_speed * consts.sim.fetch)).powf(1.0 / 3.0);

    let tma = jonswap(omega, omega_p, &consts.sim) * depth_attenuation(omega, &consts.sim);
    let spectrum = 2.0 * tma * donelan_banner(omega, omega_p, theta) * omega_d * (1.0 / k_length) * dk * dk;
    let h_0 = 1.0 / 2.0_f32.sqrt() * gaussian_tex.read(id.xy()) * spectrum.sqrt();

    unsafe {
        wave_tex.write(id.xy(), Vec4::new(k.x, k.y, omega, 1.0));
        wave_tex.write(id.xy(), Vec4::new(h_0.x, h_0.y, 0.0, 1.0));
    }
}

fn dispersion_relation(k: f32, consts: &SimConstants) -> f32 {
    (consts.gravity * k * (k * consts.depth).tanh()).sqrt()
}

fn dispersion_derivative(k: f32, consts: &SimConstants) -> f32 {
    let tanh = (consts.depth * k).tanh();
    let sech = 1.0 / (consts.depth * k).cosh();
    (consts.gravity * (tanh + consts.depth * k * sech * sech)) / (2.0 * (consts.gravity * k * tanh).sqrt())
}

fn jonswap(omega: f32,omega_p: f32, consts: &SimConstants) -> f32 {
    let sigma: f32 = match omega {
        _ if omega <= omega_p => 0.07,
        _ => 0.09,
    };
    let alpha = 0.076 * ((consts.wind_speed * consts.wind_speed) / (consts.fetch * consts.gravity)).powf(0.22);
    let r = (-1.0 * (omega - omega_p).powf(2.0) / (2.0 * omega_p * omega_p * sigma * sigma)).exp();
    alpha * consts.gravity * consts.gravity / (omega * omega * omega * omega * omega) * (-consts.beta * (omega_p / omega).powf(4.0)).exp() * consts.gamma.powf(r)
}

fn depth_attenuation(omega: f32, consts: &SimConstants) -> f32 {
    let omega_h = omega * (consts.depth / consts.gravity).sqrt();
    if omega_h <= 1.0 {
        0.5 * omega_h * omega_h 
    } else {
        1.0 - 0.5 * (2.0 - omega_h) * (2.0 - omega_h)
    }
} 

fn donelan_banner(omega: f32,omega_p: f32, theta: f32) -> f32 {
    let k = omega / omega_p; // arbitrary shorthand
    let beta_s: f32;
    if k < 0.95 {
        beta_s = 2.61 * k.powf(1.3);
    } else if k <= 1.6 && k >= 0.95 { 
        beta_s = 2.28 * k.powf(-1.3);
    } else {
        beta_s = 10.0_f32.powf(-0.4 + 0.8393 * (-0.567 * (k * k).ln()).exp())
    }
    beta_s / (2.0 * (beta_s * PI).tanh()) * (1.0 / (beta_s * theta).cosh()).powf(2.0)
}
