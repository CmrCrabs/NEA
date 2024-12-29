#![no_std]

use glam::Vec3;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Constants {
    pub time: f32,
    pub frametime: f32,
    pub width: f32,
    pub height: f32,
    pub camera_proj: glam::Mat4,
    //pub view: Vec3,
    pub shader: ShaderConstants,
    pub sim: SimConstants,
}

#[derive(Clone, Copy)]
pub struct ShaderConstants {
    pub light: Vec3,
    pub base_color: Vec3,
}
impl Default for ShaderConstants {
    fn default() -> Self {
        Self {
            light: Vec3::new(0.0,1.0,1.0),
            base_color: Vec3::new(1.0,1.0,1.0),
        }
    }
}

#[derive(Clone, Copy)]
pub struct SimConstants {
    pub lengthscale: f32,
}
impl Default for SimConstants {
    fn default() -> Self {
        Self {
            lengthscale: 64.0,
        }
    }
}
