use super::util::Texture;
use crate::{DEPTH_FORMAT, FORMAT};
use super::scene::{Mesh, Scene};
use super::Simulation;

pub struct Renderer {
    pub sampler_bind_group: wgpu::BindGroup,
    pub sampler_layout: wgpu::BindGroupLayout,
    pub depth_view: wgpu::TextureView,
    pub std_pipeline: wgpu::RenderPipeline,
    pub hdri: Texture,
    pub skybox_pipeline: wgpu::RenderPipeline,
}

impl Renderer {
    pub fn new(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        shader: &wgpu::ShaderModule,
        window: &winit::window::Window,
        sim: &Simulation,
        scene: &super::scene::Scene,
    ) -> Self {
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor::default());
        let sampler_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::NonFiltering),
                count: None,
            }],
            label: None,
        });
        let sampler_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &sampler_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::Sampler(&sampler),
            }],
            label: None,
        });

        //let hdri = Texture::from_file(&device, &queue, "HDRI", "../assets/kloppenheim.exr");
        //let hdri = Texture::from_file(&device, &queue, "HDRI", "../assets/belfast_sunset.exr");
        let hdri = Texture::from_file(&device, &queue, "HDRI", "./assets/kloofendal.exr");

        let depth_texture = device.create_texture(&wgpu::TextureDescriptor {
            label: None,
            size: wgpu::Extent3d {
                width: window.inner_size().width,
                height: window.inner_size().height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: DEPTH_FORMAT,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        });
        let depth_view = depth_texture.create_view(&wgpu::TextureViewDescriptor::default());

        let skybox_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                bind_group_layouts: &[&scene.consts_layout, &hdri.layout, &sampler_layout],
                push_constant_ranges: &[],
                label: None,
            });
        let skybox_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            layout: Some(&skybox_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("skybox::skybox_vs"),
                buffers: &[],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("skybox::skybox_fs"),
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

        let std_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            bind_group_layouts: &[
                &scene.consts_layout,
                &sampler_layout,
                &hdri.layout,
                &sim.cascade0.layout,
                &sim.cascade1.layout,
                &sim.cascade2.layout,
            ],
            push_constant_ranges: &[],
            label: None,
        });
        let std_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            layout: Some(&std_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("main_vs"),
                buffers: &[wgpu::VertexBufferLayout {
                    array_stride: std::mem::size_of::<super::scene::Vertex>() as _,
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &wgpu::vertex_attr_array![0 => Float32x4, 1=> Uint32x2],
                }],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
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

        Self {
            sampler_layout,
            sampler_bind_group,
            depth_view,
            std_pipeline,
            hdri,
            skybox_pipeline,
        }
    }

    pub fn new_depth_view(&mut self, device: &wgpu::Device, window: &winit::window::Window) {
        let depth_texture = device.create_texture(&wgpu::TextureDescriptor {
            label: None,
            size: wgpu::Extent3d {
                width: window.inner_size().width,
                height: window.inner_size().height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: DEPTH_FORMAT,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        });
        self.depth_view = depth_texture.create_view(&wgpu::TextureViewDescriptor::default())
    }

    pub fn render_skybox<'a>(
        &'a self,
        encoder: &'a mut wgpu::CommandEncoder,
        surface_view: &wgpu::TextureView,
        scene: &Scene,
    ) {
        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: surface_view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
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
        });
        pass.set_pipeline(&self.skybox_pipeline);
        pass.set_bind_group(0, &scene.consts_bind_group, &[]);
        pass.set_bind_group(1, &self.hdri.bind_group, &[]);
        pass.set_bind_group(2, &self.sampler_bind_group, &[]);
        pass.draw(0..3, 0..1);
    }

    pub fn render_standard<'a>(
        &'a self,
        encoder: &'a mut wgpu::CommandEncoder,
        pipeline: &wgpu::RenderPipeline,
        bind_groups: &[&wgpu::BindGroup],
        load_op: wgpu::LoadOp<wgpu::Color>,
        surface_view: &wgpu::TextureView,
        mesh: &Mesh,
        instances: u32,
    ) {
        // TODO: move to render
        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: surface_view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: load_op,
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
        });
        pass.set_pipeline(&pipeline);
        for i in 0..bind_groups.len() {
            pass.set_bind_group(i as _, bind_groups[i], &[]);
        }
        pass.set_vertex_buffer(0, mesh.vtx_buf.slice(..));
        pass.set_index_buffer(mesh.idx_buf.slice(..), wgpu::IndexFormat::Uint32);
        pass.draw_indexed(0..(mesh.length as _), 0, 0..(instances * instances));
    }
}
