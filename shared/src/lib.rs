#![no_std]

use glam::Vec4;

#[repr(C)]
#[derive(Clone, Copy)]
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

#[derive(Clone, Copy)]
pub struct ShaderConstants {
    pub light: Vec4,
    pub base_color: Vec4,
}
impl Default for ShaderConstants {
    fn default() -> Self {
        Self {
            light: Vec4::new(0.0,1.0,1.0,1.0),
            base_color: Vec4::new(1.0,1.0,1.0,1.0),
        }
    }
}

#[derive(Clone, Copy)]
pub struct SimConstants {
    pub lengthscale: f32,
    pub mesh_step: f32,
}
impl Default for SimConstants {
    fn default() -> Self {
        Self {
            lengthscale: 4.0,
            mesh_step: 1.0,
        }
    }
}
