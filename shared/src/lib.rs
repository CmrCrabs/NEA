#![no_std]

use core::f32;

use glam::Vec4;

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Constants {
    pub time: f32,
    pub frametime: f32,
    pub width: f32,
    pub height: f32,
    pub camera_proj: glam::Mat4,
    pub view: Vec4,
    pub shader: ShaderConstants,
    pub sim: SimConstants,
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
}
impl Default for SimConstants {
    fn default() -> Self {
        let size = 128;
        Self {
            size,
            lengthscale: 120,
            mesh_step: 0.1,
            standard_deviation: 1.0,
            mean: 0.0,
            depth: 500.0,
            gravity: 9.81,
            beta: 5.0 / 4.0,
            gamma: 3.3,
            wind_speed: 30.0,
            wind_offset: f32::consts::FRAC_PI_4,
            fetch: 8000.0,
            choppiness: 0.2,
            logsize: 0,
        }
    }
}
