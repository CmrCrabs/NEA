pub struct InitialSpectraPass {
    pipeline: wgpu::ComputePipeline,
    consts_buf: wgpu::Buffer,
    consts_bind_group: wgpu::BindGroup,
}

impl InitialSpectraPass {
    pub fn new(device: &wgpu::Device, shader: &wgpu::ShaderModule, ocean: &super::Ocean) -> Self {
        let consts_buf = device.create_buffer(&wgpu::BufferDescriptor {
            size: std::mem::size_of::<shared::SimConstants>() as u64,
            mapped_at_creation: false,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            label: None,
        });
        let consts_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None
            }],
            label: None,
        });
        let consts_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &consts_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: consts_buf.as_entire_binding(),
            }],
            label: None,
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            bind_group_layouts: &[
                &consts_layout,
                &ocean.spectrum_texture.layout,
                &ocean.wave_texture.layout,
            ],
            push_constant_ranges: &[],
            label: None,
        });
        let pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            entry_point: Some("initial_spectrum"),
            layout: Some(&pipeline_layout),
            module: shader,
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

    pub fn render<'a>(&'a self, encoder: &'a mut wgpu::CommandEncoder) {}
}
