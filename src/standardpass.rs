use crate::renderer::{Renderer, DEPTH_FORMAT, FORMAT};
use crate::scene::{OceanVertex, Scene};
use crate::sim::Cascade;
use std::mem;
use wgpu::{RenderPipeline, TextureView};

pub struct StandardPipeline {
    pub pipeline: RenderPipeline,
}

impl StandardPipeline {
    pub fn new(
        renderer: &Renderer,
        scene: &Scene,
        cascade: &Cascade,
    ) -> StandardPipeline {
        let pipeline_layout = renderer.device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            bind_group_layouts: &[&scene.consts_layout, &renderer.sampler_layout, &cascade.stg_layout],
            push_constant_ranges: &[],
            label: None,
        });
        let pipeline = renderer.device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &renderer.shader,
                entry_point: Some("main_vs"),
                buffers: &[wgpu::VertexBufferLayout {
                    array_stride: mem::size_of::<OceanVertex>() as _,
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &wgpu::vertex_attr_array![0 => Float32x4, 1=> Uint32x2, 2=> Float32x4],
                }],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &renderer.shader,
                entry_point: Some("main_fs"),
                targets: &[Some(wgpu::ColorTargetState {
                    format: FORMAT,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: Default::default(),
            }),
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: Some(wgpu::DepthStencilState {
                format: DEPTH_FORMAT,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Less,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }),
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
            label: None,
            cache: None,
        });

        StandardPipeline { pipeline }
    }

    pub fn render<'a>(
        &'a self,
        encoder: &'a mut wgpu::CommandEncoder,
        surface_view: &'a TextureView,
        depth_view: &'a TextureView,
    ) -> wgpu::RenderPass<'a> {
        encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: surface_view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: 0.0,
                        g: 0.0,
                        b: 0.0,
                        a: 1.0,
                    }),
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
        })
    }
}
