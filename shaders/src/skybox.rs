use spirv_std::glam::{Vec4,  Vec2};
use spirv_std::{spirv,image::Image2d, Sampler};
use shared::Constants;
use spirv_std::num_traits::Float;
use crate::{equirectangular_to_uv, reinhard_tonemap};

#[inline(never)]
#[spirv(vertex)]
pub fn skybox_vs(
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

#[inline(never)]
#[spirv(fragment)]
pub fn skybox_fs(
    uv: Vec2,
    #[spirv(uniform, descriptor_set = 0, binding = 0)] consts: &Constants,
    #[spirv(descriptor_set = 1, binding = 0)] hdri: &Image2d,
    #[spirv(descriptor_set = 2, binding = 0)] sampler: &Sampler,
    out_color: &mut Vec4,
) {
    let proj_inverse = consts.shader.proj_mat.inverse();
    let view_inverse = consts.shader.view_mat.inverse();
    let target = proj_inverse * Vec4::new(uv.x, uv.y, 1.0, 1.0);
    let target_transform = (target.truncate() / target.w).normalize().extend(0.0);
    let ray_dir = view_inverse * target_transform;
    

    let sky_col = reinhard_tonemap(hdri.sample(
        *sampler, 
        equirectangular_to_uv(ray_dir.truncate()),
    ).truncate()).extend(1.0);
    
    if ray_dir.dot(consts.shader.light.normalize()) > consts.shader.sun_size.cos() {
        *out_color = (consts.shader.sun_color * 10.0).truncate().extend(1.0);      
    } else {
        *out_color = sky_col;
    }
}
