#![no_std]

use core::f32;

use glam::{Vec2, Vec4};

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Constants {
    pub time: f32,
    pub deltatime: f32,
    pub width: f32,
    pub height: f32,
    pub camera_proj: glam::Mat4,
    pub view: Vec4,
    pub shader: ShaderConstants,
    pub sim: SimConstants,
    //TODO: abstract
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct ShaderConstants {
    pub light: Vec4,
    pub base_color: Vec4,
    pub light_rotation: f32,
}
impl Default for ShaderConstants {
    fn default() -> Self {
        Self {
            light: Vec4::new(0.0,1.0,1.0,1.0),
            light_rotation: 0.0,
            base_color: Vec4::new(0.0,0.1,0.3,1.0),
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
    pub foam_decay: f32,
    pub foam_bias: f32,
    pub foam_rate: f32,
}
impl Default for SimConstants {
    fn default() -> Self {
        let size = 256;
        Self {
            size,
            lengthscale: 50,
            cutoff_low: 0.00000001,
            cutoff_high: 6.0,
            mesh_step: 0.1 * 128.0 / size as f32,
            standard_deviation: 1.0,
            mean: 0.0,
            depth: 10.0,
            gravity: 9.81,
            beta: 5.0 / 4.0,
            gamma: 3.3,
            wind_speed: 10.0,
            wind_offset: f32::consts::FRAC_PI_4,
            fetch: 8000.0,
            choppiness: 0.5,
            logsize: 0,
            swell: 0.6,
            integration_step: 0.01,
            foam_decay: 0.055,
            foam_bias: 0.81,
            foam_rate: 1.0,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct FFTData {
    pub stage: u32,
    pub pingpong: u32,
}
