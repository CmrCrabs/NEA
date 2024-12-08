use wgpu::{BindGroup, Buffer, Device, RenderPipeline, ShaderModule, TextureView};
use crate::scene::Vertex;
use std::mem;
use winit::window::Window;
use crate::renderer::{DEPTH_FORMAT, FORMAT};
use shared::SceneConstants;

pub struct StandardPipeline {
    pub pipeline: RenderPipeline,
    pub scene_bind_group: BindGroup,
    pub depth_view: TextureView,
}

impl StandardPipeline {
    pub fn new (device: &Device, window: &Window, shader: &ShaderModule, scene_buf: &Buffer) -> StandardPipeline {

        let scene_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
            label: None,
        });

        let scene_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &scene_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: scene_buf.as_entire_binding(),
            }],
            label: None,
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            bind_group_layouts: &[
                &scene_bind_group_layout
            ],
            push_constant_ranges: &[],
            label: None,
        });
        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: shader,
                entry_point: "main_vs",
                buffers: &[wgpu::VertexBufferLayout {
                    array_stride: mem::size_of::<Vertex>() as _,
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3],
                }],
            },
            fragment: Some(wgpu::FragmentState {
                module: shader,
                entry_point: "main_fs",
                targets: &[Some(wgpu::ColorTargetState {
                    format: FORMAT,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
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
        });

        let depth_view = StandardPipeline::new_depth_view(window, device);

        StandardPipeline {
            pipeline,
            scene_bind_group,
            depth_view,
        }
    }

    pub fn new_scene_buf(device: &Device) -> Buffer {
        device.create_buffer(&wgpu::BufferDescriptor {
            size: mem::size_of::<SceneConstants>() as u64,
            mapped_at_creation: false,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            label: None,
        })
    }

    pub fn new_depth_view(window: &Window, device: &Device) -> TextureView {
        let size = wgpu::Extent3d {
            width: window.inner_size().width,
            height: window.inner_size().height,
            depth_or_array_layers: 1,
        };

        let depth_texture = device.create_texture(&wgpu::TextureDescriptor {
            label: None,
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: DEPTH_FORMAT,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        });
        depth_texture.create_view(&wgpu::TextureViewDescriptor::default())
    }

    pub fn render<'a> (&'a self, encoder: &'a mut wgpu::CommandEncoder, surface_view: &'a TextureView) -> wgpu::RenderPass {
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
                view: &self.depth_view,
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
