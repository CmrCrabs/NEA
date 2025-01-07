use crate::cast_slice;
use shared::Constants;
use std::mem;

pub struct ComputePass {
    pipeline: wgpu::ComputePipeline,
    consts_buf: wgpu::Buffer,
    consts_bind_group: wgpu::BindGroup,
}

impl ComputePass {
    pub fn new_initial_spectra(
        renderer: &crate::renderer::Renderer,
        cascade: &super::Cascade,
    ) -> Self {
        let mem_size = mem::size_of::<shared::Constants>()
            + mem::size_of::<shared::SimConstants>()
            + mem::size_of::<shared::ShaderConstants>();
        let consts_buf = renderer.device.create_buffer(&wgpu::BufferDescriptor {
            size: mem_size as u64,
            mapped_at_creation: false,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            label: Some("Consts Buffer"),
        });
        let consts_layout =
            renderer
                .device
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    entries: &[wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility:wgpu::ShaderStages::COMPUTE,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    }],
                    label: None,
                });
        let consts_bind_group = renderer
            .device
            .create_bind_group(&wgpu::BindGroupDescriptor {
                layout: &consts_layout,
                entries: &[wgpu::BindGroupEntry {
                    binding: 0,
                    resource: consts_buf.as_entire_binding(),
                }],
                label: Some("Consts Bind Group"),
            });

        let pipeline_layout =
            renderer
                .device
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    bind_group_layouts: &[
                        &consts_layout,
                        &cascade.gaussian_texture.layout,
                        &cascade.wave_texture.layout,
                        &cascade.spectrum_texture.layout,
                    ],
                    push_constant_ranges: &[],
                    label: Some("Initial Spectra"),
                });
        let pipeline = renderer
            .device
            .create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
                entry_point: Some("initial_spectra::main"),
                layout: Some(&pipeline_layout),
                module: &renderer.shader,
                compilation_options: Default::default(),
                cache: None,
                label: Some("Initial Spectra"),
            });

        Self {
            consts_buf,
            consts_bind_group,
            pipeline,
        }
    }

    pub fn compute_initial_spectra<'a>(
        &'a self,
        encoder: &'a mut wgpu::CommandEncoder,
        queue: &wgpu::Queue,
        consts: &Constants,
        cascade: &super::Cascade,
    ) {
        let mut pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
            timestamp_writes: None,
            label: Some("Initial Spectra"),
        });

        queue.write_buffer(&self.consts_buf, 0, cast_slice(&[*consts]));
        cascade
            .gaussian_texture
            .write(queue, cast_slice(&cascade.gaussian_noise.clone()), 16);
        pass.set_pipeline(&self.pipeline);
        pass.set_bind_group(0, &self.consts_bind_group, &[]);
        pass.set_bind_group(1, &cascade.gaussian_texture.bind_group, &[]);
        pass.set_bind_group(2, &cascade.wave_texture.bind_group, &[]);
        pass.set_bind_group(3, &cascade.spectrum_texture.bind_group, &[]);
        pass.dispatch_workgroups(consts.sim.size / 8, consts.sim.size / 8, 1);
        drop(pass);
    }

    pub fn new_conjugates(renderer: &crate::renderer::Renderer, cascade: &super::Cascade) -> Self {
        let mem_size = mem::size_of::<shared::Constants>()
            + mem::size_of::<shared::SimConstants>()
            + mem::size_of::<shared::ShaderConstants>();
        let consts_buf = renderer.device.create_buffer(&wgpu::BufferDescriptor {
            size: mem_size as u64,
            mapped_at_creation: false,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            label: None,
        });
        let consts_layout =
            renderer
                .device
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    entries: &[wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::COMPUTE,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    }],
                    label: None,
                });
        let consts_bind_group = renderer
            .device
            .create_bind_group(&wgpu::BindGroupDescriptor {
                layout: &consts_layout,
                entries: &[wgpu::BindGroupEntry {
                    binding: 0,
                    resource: consts_buf.as_entire_binding(),
                }],
                label: None,
            });

        let pipeline_layout =
            renderer
                .device
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    bind_group_layouts: &[&consts_layout, &cascade.spectrum_texture.layout],
                    push_constant_ranges: &[],
                    label: None,
                });
        let pipeline = renderer
            .device
            .create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
                entry_point: Some("initial_spectra::pack_conjugates"),
                layout: Some(&pipeline_layout),
                module: &renderer.shader,
                compilation_options: Default::default(),
                cache: None,
                label: None,
            });

        Self {
            consts_buf,
            consts_bind_group,
            pipeline,
        }
    }

    pub fn pack_conjugates<'a>(
        &'a self,
        encoder: &'a mut wgpu::CommandEncoder,
        queue: &wgpu::Queue,
        consts: &Constants,
        cascade: &super::Cascade,
    ) {
        let mut pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
            timestamp_writes: None,
            label: Some("Pack Conjugates"),
        });
        queue.write_buffer(&self.consts_buf, 0, cast_slice(&[*consts]));
        pass.set_pipeline(&self.pipeline);
        pass.set_bind_group(0, &self.consts_bind_group, &[]);
        pass.set_bind_group(1, &cascade.spectrum_texture.bind_group, &[]);
        pass.dispatch_workgroups(consts.sim.size / 8, consts.sim.size / 8, 1);
        drop(pass);
    }

    pub fn new_evolve_spectra(
        renderer: &crate::renderer::Renderer,
        cascade: &super::Cascade,
    ) -> Self {
        let mem_size = mem::size_of::<shared::Constants>()
            + mem::size_of::<shared::SimConstants>()
            + mem::size_of::<shared::ShaderConstants>();
        let consts_buf = renderer.device.create_buffer(&wgpu::BufferDescriptor {
            size: mem_size as u64,
            mapped_at_creation: false,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            label: Some("Evolve Spectra"),
        });
        let consts_layout =
            renderer
                .device
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    entries: &[wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::COMPUTE,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    }],
                    label: None,
                });
        let consts_bind_group = renderer
            .device
            .create_bind_group(&wgpu::BindGroupDescriptor {
                layout: &consts_layout,
                entries: &[wgpu::BindGroupEntry {
                    binding: 0,
                    resource: consts_buf.as_entire_binding(),
                }],
                label: None,
            });

        let pipeline_layout =
            renderer
                .device
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    bind_group_layouts: &[
                        &consts_layout,
                        &cascade.wave_texture.layout,
                        &cascade.spectrum_texture.layout,
                        &cascade.height_map.layout,
                        &cascade.tangent_map.layout,
                    ],
                    push_constant_ranges: &[],
                    label: None,
                });
        let pipeline = renderer
            .device
            .create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
                entry_point: Some("evolve_spectra::main"),
                layout: Some(&pipeline_layout),
                module: &renderer.shader,
                compilation_options: Default::default(),
                cache: None,
                label: None,
            });

        Self {
            consts_buf,
            consts_bind_group,
            pipeline,
        }
    }

    pub fn evolve_spectra<'a>(
        &'a self,
        encoder: &'a mut wgpu::CommandEncoder,
        queue: &wgpu::Queue,
        consts: &Constants,
        cascade: &super::Cascade,
    ) {
        let mut pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
            timestamp_writes: None,
            label: None,
        });

        queue.write_buffer(&self.consts_buf, 0, cast_slice(&[*consts]));
        pass.set_pipeline(&self.pipeline);
        pass.set_bind_group(0, &self.consts_bind_group, &[]);
        pass.set_bind_group(1, &cascade.wave_texture.bind_group, &[]);
        pass.set_bind_group(2, &cascade.spectrum_texture.bind_group, &[]);
        pass.set_bind_group(3, &cascade.height_map.bind_group, &[]);
        pass.set_bind_group(4, &cascade.tangent_map.bind_group, &[]);
        pass.dispatch_workgroups(consts.sim.size / 8, consts.sim.size / 8, 1);
        drop(pass);
    }

    pub fn new_fourier(renderer: &crate::renderer::Renderer, cascade: &super::Cascade) -> Self {
        let mem_size = mem::size_of::<shared::Constants>()
            + mem::size_of::<shared::SimConstants>()
            + mem::size_of::<shared::ShaderConstants>();
        let consts_buf = renderer.device.create_buffer(&wgpu::BufferDescriptor {
            size: mem_size as u64,
            mapped_at_creation: false,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            label: None,
        });
        let consts_layout =
            renderer
                .device
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    entries: &[wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::COMPUTE,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    }],
                    label: None,
                });
        let consts_bind_group = renderer
            .device
            .create_bind_group(&wgpu::BindGroupDescriptor {
                layout: &consts_layout,
                entries: &[wgpu::BindGroupEntry {
                    binding: 0,
                    resource: consts_buf.as_entire_binding(),
                }],
                label: None,
            });

        let pipeline_layout =
            renderer
                .device
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    bind_group_layouts: &[
                        &consts_layout,
                        &cascade.wave_texture.layout,
                        &cascade.height_map.layout,
                        &cascade.tangent_map.layout,
                    ],
                    push_constant_ranges: &[],
                    label: None,
                });
        let pipeline = renderer
            .device
            .create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
                entry_point: Some("fourier_transform::main"),
                layout: Some(&pipeline_layout),
                module: &renderer.shader,
                compilation_options: Default::default(),
                cache: None,
                label: None,
            });

        Self {
            consts_buf,
            consts_bind_group,
            pipeline,
        }
    }

    pub fn transform<'a>(
        &'a self,
        encoder: &'a mut wgpu::CommandEncoder,
        queue: &wgpu::Queue,
        consts: &Constants,
        cascade: &super::Cascade,
    ) {
        let mut pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
            timestamp_writes: None,
            label: Some("Fourier Transform"),
        });

        queue.write_buffer(&self.consts_buf, 0, cast_slice(&[*consts]));
        pass.set_pipeline(&self.pipeline);
        pass.set_bind_group(0, &self.consts_bind_group, &[]);
        pass.set_bind_group(1, &cascade.wave_texture.bind_group, &[]);
        pass.set_bind_group(2, &cascade.height_map.bind_group, &[]);
        pass.set_bind_group(3, &cascade.tangent_map.bind_group, &[]);
        pass.dispatch_workgroups(consts.sim.size / 8, consts.sim.size / 8, 1);
        drop(pass);
    }
}
