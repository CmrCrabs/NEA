#![no_std]

use core::f32;
use glam::Vec4;

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Constants {
    pub time: f32,
    pub deltatime: f32,
    pub width: f32,
    pub height: f32,
    pub camera_proj: glam::Mat4,
    pub eye: Vec4,
    pub shader: ShaderConstants,
    pub sim: SimConstants,
    //TODO: abstract
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct ShaderConstants {
    pub light: Vec4,
    pub sun_height: f32,
    pub sun_distance: f32,
    pub sun_angle: f32,
    pub foam_color: Vec4,
    pub water_ri: f32,
    pub air_ri: f32,
    pub roughness: f32,
    pub foam_roughness: f32,
    pub ss_height: f32,
    pub ss_reflected: f32,
    pub ss_lambert: f32,
    pub ss_ambient: f32,
    pub bubble_density: f32,
    pub bubble_color: Vec4,
    pub scatter_color: Vec4,
    pub sun_color: Vec4,
    pub shininess: f32,
    pub distance_factor: f32,
    pub pbr: u32,

}
impl Default for ShaderConstants {
    fn default() -> Self {
        Self {
            light: Vec4::new(0.0,0.0,0.0,1.0),
            sun_height: 25.0,
            sun_distance: 20.0,
            sun_angle: 0.0,
            foam_color: Vec4::new(0.79,0.92,0.96, 1.0),
            water_ri: 1.33,
            air_ri: 1.003,
            roughness: 0.05,
            foam_roughness: 0.1,
            ss_height: 0.76,
            ss_reflected: 1.0,
            ss_lambert: 1.0,
            ss_ambient: 0.9,
            bubble_density: 0.25,
            bubble_color: Vec4::new(0.0, 0.15, 0.15, 1.0),
            scatter_color: Vec4::new(0.04, 0.06, 0.14, 1.0),
            sun_color: Vec4::new(0.53, 0.45, 0.38, 1.0),
            shininess: 2.0,
            distance_factor: 1.0,
            pbr: 0,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct SimConstants {
    pub size: u32,
    pub lengthscale: u32,
    pub cutoff_low: f32,
    pub cutoff_high: f32,
    pub mesh_step: f32,
    pub standard_deviation: f32,
    pub mean: f32,
    pub depth: f32,
    pub gravity: f32,
    pub beta: f32,
    pub gamma: f32,
    pub wind_speed: f32,
    pub wind_offset: f32,
    pub fetch: f32,
    pub choppiness: f32,
    pub logsize: u32,
    pub swell: f32,
    pub integration_step: f32,
    pub foam_bias: f32,
    pub foam_decay: f32,
    pub injection_threshold: f32,
    pub injection_amount: f32,
}
impl Default for SimConstants {
    fn default() -> Self {
        let size = 512;
        Self {
            size,
            lengthscale: 50,
            cutoff_low: 0.00000001,
            cutoff_high: 10.0,
            mesh_step: 0.2 * 128.0 / size as f32,
            standard_deviation: 1.0,
            mean: 0.0,
            depth: 10.0,
            gravity: 9.81,
            beta: 5.0 / 4.0,
            gamma: 3.3,
            wind_speed: 40.0,
            wind_offset: f32::consts::FRAC_PI_4,
            fetch: 9500.0,
            choppiness: 0.6,
            logsize: 0,
            swell: 0.6,
            integration_step: 0.01,
            foam_bias: 0.71,
            foam_decay: 0.3,
            injection_threshold: 0.13,
            injection_amount: 0.5,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct FFTData {
    pub stage: u32,
    pub pingpong: u32,
}
