use wgpu::{RenderPipeline, Buffer};

pub struct UIPass {
    pub pipeline: RenderPipeline,
    pub vtx_buf: Buffer,
    pub idx_buf: Buffer,
}
