#![no_std]

#[repr(C)]
#[derive(Clone, Copy)]
pub struct SceneConstants {
    pub time: f32,
    pub frametime: f32,

    pub width: f32,
    pub height: f32,

    pub camera_proj: glam::Mat4
}
