#![no_std]

use spirv_std::glam::{Vec3, Vec4,Vec2,Vec4Swizzles, Mat4};
use spirv_std::spirv;
use spirv_std::Sampler;
use spirv_std::image::Image2d;
use spirv_std::num_traits::Float;
use shared::SceneConstants;

// Render Pass 1
#[spirv(vertex)]
pub fn main_vs(
    pos: Vec3,
    tex_coords: Vec2,
    #[spirv(uniform, descriptor_set = 0, binding = 0)] camera_view_proj: &Mat4,
    #[spirv(position)] out_pos: &mut Vec4,
    out_tex_coord: &mut Vec2,
) {
    *out_pos = *camera_view_proj * pos.extend(1.0);
    *out_tex_coord = tex_coords;
}

#[spirv(fragment)]
pub fn main_fs(
    in_tex: Vec2,
    #[spirv(descriptor_set = 1, binding = 0)] texture: &Image2d,
    #[spirv(descriptor_set = 1, binding = 1)] sampler: &Sampler,
    output: &mut Vec4,
) {    
    *output = texture.sample(*sampler, in_tex);
}

// Render Pass 2
#[spirv(vertex)]
pub fn fullscreen_quad_gen_vs(
    #[spirv(vertex_index)] vertex_index: u32,
    #[spirv(position)] out_pos: &mut Vec4,
    out_uv: &mut Vec2,
) {
    let out_uv1 = Vec2::new(
        ((vertex_index << 1) & 2) as f32,
        (vertex_index & 2) as f32,
    );
    *out_pos = Vec4::new(out_uv1.x * 2.0 - 1.0,out_uv1.y * 2.0 - 1.0, 0.0, 1.0);
    *out_uv = out_uv1;
}

#[spirv(fragment)]
pub fn quad_copy_fs(
    in_uv: Vec2,
    #[spirv(descriptor_set = 0, binding = 0)] framebuf: &Image2d,
    #[spirv(descriptor_set = 0, binding = 1)] sampler: &Sampler,
    #[spirv(uniform, descriptor_set = 1, binding = 0)] scene_consts: &SceneConstants,
    #[spirv(frag_coord)] in_frag_coord: Vec4,
    output: &mut Vec4,
) {    
    let uv = Vec2::new(in_uv.x, 1.0 - in_uv.y);
    let step_size = 2.0;
    let step = Vec2::new(step_size / scene_consts.width, step_size / scene_consts.height);
    let tleft = intensity(framebuf.sample(*sampler, uv + Vec2::new(-step.x, step.y)));
    let left = intensity(framebuf.sample(*sampler, uv + Vec2::new(-step.x, 0.0)));
    let bleft = intensity(framebuf.sample(*sampler, uv + Vec2::new(-step.x, -step.y)));
    let tright = intensity(framebuf.sample(*sampler, uv + Vec2::new(step.x, step.y)));
    let right = intensity(framebuf.sample(*sampler, uv + Vec2::new(step.x, 0.0)));
    let bright = intensity(framebuf.sample(*sampler, uv + Vec2::new(step.x, -step.y)));
    let top = intensity(framebuf.sample(*sampler, uv + Vec2::new(0.0, step.y)));
    let bottom = intensity(framebuf.sample(*sampler, uv + Vec2::new(0.0, -step.y)));

    let g_x = tleft + 2.0 * left + bleft - tright - 2.0 * right - bright;
    let g_y = -tleft - 2.0 * top - tright + bleft + 2.0 * bottom + bright;
    let c = (g_x * g_x + g_y * g_y).sqrt();  

    let edge_col = Vec4::new(0.09,0.12,0.18, 1.0);
    let base_col = framebuf.sample(*sampler, uv);
    let threshold = 0.5;
    if c > threshold {
        *output = edge_col;
    } else {
        *output = base_col;
    }
}

fn intensity(col: Vec4) -> f32 {
    (col.x * col.x + col.y * col.y + col.z * col.z).sqrt()
}
