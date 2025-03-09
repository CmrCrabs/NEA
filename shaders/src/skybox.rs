use spirv_std::glam::{Vec4,  Vec2, Vec3};
use spirv_std::{spirv,image::Image2d, Sampler};
use spirv_std::num_traits::Float;
use shared::Constants;
use crate::{equirectangular_to_uv, reinhard_tonemap, lerp};

#[inline(never)]
#[spirv(vertex)]
pub fn skybox_vs(
    #[spirv(vertex_index)] vertex_index: u32,
    #[spirv(position)] out_pos: &mut Vec4,
) {
    let out_uv1 = Vec2::new(
        ((vertex_index << 1) & 2) as f32,
        (vertex_index & 2) as f32,
    );
    *out_pos = Vec4::new(out_uv1.x * 2.0 - 1.0,out_uv1.y * 2.0 - 1.0, 0.0, 1.0);
}

#[inline(never)]
#[spirv(fragment)]
pub fn skybox_fs(
    #[spirv(frag_coord)] frag_coord: Vec4,  // Get screen-space fragment coordinates
    #[spirv(uniform, descriptor_set = 0, binding = 0)] consts: &Constants,
    #[spirv(descriptor_set = 1, binding = 0)] hdri: &Image2d,
    #[spirv(descriptor_set = 2, binding = 0)] sampler: &Sampler,
    out_color: &mut Vec4,
) {
    let proj_inverse = consts.shader.proj_mat.inverse();
    let view_inverse = consts.shader.view_mat.inverse();

    let uv = Vec2::new(
        frag_coord.x / consts.width * 2.0 - 1.0,
        1.0 - frag_coord.y / consts.height * 2.0,
    );
    
    let target = proj_inverse * Vec4::new(uv.x, uv.y, 1.0, 1.0);
    let view_pos = (target.truncate() / target.w).extend(1.0);
    let world_pos = view_inverse * view_pos;
    let ray_dir = world_pos.truncate().normalize();

    let h = (ray_dir.y / consts.eye.normalize().y).clamp(0.0, 1.0);
    let fog = (-h * consts.shader.fog_density - consts.shader.fog_height).exp();

    let sky_col = reinhard_tonemap(hdri.sample(
        *sampler, 
        equirectangular_to_uv(ray_dir),
    ).truncate()).extend(1.0);
    
    let sky_col = sky_col + dist_to_sun(ray_dir, &consts) * consts.shader.sun_color.truncate().extend(1.0);
    *out_color = lerp(
        sky_col,
        consts.shader.fog_color,
        fog,
    );
}

fn dist_to_sun(ray: Vec3, consts: &Constants) -> f32 {
    let dot = ray.dot(consts.shader.light.truncate().normalize());
    let cos = consts.shader.sun_size.cos();
    (dot - cos).max(0.0) * consts.shader.sun_falloff
}
