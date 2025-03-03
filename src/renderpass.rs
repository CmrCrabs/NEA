pub struct RenderPass {
    pipeline: wgpu::RenderPipeline,
}

impl RenderPass {
    pub fn new(
        renderer: &super::Renderer,
        bind_group_layouts: &[&wgpu::BindGroupLayout],
        label: &str,
        vs_entry_point: &str,
        fs_entry_point: &str,
        buffers: wgpu::VertexBufferLayout,
        format: wgpu::TextureFormat,
        depth_format: wgpu::TextureFormat,
    ) -> Self {
        let pipeline_layout = renderer.device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            bind_group_layouts,
            push_constant_ranges: &[],
            label: Some(label),
        });
        let pipeline = renderer.device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &renderer.shader,
                entry_point: Some(vs_entry_point),
                buffers: &[buffers],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &renderer.shader,
                entry_point: Some(fs_entry_point),
                targets: &[Some(wgpu::ColorTargetState {
                    format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: Default::default(),
            }),
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: Some(wgpu::DepthStencilState {
                format: depth_format,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Less,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }),
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
            label: Some(label),
            cache: None,
        });

        Self { pipeline }
    }

    pub fn render<'a>(
        &'a self,
        encoder: &'a mut wgpu::CommandEncoder,
        label: &str,
        bind_groups: &[&wgpu::BindGroup],
        surface_view: &wgpu::TextureView,
        depth_view: &wgpu::TextureView,
        load: wgpu::LoadOp<wgpu::Color>,
        vtx_buf: &wgpu::Buffer,
        idx_buf: &wgpu::Buffer,
        length: u32,
    ) {
        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: surface_view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load,
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                view: depth_view,
                depth_ops: Some(wgpu::Operations {
                    load: wgpu::LoadOp::Clear(1.0),
                    store: wgpu::StoreOp::Store,
                }),
                stencil_ops: None,
            }),
            timestamp_writes: None,
            occlusion_query_set: None,
            label: None,
        });
        pass.set_pipeline(&self.pipeline);

        for i in 0..bind_groups.len() {
            pass.set_bind_group(i as u32, bind_groups[i], &[]);
        }
        pass.set_vertex_buffer(0, vtx_buf.slice(..));
        pass.set_index_buffer(
            idx_buf.slice(..),
            wgpu::IndexFormat::Uint32,
        );
        pass.draw_indexed(0..(length as _), 0, 0..1);
    }
}
